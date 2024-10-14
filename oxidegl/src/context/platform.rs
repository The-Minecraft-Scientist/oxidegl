use std::mem;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use log::{info, trace};
use objc2::rc::Retained;
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::{is_main_thread, ns_string, MainThreadMarker, NSString};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLCommandBuffer, MTLCommandBufferDescriptor,
    MTLCommandBufferErrorOption, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice,
    MTLDevice, MTLPixelFormat, MTLPrimitiveTopologyClass, MTLRenderCommandEncoder,
    MTLRenderPassColorAttachmentDescriptor, MTLRenderPassDepthAttachmentDescriptor,
    MTLRenderPassDescriptor, MTLRenderPassStencilAttachmentDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureUsage,
    MTLVertexAttributeDescriptor, MTLVertexDescriptor,
};
use objc2_quartz_core::{kCAFilterNearest, CAMetalDrawable, CAMetalLayer};

use crate::{
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
    dirty_state: NeedsRefreshBits,

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
const MTL_MAX_ARGUMENT_BINDINGS: usize = 32;
bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NeedsRefreshBits: u32 {
        const RENDER_PASS =     1;
        const RENDER_PIPELINE = 1 << 1;
        const VAO =             1 << 2;
        const PROGRAM =         1 << 3;

    }
}
/// Utility that maps currently active object names to their location in the relevant Metal shader parameter table
#[derive(Debug)]
pub(crate) struct ResourceMap<T, const MAX_ENTRIES: usize = 32> {
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
        assert!(pinned_resources.len() + include_resources.len() <= MAX_ENTRIES);
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

        if let Some(l) = unsafe { view.layer() } {
            l.addSublayer(&self.layer);
        } else {
            unsafe { view.setLayer(Some(&self.layer)) };
        }
        view.setWantsLayer(true);
        trace!("injected layer {:?} into NSView", &self.layer);
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
            layer.setMagnificationFilter(kCAFilterNearest);
        };

        debug_assert!(is_main_thread());
        // Safety: GL must be called from the main thread on macOS
        let cscale = NSScreen::mainScreen(unsafe { MainThreadMarker::new_unchecked() })
            .unwrap()
            .backingScaleFactor();
        layer.setContentsScale(cscale);

        info!("Metal device: {}", device.name());
        let queue = device
            .newCommandQueue()
            .expect("failed to create command queue");
        queue.setLabel(Some(ns_string!("OxideGL command queue")));

        Self {
            dirty_state: NeedsRefreshBits::empty(),

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
    #[inline]
    fn new_render_encoder(
        buf: &ProtoObjRef<dyn MTLCommandBuffer>,
        desc: &MTLRenderPassDescriptor,
        label: Option<&'static NSString>,
    ) -> ProtoObjRef<dyn MTLRenderCommandEncoder> {
        let enc = buf
            .renderCommandEncoderWithDescriptor(desc)
            .expect("failed to create render command encoder");
        if let Some(v) = label {
            enc.setLabel(Some(v));
        }
        enc
    }
    pub(crate) fn update_state(&mut self, state: &mut GLState) {}
    //preconditions: buffer maps built, renderable program present
    pub(crate) fn build_render_pipeline_state(
        &mut self,
        state: &mut GLState,
    ) -> ProtoObjRef<dyn MTLRenderPipelineState> {
        let desc = MTLRenderPipelineDescriptor::new();
        let (Some(f), Some(v)) = (
            Self::linked_stage(state, ShaderType::FragmentShader),
            Self::linked_stage(state, ShaderType::VertexShader),
        ) else {
            panic!("Tried to build a render pipeline while missing a linked vertex or fragment shader stage");
        };
        desc.setVertexFunction(Some(&v.function));
        desc.setFragmentFunction(Some(&f.function));
        //TODO: primitive topology real
        unsafe { desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Triangle) };
        self.device
            .newRenderPipelineStateWithDescriptor_error(&desc)
            .expect("failed to create pipeline state")
    }
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "we hope this works"
    )]
    //preconditions: view set on context
    pub(crate) fn build_render_pass_descriptor(
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
            let &(_, first) = iter.peek().expect("No draw buffer set");
            let rect = &self
                .view
                .as_ref()
                .expect("GL command called before setting the view for this context")
                .frame();
            let dims = (rect.size.width as u32, rect.size.width as u32);
            let mut drawable = self.get_internal_drawbuffer(first, dims).clone();

            if let Some(depth) = &drawable.depth {
                let a_desc = unsafe { MTLRenderPassDepthAttachmentDescriptor::new() };
                a_desc.setTexture(Some(depth));
                desc.setDepthAttachment(Some(&a_desc));
            }
            if let Some(stencil) = &drawable.stencil {
                let a_desc = unsafe { MTLRenderPassStencilAttachmentDescriptor::new() };
                a_desc.setTexture(Some(stencil));
                desc.setStencilAttachment(Some(&a_desc));
            }

            for (idx, buf) in iter {
                if buf == DrawBufferMode::FrontLeft {
                    drawable.with_ca_metal_drawable(self.current_drawable());
                }
                let a_desc = MTLRenderPassColorAttachmentDescriptor::new();
                a_desc.setTexture(Some(&drawable.color));
                unsafe {
                    desc.colorAttachments()
                        .setObject_atIndexedSubscript(Some(&a_desc), idx);
                };
            }
            desc.setRenderTargetWidth(dims.0 as usize);
            desc.setRenderTargetHeight(dims.1 as usize);
        }
        todo!();
    }
    pub(crate) fn get_internal_drawbuffer(
        &mut self,
        target: DrawBufferMode,
        viewport_dims: (u32, u32),
    ) -> &InternalDrawable {
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
            let vertex_buffers = vao
                .buffer_bindings
                .iter()
                .filter_map(|b| b.buf)
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
        let vao = state.vao_list.get(state.vao_binding.unwrap());
        let mtl_vertex_desc = unsafe { MTLVertexDescriptor::new() };
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
            unsafe { mtl_attrib_desc.setOffset(attr_binding.offset) };

            unsafe {
                mtl_vertex_desc
                    .attributes()
                    .setObject_atIndexedSubscript(Some(&mtl_attrib_desc), idx);
            };
        }
        mtl_vertex_desc
    }
    #[inline]
    pub(crate) fn current_render_encoder(&mut self) -> &ProtoObjRef<dyn MTLRenderCommandEncoder> {
        self.render_encoder
            .as_ref()
            .expect("render command encoder should have been created!")
    }
    #[inline]
    pub(crate) fn current_drawable(&mut self) -> &mut ProtoObjRef<dyn CAMetalDrawable> {
        self.drawable.get_or_insert_with(|| {
            unsafe { self.layer.nextDrawable() }
                .expect("Failed to get next drawable from CAMetalLayer")
        })
    }
}
