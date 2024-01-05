use std::mem;

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

#[derive(Debug, Clone)]
pub struct ContextMetalComponents {
    device: Device,
}
impl ContextMetalComponents {
    //SAFETY: this is wildly unsafe. However, it seems to work
    pub(crate) fn new(mut view: NSViewPtr) -> Self {
        let device = Device::system_default().unwrap();
        let mut layer = MetalLayer::new();
        layer.set_device(&device);
        // The amount of invariants that need to hold for this cast to be valid cannot be counted on two hands
        let cast_view: Id<NSView> = unsafe { std::mem::transmute(view) };
        let cast_layer = unsafe {
            //Unspeakable horrors beyond mortal comprehension
            Id::new(core::mem::transmute::<*mut MetalLayerRef, *mut CALayer>(
                layer.as_mut(),
            ))
            .unwrap()
        };

        mem::forget(layer);
        //This function call is actually surprisingly safe
        unsafe { cast_view.set_layer(&cast_layer) };
        Self { device }
    }
}
