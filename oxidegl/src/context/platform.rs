use std::mem;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use log::{info, trace};
use objc2::rc::Retained;
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::{is_main_thread, ns_string, MainThreadMarker, NSString};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLClearColor, MTLCommandBuffer, MTLCommandBufferDescriptor,
    MTLCommandBufferErrorOption, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice,
    MTLDevice, MTLLoadAction, MTLPixelFormat, MTLRenderCommandEncoder,
    MTLRenderPassColorAttachmentDescriptor, MTLRenderPassDepthAttachmentDescriptor,
    MTLRenderPassDescriptor, MTLRenderPassStencilAttachmentDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureUsage,
    MTLVertexAttributeDescriptor, MTLVertexBufferLayoutDescriptor, MTLVertexDescriptor,
    MTLViewport,
};
use objc2_quartz_core::{kCAFilterNearest, CAMetalDrawable, CAMetalLayer};

use crate::{
    context::debug::{gl_info, gl_trace},
    enums::{DrawBufferMode, ShaderType},
    ProtoObjRef,
};

use super::{
    commands::buffer::Buffer,
    framebuffer::InternalDrawable,
    program::LinkedShaderStage,
    state::{GLState, NamedObject, ObjectName},
};

#[derive(Debug)]
pub struct PlatformState {
    /// dirty components
    pub(crate) dirty_state: NeedsRefreshBits,

    /// the View this state is associated with
    view: Option<Retained<NSView>>,

