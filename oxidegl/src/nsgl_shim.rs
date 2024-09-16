use std::{
    cell::OnceCell,
    ffi::{c_void, CStr},
    hint::black_box,
    mem::{self},
    ptr::{self, NonNull},
    sync::Once,
};

use crate::{
    context::Context,
    entry_point::{box_ctx, oxidegl_platform_init, set_context, swap_buffers},
};
use core_foundation_sys::{
    base::CFEqual,
    bundle::{CFBundleGetFunctionPointerForName, CFBundleGetIdentifier, CFBundleRef},
    string::{
        kCFStringEncodingASCII, CFStringCreateWithCString, CFStringGetCStringPtr, CFStringRef,
    },
};
use ctor::ctor;
use libc::{dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use objc2::{
    declare_class, extern_class,
    ffi::{class_replaceMethod, objc_class},
    msg_send_id, mutability,
    rc::{Allocated, Retained},
    runtime::{AnyClass, NSObject, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_foundation::MainThreadMarker;
//TODO: safety comments for this hot mess

declare_class! {
    /// This is an objective C class whose methods shadow NSOpenGLContext's.
    /// When the nsgl_shim feature is enabled, a static initializer load this class, which in turn
    struct OXGLOxideGlCtxShim;
    unsafe impl ClassType for OXGLOxideGlCtxShim {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "OXGLOxideGlCtxShim";
    }
    impl DeclaredClass for OXGLOxideGlCtxShim {
        type Ivars = NonNull<Context>;
    }
    unsafe impl OXGLOxideGlCtxShim {
        #[method_id(initWithFormat:shareContext:)]
        fn init_with_format_share_ctx(this: Allocated<Self>, _format: &AnyClass, share: Option<&Self>) -> Option<Retained<Self>> {
            unsafe {oxidegl_platform_init()}
            println!("fake context init");
            assert!(share.is_none(), "OxideGL does not support linked contexts!");
            let ctx_ptr = box_ctx(Context::new());
            let this = this.set_ivars(ctx_ptr);
            // Safety: superclass is NSObject and the init method exists on it
            unsafe {msg_send_id![super(this), init]}
        }
        #[method_id(initWithCGLPixelFormatObj:)]
        fn init_with_cgl_pf_obj(this: Allocated<Self>, _obj: *const c_void) -> Option<Retained<Self>> {
            unsafe {oxidegl_platform_init()}
            println!("fake context init");
            let ctx_ptr = box_ctx(Context::new());
            let this = this.set_ivars(ctx_ptr);
            // Safety: superclass is NSObject and the init method exists on it
            unsafe {msg_send_id![super(this), init]}
        }
        #[method(setValues:forParameter:)]
        fn set_values(&self, _values: *const i32, _parameter: isize) {}
        #[method(getValues:forParameter:)]
        unsafe fn get_values(&self, values: *mut i32, parameter: isize) {
            let outv = match parameter {
                // NSGLParamSwapInterval | NSGLParamSurfaceOrder
                222 | 235 => 1,
                _ => panic!("tried to get NSGL context parameters from oxidegl nsgl shim (param code {parameter})"),
            };
            //Safety: caller ensure pointer is valid
            unsafe {*values = outv};
        }
        #[method(makeCurrentContext)]
        fn make_current(&self) {
            set_context(Some(*self.ivars()));
        }
        #[method(clearCurrentContext)]
        fn clear_current() {
            set_context(None);
        }
        #[method(flushBuffer)]
        fn flush_buffer(&self) {
            swap_buffers();
        }

    }
}
unsafe extern "C" fn alloc_the_shim(_this: *const c_void, _cmd: Sel) -> *mut OXGLOxideGlCtxShim {
    let mut alloced = MainThreadMarker::new()
        .unwrap()
        .alloc::<OXGLOxideGlCtxShim>();

    Allocated::as_mut_ptr(&mut alloced)
}
const ALLOC_THE_SHIM_IMP: unsafe extern "C" fn(*const c_void, Sel) -> *mut OXGLOxideGlCtxShim =
    alloc_the_shim;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSOpenGLContext;

    #[rustfmt::skip]
    unsafe impl ClassType for NSOpenGLContext {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl OXGLOxideGlCtxShim {
    /// # Safety
    /// Must be called from a serial objc context (e.g. a static initializer or +load method)
    unsafe fn install_swizzle() {
        //TODO: entirely rawdog the objc runtime C API here instead of mixing the safe and unsafe bindings
        static INSTALL_SWIZZLE_ONCE: Once = Once::new();
        INSTALL_SWIZZLE_ONCE.call_once(|| {
            let _self_class = black_box(OXGLOxideGlCtxShim::class());
            // Calling `class` implicitly ensures that the class actually gets loaded before we try to modify it
            let opengl_ctx_class = NSOpenGLContext::class().metaclass();

            // Safety: we are Doing Crimes. any justification of safety in
            // this comment will likely be inadequate due to the objective c
            // runtime functions being objectively sketchy as hell to call.
            // However, the caller ensures this method is only called from within
            // a static initializer (which seems to be a common practice for "safe" method swizzling in objc)

            unsafe {
                // the cast_mut on the objc_class pointer derived from the shared AnyClass reference
                // here is **EXTREMELY** sus and we probably shouldn't be using the safe API and references at all in the above code
                // this is the primary reason that this function needs to be externally synchronized; We need to ensure that
                // our mutations to the class do not race with other threads potential usage of it
                class_replaceMethod(
                    ptr::from_ref(opengl_ctx_class)
                        .cast::<objc_class>()
                        .cast_mut(),
                    sel!(alloc).as_ptr(),
                    Some(mem::transmute::<
                        unsafe extern "C" fn(_, _) -> _,
                        unsafe extern "C" fn(),
                    >(ALLOC_THE_SHIM_IMP)),
                    // Magic string :)
                    c"@16@0:8".as_ptr(),
                );
            };
        });
    }
}

#[repr(C)]
struct DyldInterposeTuple {
    replacement: *const c_void,
    replacee: *const c_void,
}
unsafe impl Sync for DyldInterposeTuple {}

#[allow(non_snake_case)]
unsafe extern "C" fn CFBundleGetFunctionPointerForNameOverride(
    bundle: CFBundleRef,
    function_name: CFStringRef,
) -> *const c_void {
    thread_local! {

        static OXIDEGL_HANDLE: OnceCell<*mut c_void> = const { OnceCell::new() };
    }
    println!("get functionpointerforname override");
    // Safety: Uh :3
    unsafe {
        let bundle_name = CFBundleGetIdentifier(bundle);
        let comp_str = CFStringCreateWithCString(
            ptr::null(),
            c"com.apple.opengl".as_ptr(),
            kCFStringEncodingASCII,
        );
        let symbol = CFStringGetCStringPtr(function_name, kCFStringEncodingASCII);
        dbg!(CStr::from_ptr(symbol));
        if CFEqual(bundle_name.cast(), comp_str.cast()) == 1 {
            let handle = OXIDEGL_HANDLE
                .with(|v| *v.get_or_init(|| dlopen(c"liboxidegl.dylib".as_ptr(), RTLD_LAZY)));
            dlsym(handle, symbol)
        } else {
            CFBundleGetFunctionPointerForName(bundle, function_name)
        }
    }
}
// ...
// I love linker magic
#[link_section = "__DATA,__interpose"]
#[allow(private_interfaces)]
pub static DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE: DyldInterposeTuple =
    DyldInterposeTuple {
        replacement: CFBundleGetFunctionPointerForNameOverride as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
        replacee: CFBundleGetFunctionPointerForName as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
    };

#[ctor]
fn ctor() {
    // Need to use the static to keep the linker/rustc from stripping it
    let _ = black_box(&DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE);
    unsafe { OXGLOxideGlCtxShim::install_swizzle() }
}
