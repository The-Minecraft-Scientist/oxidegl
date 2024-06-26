use std::mem::{self, ManuallyDrop};

use libc::c_void;
use metal::{CAMetalLayer, Device, MTLDevice, MetalLayer, MetalLayerRef};

use icrate::AppKit::{NSCollectionView, NSResponder, NSView as IcNSView};
use icrate::CoreAnimation::CALayer;
use objc2::rc::Id;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, msg_send_id, mutability, ClassType};

use super::NSViewPtr;

extern_class!(
    #[derive(Debug)]
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
        pub unsafe fn set_wants_layer(&self, wants_layer: bool);
        #[method_id(@__retain_semantics Other getLayer)]
        pub unsafe fn get_layer(&self) -> Id<CALayer>;
        #[method(setLayer:)]
        pub unsafe fn set_layer(&self, layer: &CALayer);
    }
);
#[derive(Debug)]
pub struct PlatformState {
    pub metal: MetalComponents
}
#[derive(Debug, Clone)]
pub struct MetalComponents {
    device: Device,
}
impl MetalComponents {
    //TODO: hold a reference to the NSView
    pub(crate) unsafe fn new(mut view: NSViewPtr) -> Self {
        let device = Device::system_default().unwrap();
        // Avoid decrementing the refcount to this CALayer when MetalLayer is dropped
        let mut layer = ManuallyDrop::new(MetalLayer::new());

        layer.set_device(&device);
        // Caller must ensure that this is a pointer to an objective C NSView class with a retain count of 1
        let cast_view: Id<NSView> = unsafe { std::mem::transmute(view) };
        let cast_layer = 
            //Unspeakable horrors beyond mortal comprehension (this is essentially just casting one *mut c_void to another)
            Id::new(core::mem::transmute::<*mut MetalLayerRef, *mut CALayer>(
                layer.as_mut(),
            ))
            .unwrap();
        //This function call is actually surprisingly safe compared to the rest
        cast_view.set_layer(&cast_layer);
        Self { device }
    }
}