    /// Metal device
    pub(crate) device: ProtoObjRef<dyn MTLDevice>,

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
#[derive(Debug, Clone)]
pub struct InternalDrawables {
    front_left: Option<InternalDrawable>,
    front_right: Option<InternalDrawable>,
    back_left: Option<InternalDrawable>,
    back_right: Option<InternalDrawable>,
}
impl InternalDrawables {
    fn new() -> Self {
        Self {
            front_left: None,
            front_right: None,
            back_left: None,
            back_right: None,
        }
    }
}
const MTL_MAX_ARGUMENT_BINDINGS: usize = 31;
bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NeedsRefreshBits: u32 {
        /// Create a fresh render encoder from the current GL state
        const NEW_RENDER_ENCODER = 1;
        /// Update dynamic render encoder state from the current GL state
        const UPDATE_CURRENT_ENCODER = 1 << 1;
        /// Create a fresh render pipeline state from the current GL state
        const NEW_RENDER_PIPELINE = 1 << 2;
        /// Update buffer maps (e.g. when a new VAO or program is bound)
        const REMAP_BUFFERS = 1 << 3;


    }
}

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
        for (name, idx) in pinned_resources.iter().copied() {
            self.inner.insert(name, u32::from(idx));
            self.buf.insert(u32::from(idx));
            #[cfg(debug_assertions)]
            if let Some(prev_obj) = self.dbg_check.insert(u32::from(idx), name) {
                panic!("tried to assign objects {name:?} and {prev_obj:?} to the same fixed argument table index ({idx})");
            }
        }
        #[expect(clippy::cast_possible_truncation, reason = "const checked")]
        let mut ctr = MAX_ENTRIES as u32 - 1;
        assert!(
            pinned_resources.len() + include_resources.len() <= MAX_ENTRIES,
            "OxideGL exceeded the maximum available resource "
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
    pub(crate) fn set_view(&mut self, view: &Retained<NSView>) {
        self.view = Some(view.clone());
        self.layer.setFrame(view.frame());
        // set backing layer
        unsafe { view.setLayer(Some(&self.layer)) };
        // tell the view it is now layer-backed
        view.setWantsLayer(true);

        trace!("injected layer {:?} into NSView", &self.layer);
    }
    pub(crate) fn swap_buffers(&mut self, state: &mut GLState) {
        self.update_state(state, false);

        self.end_encoding();

        if let Some(drawable) = self.drawable.take() {
            self.current_command_buffer()
                .presentDrawable(drawable.as_ref().as_ref());
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
        let device = unsafe { Retained::from_raw(MTLCreateSystemDefaultDevice()) }.unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            layer.setPixelFormat(pixel_format);
            layer.setDevice(Some(&device));
            layer.setFramebufferOnly(false);
        };
        layer.setMagnificationFilter(unsafe { kCAFilterNearest });

        // use `info` because gl logging state isn't initialized yet
        info!("Metal device: {}", device.name());
        let queue = device
            .newCommandQueue()
            .expect("failed to create command queue");
        queue.setLabel(Some(ns_string!("OxideGL command queue")));

        Self {
            dirty_state: NeedsRefreshBits::all(),

            view: None,
            device,
            queue,
            layer,
            drawable: None,
            command_buffer: None,
            blit_command_buffer: None,
            blit_encoder: None,

            internal_drawables: InternalDrawables::new(),

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

        //use extra metal debugging/validation when debug assertions are enabled
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
    pub(crate) fn update_state(&mut self, state: &mut GLState, is_draw_command: bool) {
        // if there is no VAO, we still need to modify render encoder state for some commands like glClear but not do any further work
        if state.vao_binding.is_none() {
            if is_draw_command {
                panic!("tried to call a draw command without a VAO bound")
            } else {
                self.end_encoding();
                self.render_encoder = Some(self.build_render_encoder(state));
                return;
            }
        }

        if self.dirty_state.contains(NeedsRefreshBits::REMAP_BUFFERS) {
            self.remap_buffer_arguments(state);
        }

        if self
            .dirty_state
            .contains(NeedsRefreshBits::NEW_RENDER_ENCODER)
        {
            gl_trace!("generating new render command encoder");
            self.end_encoding();
            self.render_encoder = Some(self.build_render_encoder(state));
            self.update_dynamic_encoder_state(state);
        }
        if self
            .dirty_state
            .contains(NeedsRefreshBits::UPDATE_CURRENT_ENCODER)
        {
            self.update_dynamic_encoder_state(state);
        }
        if self
            .dirty_state
            .contains(NeedsRefreshBits::NEW_RENDER_PIPELINE)
        {
            gl_trace!("generating new render pipeline state");
            self.render_pipeline_state = Some(self.build_render_pipeline_state(state));
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
            panic!("Tried to build a render pipeline while missing a linked vertex or fragment shader stage");
        };
        let attachments = desc.colorAttachments();
        if state.framebuffer_binding.is_some() {
            todo!()
        } else {
            for i in 0..state.default_draw_buffers.drawbuf_iter().count() {
                unsafe {
                    attachments
                        .objectAtIndexedSubscript(i)
                        .setPixelFormat(self.pixel_format);
                }
            }
            //TODO depth/stencil
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
    fn update_dynamic_encoder_state(&mut self, state: &mut GLState) {
        self.bind_buffers_to_encoder(state);

        #[expect(
            clippy::cast_lossless,
            reason = "pixel aligned rect only uses 32, always exactly representable as f64"
        )]
        self.current_render_encoder().setViewport(MTLViewport {
            originX: state.viewport.x as f64,
            originY: state.viewport.y as f64,
            width: state.viewport.width as f64,
            height: state.viewport.height as f64,
            // TODO: depth range
            znear: 0.0,
            zfar: 1.0,
        });
    }
    fn bind_buffers_to_encoder(&mut self, state: &mut GLState) {
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
    pub(crate) fn current_defaultfb_dimensions(&mut self) -> (u32, u32) {
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
                .drawbuf_iter()
                .enumerate()
                .peekable();
            //FIXME this expect contradicts the spec, should be an early return of some kind
            let &(_, first) = iter.peek().expect("No draw buffer set");
            let ca_drawable_tex = unsafe { self.current_drawable().texture() };
            let dims = (
                ca_drawable_tex.width() as u32,
                ca_drawable_tex.height() as u32,
            );
            let drawbuffer = self.get_internal_drawbuffer(first, dims);
            // When multiple draw buffers are present with the default FB it's somewhat unclear which one should be responsible for the depth/stencil drawables, so we use the first one (index 0) as a sane default
            if let Some(depth) = &drawbuffer.depth {
                let a_desc = unsafe { MTLRenderPassDepthAttachmentDescriptor::new() };
                a_desc.setTexture(Some(depth));
                desc.setDepthAttachment(Some(&a_desc));
            }
            if let Some(stencil) = &drawbuffer.stencil {
                let a_desc = unsafe { MTLRenderPassStencilAttachmentDescriptor::new() };
                a_desc.setTexture(Some(stencil));
                desc.setStencilAttachment(Some(&a_desc));
            }

            for (idx, buf) in iter {
                let a_desc = MTLRenderPassColorAttachmentDescriptor::new();
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
                    a_desc.setTexture(Some(&drawbuffer.color));
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
    pub(crate) fn get_internal_drawbuffer(
        &mut self,
        target: DrawBufferMode,
        viewport_dims: (u32, u32),
    ) -> &InternalDrawable {
        gl_trace!("getting internal (default FB) drawbuffer for {target:?}");
        let r = match target {
            DrawBufferMode::FrontLeft => &mut self.internal_drawables.front_left,
            DrawBufferMode::FrontRight => &mut self.internal_drawables.front_right,
            DrawBufferMode::BackLeft => &mut self.internal_drawables.back_left,
            DrawBufferMode::BackRight => &mut self.internal_drawables.back_right,
            _ => todo!("oxidegl does not support aliased draw buffer modes"),
        };
        let device = &self.device;
        if !r.as_ref().is_some_and(|v| v.dimensions == viewport_dims) {
            // Need a new internaldrawable
            let new_tex =
                Self::new_drawbuffer_size_format(device, viewport_dims, self.pixel_format, false);
            let mut replacement = Some(InternalDrawable::new(new_tex, viewport_dims));
            mem::swap(r, &mut replacement);
            drop(replacement);
        };
        // must be Some due to code above
        r.as_ref().unwrap()
    }
    pub(crate) fn new_drawbuffer_size_format(
        device: &ProtoObjRef<dyn MTLDevice>,
        size: (u32, u32),
        format: MTLPixelFormat,
        gpu_private: bool,
    ) -> ProtoObjRef<dyn MTLTexture> {
        gl_trace!(
            "creating new {}x{} {format:?} drawable texture",
            size.0,
            size.1
        );
        let desc = unsafe { MTLTextureDescriptor::new() };
        if gpu_private {
            desc.setAllowGPUOptimizedContents(true);
            desc.setStorageMode(MTLStorageMode::Private);
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
    pub(crate) fn linked_stage(
        state: &GLState,
        shader_type: ShaderType,
    ) -> &Option<LinkedShaderStage> {
        //TODO: program pipeline handling
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
            ShaderType::GeometryShader => &link.compute,
            ShaderType::TessEvaluationShader => todo!(),
            ShaderType::TessControlShader => todo!(),
            ShaderType::ComputeShader => todo!(),
        }
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
    fn stage_pinned_buffers(
        state: &GLState,
        shader_stage: &LinkedShaderStage,
    ) -> Vec<(ObjectName<Buffer>, u8)> {
        let mut v = Vec::new();
        for ssbo in &shader_stage.resources.shader_storage_buffers {
            let binding = ssbo
                .binding
                .expect("SSBO declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!("Shader requested SSBO at binding {binding} but no buffer was bound at that index");
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        for ubo in &shader_stage.resources.uniform_buffers {
            let binding = ubo
                .binding
                .expect("UBO declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!("Shader requested UBO at binding {binding} but no buffer was bound at that index");
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        for acb in &shader_stage.resources.atomic_counter_buffers {
            let binding = acb
                .binding
                .expect("atomic counter buffer declaration missing binding attribute");
            let Some(name) = state.buffer_bindings.shader_storage[binding as usize] else {
                panic!("Shader requested atomic counter buffer at binding {binding} but no buffer was bound at that index");
            };
            #[allow(clippy::cast_possible_truncation)]
            v.push((name, binding as u8));
        }
        //TODO XFBs
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
            let Some(attr) = attr else {
                continue;
            };
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
    //TODO: use onresized or something for updating drawable size instead of effectively polling
    #[inline]
    #[track_caller]
    pub(crate) fn current_drawable(&mut self) -> &ProtoObjRef<dyn CAMetalDrawable> {
        let r = self.drawable.get_or_insert_with(|| {
            let view = self
                .view
                .as_ref()
                .expect("Can't get metal drawable before attaching Context to a view");
            unsafe {
                self.layer
                    .setDrawableSize(view.convertSizeToBacking(view.frame().size));
            };
            unsafe { self.layer.nextDrawable() }
                .expect("Failed to get next drawable from CAMetalLayer")
        });
        &*r
    }
}
