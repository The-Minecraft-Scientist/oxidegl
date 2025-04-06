use std::mem;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use log::{info, trace};
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use objc2_foundation::{NSString, ns_string};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLColorWriteMask, MTLCommandBuffer, MTLCommandBufferDescriptor,
    MTLCommandBufferErrorOption, MTLCommandEncoder, MTLCommandQueue, MTLCompareFunction,
    MTLCreateSystemDefaultDevice, MTLCullMode, MTLDepthStencilDescriptor, MTLDevice,
    MTLPixelFormat, MTLRenderCommandEncoder, MTLRenderPassColorAttachmentDescriptor,
    MTLRenderPassDepthAttachmentDescriptor, MTLRenderPassDescriptor,
    MTLRenderPassStencilAttachmentDescriptor, MTLRenderPipelineColorAttachmentDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLStencilDescriptor, MTLStencilOperation,
    MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureUsage,
    MTLVertexAttributeDescriptor, MTLVertexBufferLayoutDescriptor, MTLVertexDescriptor,
    MTLViewport,
};
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer, kCAFilterNearest};

use crate::{
    context::{
        debug::{gl_debug, gl_trace},
        state::{Capabilities, StencilFaceState},
    },
    device_properties::MetalProperties,
    enums::{DepthFunction, DrawBufferMode, ShaderType, StencilFunction, StencilOp, TriangleFace},
    util::{ProtoObjRef, bitflag_bits},
};

use super::{
    Context,
    commands::buffer::Buffer,
    framebuffer::InternalDrawable,
    gl_object::{NamedObject, ObjectName},
    program::LinkedStage,
    state::{ColorWriteMask, DrawbufferBlendState, GLState},
};

#[derive(Debug)]
pub struct PlatformState {
    /// dirty components
    pub(crate) dirty_state: Dirty,

    /// the `NSView` this context is associated with
    view: Option<Retained<NSView>>,

    /// Metal device
    pub(crate) device: ProtoObjRef<dyn MTLDevice>,

    /// Device and Metal version properties
    pub(crate) props: MetalProperties,

    /// Metal command queue
    pub(crate) queue: ProtoObjRef<dyn MTLCommandQueue>,

    /// Metal layer for rendering
    pub(crate) layer: Retained<CAMetalLayer>,

    /// Current drawable
    // TODO: see if it's really necessary to have this as a field as opposed to generating it on-the-fly
    pub(crate) drawable: Option<ProtoObjRef<dyn CAMetalDrawable>>,

    /// Command buffer for this frame's rendering/compute commands
    pub(crate) command_buffer: Option<ProtoObjRef<dyn MTLCommandBuffer>>,

    /// Command buffer used to realize buffer to buffer copies
    /// This is separate from the primary command buffer because
    /// we need to realize private to shared/shared to private buffer copies which
    /// happen before the user calls mapBuffer, which can happen
    /// at any time, and we don't want to submit partial render commands by accident
    pub(crate) blit_command_buffer: Option<ProtoObjRef<dyn MTLCommandBuffer>>,

    /// Current encoder for blit commands. We only really need one encoder instance, so
    /// the procedure is to commit this encoder and immediately submit the `blit_command_buffer`
    /// upon a buffer copy flush
    pub(crate) blit_encoder: Option<ProtoObjRef<dyn MTLBlitCommandEncoder>>,

    /// Current encoder for render commands
    pub(crate) render_encoder: Option<ProtoObjRef<dyn MTLRenderCommandEncoder>>,

    /// Current render pipeline state
    pub(crate) render_pipeline_state: Option<ProtoObjRef<dyn MTLRenderPipelineState>>,

    /// Mapping from buffer name to metal vertex shader argument index
    pub(crate) vertex_buffer_map: ResourceMap<Buffer, MTL_MAX_ARGUMENT_BINDINGS>,

    /// Mapping from metal vertex argument table index to vertex descriptor buffer offset
    pub(crate) vertex_buffer_offsets: HashMap<ObjectName<Buffer>, usize>,

    /// Mapping from buffer name to metal fragment shader argument index
    pub(crate) fragment_buffer_map: ResourceMap<Buffer, MTL_MAX_ARGUMENT_BINDINGS>,

    /// List of internal drawables that back the various bitplanes in the default framebuffer
    pub(crate) internal_drawables: InternalDrawables,

    /// Pixel format of the default framebuffer
    pub(crate) pixel_format: MTLPixelFormat,

    /// Depth buffer format of the default framebuffer
    pub(crate) depth_format: Option<MTLPixelFormat>,

