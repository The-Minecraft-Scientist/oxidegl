use log::{info, trace};
use objc2::rc::{Id, Retained};
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::MainThreadMarker;
use objc2_metal::{
    MTLCommandBuffer, MTLCreateSystemDefaultDevice, MTLDevice, MTLPixelFormat,
    MTLRenderCommandEncoder,
};
use objc2_quartz_core::{kCAFilterNearest, CAMetalDrawable, CAMetalLayer};

use crate::RetainedObject;

#[derive(Debug, Clone)]
pub struct PlatformState {
    pub(crate) device: RetainedObject<dyn MTLDevice>,
    pub(crate) layer: Retained<CAMetalLayer>,
    pub(crate) drawable: Option<RetainedObject<dyn CAMetalDrawable>>,
    pub(crate) command_buffer: Option<RetainedObject<dyn MTLCommandBuffer>>,
    pub(crate) render_encoder: Option<RetainedObject<dyn MTLRenderCommandEncoder>>,
}

#[allow(clippy::undocumented_unsafe_blocks)]
impl PlatformState {
    pub(crate) fn new(view: &Id<NSView>, drawable_pixel_format: MTLPixelFormat) -> Self {
        let device = unsafe { Retained::retain(MTLCreateSystemDefaultDevice()) }.unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            layer.setPixelFormat(drawable_pixel_format);
            layer.setDevice(Some(&device));
            layer.setFramebufferOnly(false);
            layer.setFrame(view.frame());
            layer.setMagnificationFilter(kCAFilterNearest);
        };

        if let Some(l) = unsafe { view.layer() } {
            l.addSublayer(&layer);
        } else {
            unsafe { view.setLayer(Some(&layer)) };
        }
        trace!("injected layer {:?} into NSView", &layer);
        // Safety: GL must be called from the main thread on Apple platforms
        let cscale = NSScreen::mainScreen(unsafe { MainThreadMarker::new_unchecked() })
            .unwrap()
            .backingScaleFactor();
        layer.setContentsScale(cscale);

        view.setWantsLayer(true);
        info!("Metal device: {}", device.name());
        Self {
            device,
            layer,
            drawable: None,
            command_buffer: None,
            render_encoder: None,
        }
    }
}
