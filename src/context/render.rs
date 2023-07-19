use metal::{CAMetalLayer, Device, MTLDevice, MetalLayer};

use icrate::AppKit::NSResponder;
use icrate::CoreAnimation::CALayer;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, msg_send_id, mutability, ClassType};

extern_class!(
    pub struct NSView;

    unsafe impl ClassType for NSView {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl NSView {
        #[method(setWantsLayer:)]
        pub fn set_wants_layer(&self, wants_layer: bool);
        #[method(getLayer)]
        pub fn get_layer(&self) -> *mut CALayer;
        #[method(setLayer:)]
        pub fn set_layer(&self, layer: *mut CALayer);
    }
);

pub struct OxideGLMetalComponents {
    device: Device,
    layer: MetalLayer,
}
impl OxideGLMetalComponents {
    pub fn new(view: *mut NSView) {
        let device = Device::system_default().unwrap();
        unsafe {
            let mut layer = MetalLayer::new();
            view.as_ref().unwrap().set_wants_layer(true);
            (*view).set_layer(std::mem::transmute(layer.as_ref()))
        }
    }
}
