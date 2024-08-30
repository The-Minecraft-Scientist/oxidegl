use log::{info, trace};
use objc2::rc::{Id, Retained};
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::MainThreadMarker;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice, MTLPixelFormat};
use objc2_quartz_core::{kCAFilterNearest, CAMetalLayer};

#[derive(Debug, Clone)]
pub struct PlatformState {
    pub(crate) device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub(crate) layer: Retained<CAMetalLayer>,
}
#[allow(clippy::undocumented_unsafe_blocks)]
impl PlatformState {
    pub(crate) fn new(view: &Id<NSView>) -> Self {
        let device = unsafe { Retained::retain(MTLCreateSystemDefaultDevice()) }.unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            //TODO: pixel format real
            layer.setPixelFormat(MTLPixelFormat::R8Unorm_sRGB);
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
        trace!("injected layer {:?} into NSWindow", &layer);
        // Safety: GL must be called from the main thread on Apple platforms
        let cscale = NSScreen::mainScreen(unsafe { MainThreadMarker::new_unchecked() })
            .unwrap()
            .backingScaleFactor();
        layer.setContentsScale(cscale);

        view.setWantsLayer(true);
        info!("Metal device: {}", device.name());
        Self { device, layer }
    }
}