    /// Stencil buffer format of the default framebuffer
    pub(crate) stencil_format: Option<MTLPixelFormat>,
}
#[derive(Default, Debug, Clone)]
pub struct InternalDrawables {
    front_left: Option<InternalDrawable>,
    front_right: Option<InternalDrawable>,
    back_left: Option<InternalDrawable>,
    back_right: Option<InternalDrawable>,
    depth: Option<InternalDrawable>,
    stencil: Option<InternalDrawable>,
}
bitflag_bits! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub(crate) struct Dirty: u32 bits: {
        /// Create a fresh render encoder from the current GL state
        NEW_RENDER_ENCODER: 0,
        /// Update dynamic render encoder state from the current GL state
        UPDATE_RENDER_ENCODER: 1,
        /// Create a fresh render pipeline state from the current GL state
        NEW_RENDER_PIPELINE: 2,
        /// Update buffer maps (e.g. when a new VAO or program is bound)
        REMAP_BUFFERS: 3,
    }
}
impl Dirty {
    #[inline]
    pub(crate) fn any_set(self, other: Self) -> bool {
        self.intersects(other)
    }
    #[inline]
    pub(crate) fn set_bits(&mut self, other: Self) {
        *self = self.union(other);
    }
    #[inline]
    pub(crate) fn unset(&mut self, other: Self) {
        *self = self.difference(other);
    }
}
impl Context {
    #[inline]
    pub(crate) fn new_encoder(&mut self) {
        // need to update the new encoder after creation
        self.platform_state
            .dirty_state
            .set_bits(Dirty::NEW_RENDER_ENCODER);
    }
    #[inline]
    pub(crate) fn update_encoder(&mut self) {
        self.platform_state
            .dirty_state
            .set_bits(Dirty::UPDATE_RENDER_ENCODER);
    }
    #[inline]
    pub(crate) fn remap_buffers(&mut self) {
        self.platform_state
            .dirty_state
            .set_bits(Dirty::REMAP_BUFFERS);
    }
    #[inline]
    pub(crate) fn new_pipeline(&mut self) {
        self.platform_state
            .dirty_state
            .set_bits(Dirty::NEW_RENDER_PIPELINE);
    }
}
const MTL_MAX_ARGUMENT_BINDINGS: usize = 31;

/// Utility that maps currently active object names to their location in the relevant Metal shader parameter table
#[derive(Debug)]
pub(crate) struct ResourceMap<T: NamedObject, const MAX_ENTRIES: usize = 32> {
    inner: HashMap<ObjectName<T>, u32>,
    buf: HashSet<u32>,
    #[cfg(debug_assertions)]
    dbg_check: HashMap<u32, ObjectName<T>>,
}
impl<const MAX_ENTRIES: usize, T: NamedObject + 'static> ResourceMap<T, MAX_ENTRIES> {
    pub(crate) fn new() -> Self {
        Self {
            inner: HashMap::new(),
            buf: HashSet::new(),
            #[cfg(debug_assertions)]
            dbg_check: HashMap::new(),
        }
    }
    /// Build
    #[inline]
    pub(crate) fn build(
        &mut self,
        pinned_resources: &[(ObjectName<T>, u8)],
        include_resources: &[ObjectName<T>],
    ) {
        self.inner.clear();
        self.buf.clear();
        #[cfg(debug_assertions)]
        self.dbg_check.clear();
        for (name, idx) in pinned_resources.iter().copied() {
            self.inner.insert(name, u32::from(idx));
            self.buf.insert(u32::from(idx));
            #[cfg(debug_assertions)]
            if let Some(prev_obj) = self.dbg_check.insert(u32::from(idx), name) {
                panic!(
                    "tried to assign objects {name:?} and {prev_obj:?} to the same fixed argument table index ({idx})"
                );
            }
        }
        #[expect(clippy::cast_possible_truncation, reason = "const checked")]
        let mut ctr = MAX_ENTRIES as u32 - 1;
        assert!(
            pinned_resources.len() + include_resources.len() <= MAX_ENTRIES,
            "OxideGL exceeded the maximum number of Metal buffer binding points ({})",
            MAX_ENTRIES
        );
        for res in include_resources.iter().copied() {
            loop {
                // Could theoretically cause infinite loop, assert above should prevent this
                if self.buf.contains(&ctr) {
                    ctr = ctr.saturating_sub(1);
                } else {
                    break;
                }
            }
            self.inner.insert(res, ctr);
            ctr = ctr.saturating_sub(1);
        }
    }
    #[inline]
    pub(crate) fn get(&self, obj: ObjectName<T>) -> Option<u32> {
        self.inner.get(&obj).copied()
    }
}

