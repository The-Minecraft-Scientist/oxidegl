use log::{info, trace};
use objc2::rc::{Id, Retained};
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::{is_main_thread, ns_string, MainThreadMarker, NSString};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLCommandBuffer, MTLCommandBufferDescriptor,
    MTLCommandBufferErrorOption, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice,
    MTLDevice, MTLPixelFormat, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
};
use objc2_quartz_core::{kCAFilterNearest, CAMetalDrawable, CAMetalLayer};

use crate::ProtoObjRef;

#[derive(Debug, Clone)]
pub struct PlatformState {
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
}

#[allow(clippy::undocumented_unsafe_blocks)]
impl PlatformState {
    pub(crate) fn set_view(&self, view: &Id<NSView>) {
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
        let device = unsafe { Retained::retain(MTLCreateSystemDefaultDevice()) }.unwrap();

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
            render_encoder: None,
            blit_command_buffer: None,
            blit_encoder: None,
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
                .expect("failed to create command buffer")
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            buf = queue
                .commandBuffer()
                .expect("failed to create command buffer")
        }
        label.map(|v| buf.setLabel(Some(v)));
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
        label.map(|v| enc.setLabel(Some(v)));
        enc
    }
    pub(crate) fn current_render_encoder(&mut self) -> &ProtoObjRef<dyn MTLRenderCommandEncoder> {
        self.render_encoder
            .as_ref()
            .expect("render command encoder should have been created!")
    }
}
