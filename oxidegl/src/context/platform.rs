use log::{debug, trace};
use objc2::rc::{Id, Retained};
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSScreen, NSView};
use objc2_foundation::MainThreadMarker;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};
use objc2_quartz_core::{kCAFilterNearest, CAMetalLayer};

#[derive(Debug)]
pub struct PlatformState {
    pub metal: MetalComponents,
}
#[derive(Debug, Clone)]
pub struct MetalComponents {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    layer: Retained<CAMetalLayer>,
}
#[allow(clippy::undocumented_unsafe_blocks)]
impl MetalComponents {
    pub(crate) fn new(view: &Id<NSView>) -> Self {
        let device = unsafe { Retained::retain(MTLCreateSystemDefaultDevice()) }.unwrap();

        let layer = unsafe { CAMetalLayer::new() };

        unsafe {
            //TODO: pixel format
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
        trace!("injected layer{:?} into window!", &layer);
        //SAFETY: GL must be called from the main thread on Apple platforms
        let cscale = NSScreen::mainScreen(unsafe { MainThreadMarker::new_unchecked() })
            .unwrap()
            .backingScaleFactor();
        layer.setContentsScale(cscale);

        unsafe { view.setLayer(Some(&layer)) };
        view.setWantsLayer(true);
        debug!("OxideGL got device: {}", device.name());
        Self { device, layer }
    }
}
