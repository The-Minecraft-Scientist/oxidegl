use ahash::{HashMap, HashMapExt};
use log::{info, trace};
use objc2::rc::{Id, Retained};
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::{is_main_thread, ns_string, MainThreadMarker, NSString};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLCommandBuffer, MTLCommandBufferDescriptor,
    MTLCommandBufferErrorOption, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice,
    MTLDevice, MTLPixelFormat, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLVertexAttributeDescriptor,
    MTLVertexDescriptor,
};
use objc2_quartz_core::{kCAFilterNearest, CAMetalDrawable, CAMetalLayer};

use crate::ProtoObjRef;

use super::{
    commands::buffer::Buffer,
    program::LinkedProgram,
    state::{GLState, ObjectName},
};

#[derive(Debug, Clone)]
pub struct PlatformState {
    /// dirty components
    dirty_state: NeedsRefreshBits,
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
    /// we need to realize private to shared buffer copies which
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
    pub(crate) vertex_buffer_map: HashMap<ObjectName<Buffer>, u32>,

    /// Mapping from buffer name to metal fragment shader argument index
    pub(crate) fragment_buffer_map: HashMap<ObjectName<Buffer>, u32>,
}
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

#[allow(clippy::undocumented_unsafe_blocks)]
impl PlatformState {
    pub(crate) fn set_view(&self, view: &Retained<NSView>) {
        self.layer.setFrame(view.frame());
        if let Some(l) = unsafe { view.layer() } {
            l.addSublayer(&self.layer);
        } else {
            unsafe { view.setLayer(Some(&self.layer)) };
        }
        view.setWantsLayer(true);
        trace!("injected layer {:?} into NSView", &self.layer);
    }
    pub(crate) fn new(drawable_pixel_format: MTLPixelFormat) -> Self {
        let device = unsafe { Retained::from_raw(MTLCreateSystemDefaultDevice()) }.unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            layer.setPixelFormat(drawable_pixel_format);
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
            device,
            queue,
            layer,
            drawable: None,
            command_buffer: None,
            blit_command_buffer: None,
            blit_encoder: None,

            render_encoder: None,
            render_pipeline_state: None,

            vertex_buffer_map: HashMap::new(),
            fragment_buffer_map: HashMap::new(),
            dirty_state: NeedsRefreshBits::empty(),
        }
    }
    #[inline]
    fn new_command_buffer(
        queue: &ProtoObjRef<dyn MTLCommandQueue>,
        label: Option<&'static NSString>,
    ) -> ProtoObjRef<dyn MTLCommandBuffer> {
        let buf;
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
    pub(crate) fn build_render_pipeline_state(
        &mut self,
        state: &mut GLState,
    ) -> ProtoObjRef<dyn MTLRenderPipelineState> {
        let desc = MTLRenderPipelineDescriptor::new();
        todo!();
    }
    /// precondition: none
    pub(crate) fn remap_buffer_arguments(&mut self, state: &mut GLState) {
        let program = state
            .program_list
            .get(state.program_binding.expect("No program bound"));
        let Some(LinkedProgram {
            fragment_entry,
            vertex_entry,
            compute_entry,
        }) = &program.latest_linkage
        else {
            panic!("Program should have been linked");
        };
    }
    /// precondition: Buffer maps built
    pub(crate) fn build_vertex_descriptor(
        &mut self,
        state: &mut GLState,
    ) -> Retained<MTLVertexDescriptor> {
        let vao = state.vao_list.get(
            state
                .vao_binding
                .expect("No VAO bound when trying to build Metal vertex descriptor!"),
        );
        let mtl_vertex_desc = unsafe { MTLVertexDescriptor::new() };
        for (idx, attr) in vao.attribs.iter().enumerate() {
            let Some(attr) = attr else {
                continue;
            };
            let attr_binding = vao.buffer_bindings[attr.buffer_idx as usize];

            let mtl_attrib_desc = MTLVertexAttributeDescriptor::new();

            // Get the index of the corresponding buffer object in the vertex shader argument table
            let buffer_argument_index = *self
                .vertex_buffer_map
                .get(&attr_binding.buf.expect("Buffer for attribute not bound"))
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
    pub(crate) fn current_render_encoder(&mut self) -> &ProtoObjRef<dyn MTLRenderCommandEncoder> {
        self.render_encoder
            .as_ref()
            .expect("render command encoder should have been created!")
    }
}