#[allow(clippy::undocumented_unsafe_blocks)]
impl PlatformState {
    pub(crate) fn set_view(&mut self, view: &Retained<NSView>, backing_scale_factor: f64) {
        self.view = Some(view.clone());
        self.layer.setFrame(view.frame());
        self.layer.setContentsScale(backing_scale_factor);
        // ensure the view is layer-backed
        view.setWantsLayer(true);
        // set the backing layer
        unsafe { view.setLayer(Some(&self.layer)) };

        trace!("injected layer {:?} into NSView", &self.layer);
    }
    pub(crate) fn swap_buffers(&mut self, state: &mut GLState) {
        self.update_state(state, false);

        self.end_encoding();

        if let Some(drawable) = self.drawable.take() {
            self.current_command_buffer()
                .presentDrawable(drawable.as_ref());
            drop(drawable);
        }
        self.current_command_buffer().commit();
        self.command_buffer = None;
    }
    pub(crate) fn new(
        pixel_format: MTLPixelFormat,
        depth_format: Option<MTLPixelFormat>,
        stencil_format: Option<MTLPixelFormat>,
    ) -> Self {
        let device = MTLCreateSystemDefaultDevice().unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            layer.setPixelFormat(pixel_format);
            layer.setDevice(Some(&device));
            layer.setFramebufferOnly(false);
        };
        layer.setMagnificationFilter(unsafe { kCAFilterNearest });

        // use `info` instead of `gl_info` because gl logging state might not be initialized yet
        info!("Metal device: {}", device.name());
        let queue = device
            .newCommandQueue()
            .expect("failed to create command queue");
        queue.setLabel(Some(ns_string!("OxideGL command queue")));

        let props = MetalProperties::new(&device);
        Self {
            dirty_state: Dirty::all(),

            view: None,
            device,
            props,
            queue,
            layer,
            drawable: None,
            command_buffer: None,
            blit_command_buffer: None,
            blit_encoder: None,

            internal_drawables: InternalDrawables::default(),

            render_encoder: None,
            render_pipeline_state: None,

            vertex_buffer_map: ResourceMap::new(),
            vertex_buffer_offsets: HashMap::new(),

            fragment_buffer_map: ResourceMap::new(),

            pixel_format,
            depth_format,
            stencil_format,
        }
    }
    #[inline]
    fn new_command_buffer(
        queue: &ProtoObjRef<dyn MTLCommandQueue>,
        label: Option<&'static NSString>,
    ) -> ProtoObjRef<dyn MTLCommandBuffer> {
        let buf;

        // use extra metal debugging/validation when debug assertions are enabled
        #[cfg(debug_assertions)]
        unsafe {
            let desc = MTLCommandBufferDescriptor::new();
            desc.setErrorOptions(MTLCommandBufferErrorOption::EncoderExecutionStatus);
            buf = queue
                .commandBufferWithDescriptor(&desc)
                .expect("failed to create command buffer");
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            buf = queue
                .commandBuffer()
                .expect("failed to create command buffer")
        }
        if let Some(v) = label {
            buf.setLabel(Some(v));
        }
        buf
    }
    #[inline]
    fn current_command_buffer(&mut self) -> &ProtoObjRef<dyn MTLCommandBuffer> {
        self.command_buffer.get_or_insert_with(|| {
            Self::new_command_buffer(
                &self.queue,
                Some(ns_string!("OxideGL render command buffer")),
            )
        })
    }
    #[inline]
    #[track_caller]
    pub(crate) fn current_render_encoder(&mut self) -> &ProtoObjRef<dyn MTLRenderCommandEncoder> {
        self.render_encoder
            .as_ref()
            .expect("render command encoder should have been created!")
    }
    #[inline]
    pub(crate) fn end_encoding(&mut self) {
        if let Some(enc) = &self.render_encoder {
            enc.endEncoding();
        }
        self.render_encoder = None;
    }
    //TODO: use onresized or something for updating drawable size instead of effectively polling every frame
    #[inline]
    #[track_caller]
    pub(crate) fn current_drawable(&mut self) -> &ProtoObjRef<dyn CAMetalDrawable> {
        self.drawable.get_or_insert_with(|| {
            let view = self
                .view
                .as_ref()
                .expect("Can't get metal drawable before attaching Context to a view");
            let maybe_new_size = unsafe { view.convertSizeToBacking(view.frame().size) };
            if maybe_new_size != unsafe { self.layer.drawableSize() } {
                unsafe {
                    self.layer.setDrawableSize(maybe_new_size);
                };
                self.dirty_state |= Dirty::NEW_RENDER_ENCODER;
            }

            unsafe { self.layer.nextDrawable() }
                .expect("Failed to get next drawable from CAMetalLayer")
        })
    }

    /// Core function of OpenGL state machine emulation. "steps" the state forward,
    /// reintegrating all of the state that has been made dirty since the last step
    pub(crate) fn update_state(&mut self, state: &mut GLState, is_draw_command: bool) {
        // if we don't currently have an encoder, make sure we make a new one
        // (in case there are no state changes that mark it dirty over the course of a frame)
        if self.render_encoder.is_none() {
            self.dirty_state
                .set_bits(Dirty::NEW_RENDER_ENCODER | Dirty::UPDATE_RENDER_ENCODER);
        }

        let all_dirty = self.dirty_state;
        // if there is no VAO, we still need to update render encoder state for some commands like glClear but not do any further work
        if state.vao_binding.is_none() {
            if is_draw_command {
                panic!("tried to call a draw command without a VAO bound")
            } else {
                self.end_encoding();
                self.render_encoder = Some(self.build_render_encoder(state));
                return;
            }
        }

        if all_dirty.any_set(Dirty::REMAP_BUFFERS) {
            self.remap_buffer_arguments(state);
        }

        if all_dirty.any_set(Dirty::NEW_RENDER_ENCODER) || self.render_encoder.is_none() {
            gl_trace!("generating new render command encoder");
            self.end_encoding();
            self.render_encoder = Some(self.build_render_encoder(state));
            self.dirty_state.unset(Dirty::NEW_RENDER_ENCODER);
        }
        // this code path is taken if we have a new encoder and need to finish initializing it, or if we just need to update the dynamic state of the current encoder
        if all_dirty.any_set(Dirty::UPDATE_RENDER_ENCODER | Dirty::NEW_RENDER_ENCODER) {
            self.update_encoder(state);
            self.dirty_state.unset(Dirty::UPDATE_RENDER_ENCODER);
        }
        if all_dirty.any_set(Dirty::NEW_RENDER_PIPELINE) {
            gl_trace!("generating new render pipeline state");
            self.render_pipeline_state = Some(self.build_render_pipeline_state(state));
            self.dirty_state.unset(Dirty::NEW_RENDER_PIPELINE);
        }
        let ps = self.render_pipeline_state.as_ref().unwrap();
        let enc = self.render_encoder.as_ref().unwrap();
        enc.setRenderPipelineState(ps);
    }
    //preconditions: buffer maps built, renderable program present
    pub(crate) fn build_render_pipeline_state(
        &mut self,
        state: &mut GLState,
    ) -> ProtoObjRef<dyn MTLRenderPipelineState> {
        let desc = MTLRenderPipelineDescriptor::new();
        #[cfg(debug_assertions)]
        desc.setLabel(Some(ns_string!("OxideGL render pipeline")));
        let (Some(f), Some(v)) = (
            Self::linked_stage(state, ShaderType::FragmentShader),
            Self::linked_stage(state, ShaderType::VertexShader),
        ) else {
            panic!(
                "Tried to build a render pipeline while missing a linked vertex or fragment shader stage"
            );
        };
        let attachments = desc.colorAttachments();
        if state.framebuffer_binding.is_some() {
            todo!()
        } else {
            for (i, mode) in state.default_draw_buffers.modes.iter().enumerate() {
                if mode.is_some() {
                    let attachment_desc =
                        unsafe { MTLRenderPipelineColorAttachmentDescriptor::new() };
                    attachment_desc.setPixelFormat(self.pixel_format);

                    // Apply blend state if present
                    state.blend.drawbuffer_states[i].apply_to_mtl_desc(&attachment_desc);
                    unsafe { attachments.setObject_atIndexedSubscript(Some(&attachment_desc), i) };
                }
            }
            if state.caps.is_any_enabled(Capabilities::DEPTH_TEST) {
                desc.setDepthAttachmentPixelFormat(
                    self.depth_format
                    .expect("Tried to use depth test on the default framebuffer without specifying a depth format during context creation!")
                );
            }
            //TODO depth/stencil attachment formats
        }
        desc.setVertexFunction(Some(&v.function));
        desc.setFragmentFunction(Some(&f.function));
        //TODO: primitive topology real
        // unsafe { desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Triangle) };
        let v_desc = self.build_vertex_descriptor(state);
        desc.setVertexDescriptor(Some(&v_desc));
        self.device
            .newRenderPipelineStateWithDescriptor_error(&desc)
            .expect("failed to create pipeline state")
        // TODO clear state, depth test config, scissor box
    }
    // precondition: buffers mapped
    fn update_encoder(&mut self, state: &mut GLState) {
        fn stencil_descriptor_for_stencil_state(
            state: &StencilFaceState,
            writemask: u32,
        ) -> Retained<MTLStencilDescriptor> {
            let desc = unsafe { MTLStencilDescriptor::new() };
            desc.setStencilCompareFunction(state.func.into());
            desc.setStencilFailureOperation(state.fail_action.into());

            desc.setDepthFailureOperation(state.depth_fail_action.into());
            desc.setDepthStencilPassOperation(state.depth_pass_action.into());

            desc.setWriteMask(writemask);
            desc.setReadMask(state.mask);
            desc
        }
        self.bind_buffers_to_render_encoder(state);
        let enc;
        if state
            .caps
            .is_any_enabled(Capabilities::DEPTH_TEST | Capabilities::STENCIL_TEST)
        {
            let desc = unsafe { MTLDepthStencilDescriptor::new() };
            if state.caps.is_any_enabled(Capabilities::DEPTH_TEST) {
                desc.setDepthCompareFunction(state.depth_func.into());
                desc.setDepthWriteEnabled(state.writemasks.depth);
            }
            if state.caps.is_any_enabled(Capabilities::STENCIL_TEST) {
                let front = stencil_descriptor_for_stencil_state(
                    &state.stencil.front,
                    state.writemasks.stencil_front,
                );
                desc.setFrontFaceStencil(Some(&front));
                let back = stencil_descriptor_for_stencil_state(
                    &state.stencil.back,
                    state.writemasks.stencil_back,
                );
                desc.setBackFaceStencil(Some(&back));
            }
            let ds_state = self
                .device
                .newDepthStencilStateWithDescriptor(&desc)
                .expect("failed to create MTLDepthStencilState");
            enc = self.current_render_encoder();
            enc.setDepthStencilState(Some(&ds_state));
        } else {
            enc = self.current_render_encoder();
        }
        if state.caps.is_any_enabled(Capabilities::CULL_FACE) {
            enc.setCullMode(state.cull_face_mode.into());
        }

        // we *could* set this only when blending is actually enabled, but that's done on a per-attachment basis anyways (and
        // this call is quite cheap (just sets a similar variable somewhere within the encoder state)
        let blend_col = state.blend.blend_color;
        enc.setBlendColorRed_green_blue_alpha(
            blend_col[0],
            blend_col[1],
            blend_col[2],
            blend_col[3],
        );

        // TODO scissor test
        #[expect(
            clippy::cast_lossless,
            reason = "pixel aligned rect values are always 32 bits, and as such are exactly representable as f64"
        )]
        enc.setViewport(MTLViewport {
            originX: state.viewport.x as f64,
            originY: state.viewport.y as f64,
            width: state.viewport.width as f64,
            height: state.viewport.height as f64,
            // TODO: depth range
            znear: 0.0,
            zfar: 1.0,
        });
    }
    fn bind_buffers_to_render_encoder(&mut self, state: &mut GLState) {
        let enc = self.render_encoder.as_ref().unwrap();
        for (&buf, &binding) in &self.vertex_buffer_map.inner {
            let buf_obj = state.buffer_list.get(buf);
            gl_trace!("binding {buf:?} to metal argument table index {binding}");
            if let Some(alloc) = buf_obj.allocation.as_ref() {
                unsafe {
                    enc.setVertexBuffer_offset_atIndex(
                        Some(&alloc.mtl),
                        *self.vertex_buffer_offsets.get(&buf).unwrap(),
                        binding as usize,
                    );
                };
            }
        }
    }
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "we hope this works"
    )]
    #[inline]
    // "guesstimate," it can still get out of sync with the actual size of the current drawable
    pub(crate) fn target_defaultfb_dims(&mut self) -> (u32, u32) {
        // reproduce the calculation done by the CAMetalLayer when generating the next drawable size
        // (from https://developer.apple.com/documentation/quartzcore/cametallayer/1478174-drawablesize)
        let size = self.layer.bounds().size;
        let scale = self.layer.contentsScale();
        let size = (size.width * scale, size.height * scale);
        debug_assert!(
            (size.0 - size.0.floor()) == 0.0
                && (size.1 - size.1.floor()) == 0.0
                && size.0 > 0.0
                && size.1 > 0.0,
            "bad size ({size:?})"
        );
        (size.0 as u32, size.1 as u32)
    }
    #[expect(clippy::cast_possible_truncation, reason = "we hope this works")]
    //preconditions: view set on context
    pub(crate) fn build_render_encoder(
        &mut self,
        state: &mut GLState,
    ) -> ProtoObjRef<dyn MTLRenderCommandEncoder> {
        let desc = unsafe { MTLRenderPassDescriptor::new() };

        if let Some(fbo) = state.framebuffer_binding {
            // user-defined FBO
            todo!()
        } else {
            // default FBO
            let mut iter = state
                .default_draw_buffers
                .modes
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(idx, v)| v.map(|v| (idx, v)))
                .peekable();
            //FIXME this expect contradicts the spec, should be an early return of some kind
            let &(_, first) = iter.peek().expect("No draw buffer set");
            let ca_drawable_tex = unsafe { self.current_drawable().texture() };

            // Use the current drawable size as the targeted size for rendering. If the drawable size changes, a new
            // render encoder will be created, which will inherit the new size from the new drawawable
            let dims = (
                ca_drawable_tex.width() as u32,
                ca_drawable_tex.height() as u32,
            );

            if state.caps.is_any_enabled(Capabilities::DEPTH_TEST) {
                let a_desc = unsafe { MTLRenderPassDepthAttachmentDescriptor::new() };
                a_desc.setTexture(Some(&self.get_internal_depthbuffer(dims).tex));

                desc.setDepthAttachment(Some(&a_desc));
            }
            if state.caps.is_any_enabled(Capabilities::STENCIL_TEST) {
                let a_desc = unsafe { MTLRenderPassStencilAttachmentDescriptor::new() };
                a_desc.setTexture(Some(&self.get_internal_stencilbuffer(dims).tex));
                desc.setStencilAttachment(Some(&a_desc));
            }

            let drawbuffer = self.get_internal_drawbuffer(first, dims);
            for (idx, buf) in iter {
                let a_desc = MTLRenderPassColorAttachmentDescriptor::new();
                // set attachment texture
                if buf == DrawBufferMode::FrontLeft {
                    // Replace the texture with the current drawable
                    debug_assert_eq!(
                        (
                            ca_drawable_tex.width() as u32,
                            ca_drawable_tex.height() as u32
                        ),
                        drawbuffer.dimensions,
                        "Metal drawable had different dimensions than associated drawbuffer!"
                    );
                    a_desc.setTexture(Some(&ca_drawable_tex));
                } else {
                    a_desc.setTexture(Some(&drawbuffer.tex));
                }

                // FIXME need to do load action real
                // a_desc.setLoadAction(MTLLoadAction::Clear);
                // a_desc.setClearColor(MTLClearColor {
                //     red: 1.0,
                //     green: 0.0,
                //     blue: 0.0,
                //     alpha: 0.5,
                // });
                unsafe {
                    desc.colorAttachments()
                        .setObject_atIndexedSubscript(Some(&a_desc), idx);
                };
            }
            desc.setRenderTargetWidth(dims.0 as usize);
            desc.setRenderTargetHeight(dims.1 as usize);
        }
        let enc = self
            .current_command_buffer()
            .renderCommandEncoderWithDescriptor(&desc)
            .expect("failed to create new render command encoder");
        #[cfg(debug_assertions)]
        enc.setLabel(Some(ns_string!("OxideGL render encoder")));
        enc
    }
    #[inline]
    fn check_and_resize_drawable<'a>(
        device: &ProtoObjRef<dyn MTLDevice>,
        dims: (u32, u32),
        pixel_format: MTLPixelFormat,
        gpu_private: bool,
        r: &'a mut Option<InternalDrawable>,
    ) -> &'a InternalDrawable {
        if r.as_ref().is_none_or(|v| v.dimensions != dims) {
            // Need a new internal drawable
            let new_tex = Self::new_drawbuffer_size_format(device, dims, pixel_format, gpu_private);
            let mut replacement = Some(InternalDrawable::new(new_tex, dims));
            mem::swap(r, &mut replacement);
            drop(replacement);
        };
        // must be Some due to code above
        r.as_ref().unwrap()
    }
    pub(crate) fn get_internal_drawbuffer(
        &mut self,
        target: DrawBufferMode,
        dims: (u32, u32),
    ) -> &InternalDrawable {
        gl_trace!("getting internal (default FB) drawbuffer for {target:?}");
        let r = match target {
            DrawBufferMode::FrontLeft => &mut self.internal_drawables.front_left,
            DrawBufferMode::FrontRight => &mut self.internal_drawables.front_right,
            DrawBufferMode::BackLeft => &mut self.internal_drawables.back_left,
            DrawBufferMode::BackRight => &mut self.internal_drawables.back_right,
            _ => todo!("oxidegl does not support aliased draw buffer modes"),
        };
        Self::check_and_resize_drawable(&self.device, dims, self.pixel_format, false, r)
    }
    // precondition: user specifies depth format for defaultfb
    pub(crate) fn get_internal_depthbuffer(&mut self, dims: (u32, u32)) -> &InternalDrawable {
        Self::check_and_resize_drawable(
            &self.device,
            dims,
            self.depth_format.expect("tried to generate a depth buffer for the default framebuffer, but no depth format was specified at context creation!"),
            true,
            &mut self.internal_drawables.depth,
        )
    }
    // precondition: user specifies stencil format for defaultfb
    pub(crate) fn get_internal_stencilbuffer(&mut self, dims: (u32, u32)) -> &InternalDrawable {
        Self::check_and_resize_drawable(
            &self.device,
            dims,
            self.depth_format.expect("tried to generate a stencil buffer for the default framebuffer, but no stencil format was specified at context creation!"),
            true,
            &mut self.internal_drawables.stencil,
        )
    }
    pub(crate) fn new_drawbuffer_size_format(
        device: &ProtoObjRef<dyn MTLDevice>,
        size: (u32, u32),
        format: MTLPixelFormat,
        gpu_private: bool,
    ) -> ProtoObjRef<dyn MTLTexture> {
        gl_debug!(
            "creating new {}x{} {format:?} drawable texture",
            size.0,
            size.1
        );
        let desc = unsafe { MTLTextureDescriptor::new() };
        if gpu_private {
            desc.setStorageMode(MTLStorageMode::Private);
            desc.setAllowGPUOptimizedContents(true);
        }
        unsafe { desc.setWidth(size.0 as usize) };
        unsafe { desc.setHeight(size.1 as usize) };
        desc.setPixelFormat(format);
        desc.setUsage(MTLTextureUsage::RenderTarget);
        device
            .newTextureWithDescriptor(&desc)
            .expect("failed to create drawable texture")
    }
    #[inline]
    pub(crate) fn linked_stage(state: &GLState, shader_type: ShaderType) -> Option<&LinkedStage> {
        //TODO: handle program pipelines
        let program = state
            .program_list
            .get(state.program_binding.expect("No program bound"));
        let link = program
            .latest_linkage
            .as_ref()
            .expect("Program should have been linked");
        match shader_type {
            ShaderType::FragmentShader => &link.fragment,
            ShaderType::VertexShader => &link.vertex,
            ShaderType::ComputeShader => &link.compute,

            ShaderType::TessEvaluationShader => todo!(),
            ShaderType::TessControlShader => todo!(),
            ShaderType::GeometryShader => todo!(),
        }
        .as_ref()
    }

    /// precondition: has program
    pub(crate) fn remap_buffer_arguments(&mut self, state: &mut GLState) {
        if let (Some(vert), Some(vao)) = (
            Self::linked_stage(state, ShaderType::VertexShader),
            state.vao_binding.and_then(|v| state.vao_list.get_opt(v)),
        ) {
            self.vertex_buffer_offsets.clear();
            let vertex_buffers = vao
                .buffer_bindings
                .iter()
                .filter_map(|v| v.buf.map(|n| (n, v.offset)))
                .map(|(name, offset)| {
                    self.vertex_buffer_offsets.insert(name, offset);
                    name
                })
                .collect::<Vec<_>>();
            let pinned_buffers = Self::stage_pinned_buffers(state, vert);
            self.vertex_buffer_map
                .build(&pinned_buffers, &vertex_buffers);
        }
        if let Some(frag) = Self::linked_stage(state, ShaderType::FragmentShader) {
            self.fragment_buffer_map
                .build(&Self::stage_pinned_buffers(state, frag), &[]);
        }
    }

    // broadcast panic location up into calling functions for easier debugging
    #[track_caller]
    #[inline]
    // TODO cache this info inside of LinkedShaderStage
    fn stage_pinned_buffers(
        state: &GLState,
        shader_stage: &LinkedStage,
    ) -> Vec<(ObjectName<Buffer>, u8)> {
        let mut v = Vec::new();
        for ssbo in &shader_stage.resources.shader_storage_buffers {
            let binding = ssbo
                .binding
                .expect("SSBO declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!(
                    "Shader requested SSBO at binding {binding} but no buffer was bound at that index"
                );
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        for ubo in &shader_stage.resources.uniform_buffers {
            let binding = ubo
                .binding
                .expect("UBO declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!(
                    "Shader requested UBO at binding {binding} but no buffer was bound at that index"
                );
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        for acb in &shader_stage.resources.atomic_counter_buffers {
            let binding = acb
                .binding
                .expect("atomic counter buffer declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!(
                    "Shader requested atomic counter buffer at binding {binding} but no buffer was bound at that index"
                );
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        v
    }
    /// precondition: Buffer maps built, VAO present
    pub(crate) fn build_vertex_descriptor(
        &mut self,
        state: &mut GLState,
    ) -> Retained<MTLVertexDescriptor> {
        gl_trace!("generating Metal vertex descriptor from GL VAO state");
        let vao = state.vao_list.get(state.vao_binding.unwrap());
        let mtl_vertex_desc = unsafe { MTLVertexDescriptor::new() };
        for bdg in &vao.buffer_bindings {
            if let Some(buf) = bdg.buf {
                let buffer_argument_index = self.vertex_buffer_map.get(buf).unwrap();
                let desc = MTLVertexBufferLayoutDescriptor::new();
                unsafe { desc.setStride(bdg.stride.into()) };

                unsafe {
                    mtl_vertex_desc
                        .layouts()
                        .setObject_atIndexedSubscript(Some(&desc), buffer_argument_index as usize);
                };
            }
        }
        for (idx, attr) in vao.attribs.iter().enumerate() {
            if !attr.enabled {
                continue;
            }
            let attr_binding = vao.buffer_bindings[attr.buffer_idx as usize];

            let mtl_attrib_desc = MTLVertexAttributeDescriptor::new();

            // Get the index of the corresponding buffer object in the vertex shader argument table
            let buffer_argument_index = self
                .vertex_buffer_map
                .get(attr_binding.buf.expect("Buffer for attribute not bound"))
                .unwrap();

            unsafe { mtl_attrib_desc.setBufferIndex(buffer_argument_index as usize) };
            mtl_attrib_desc.setFormat(attr.get_mtl_layout().to_vertex_format());
            unsafe { mtl_attrib_desc.setOffset(attr.relative_offset as usize) };

            unsafe {
                mtl_vertex_desc
                    .attributes()
                    .setObject_atIndexedSubscript(Some(&mtl_attrib_desc), idx);
            };
        }

        mtl_vertex_desc
    }
}

impl From<DepthFunction> for MTLCompareFunction {
    fn from(value: DepthFunction) -> Self {
        // 1:1 correspondance between depthfunc and MTLCompareFunction after an offsetting subtraction
        Self((value as u32 - DepthFunction::Never as u32) as usize)
    }
}
impl From<StencilFunction> for MTLCompareFunction {
    fn from(value: StencilFunction) -> Self {
        // 1:1 correspondance between stencilfunc and MTLCompareFunction after an offsetting subtraction
        Self((value as u32 - StencilFunction::Never as u32) as usize)
    }
}

impl From<StencilOp> for MTLStencilOperation {
    #[inline]
    fn from(value: StencilOp) -> Self {
        match value {
            StencilOp::Zero => Self::Zero,
            StencilOp::Invert => Self::Invert,
            StencilOp::Keep => Self::Keep,
            StencilOp::Replace => Self::Replace,
            StencilOp::Incr => Self::IncrementClamp,
            StencilOp::Decr => Self::DecrementClamp,
            StencilOp::IncrWrap => Self::IncrementWrap,
            StencilOp::DecrWrap => Self::DecrementWrap,
        }
    }
}

impl DrawbufferBlendState {
    #[inline]
    fn apply_to_mtl_desc(&self, desc: &Retained<MTLRenderPipelineColorAttachmentDescriptor>) {
        if !self.blend_enabled {
            return;
        }
        desc.setBlendingEnabled(true);

        desc.setSourceRGBBlendFactor(self.src_rgb.into());
        desc.setSourceAlphaBlendFactor(self.src_alpha.into());

        desc.setDestinationRGBBlendFactor(self.dst_rgb.into());
        desc.setDestinationAlphaBlendFactor(self.dst_alpha.into());

        desc.setRgbBlendOperation(self.eq_rgb.into());
        desc.setAlphaBlendOperation(self.eq_alpha.into());
    }
}
impl From<ColorWriteMask> for MTLColorWriteMask {
    #[inline]
    fn from(value: ColorWriteMask) -> Self {
        let arr: [bool; 4] = value.into();
        // See: MTLRenderPipeline.h L#54
        let val = usize::from(arr[3])
            | (usize::from(arr[2]) << 1)
            | (usize::from(arr[1]) << 2)
            | (usize::from(arr[0]) << 3);
        Self(val)
    }
}
impl From<TriangleFace> for MTLCullMode {
    #[inline]
    fn from(value: TriangleFace) -> Self {
        match value {
            TriangleFace::Front => Self::Front,
            //FIXME this is incorrect. This should cull all faces but not lines. This will be a fun™ behavior to emulate with transform feedback
            TriangleFace::FrontAndBack => Self::None,
            TriangleFace::Back => Self::Back,
        }
    }
}
