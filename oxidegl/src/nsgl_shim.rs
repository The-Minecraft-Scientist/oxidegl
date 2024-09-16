use std::{
    cell::OnceCell,
    ffi::{c_void, CStr},
    hint::black_box,
    mem::{self, ManuallyDrop},
    ptr::{self, NonNull},
    sync::Once,
};

use crate::{
    context::{Context, CTX},
    entry_point::{box_ctx, oxidegl_platform_init, set_context, swap_buffers},
};
use core_foundation_sys::{
    base::CFEqual,
    bundle::{CFBundleGetFunctionPointerForName, CFBundleGetIdentifier, CFBundleRef},
    string::{kCFStringEncodingASCII, CFStringCreateWithCString, CFStringGetCString, CFStringRef},
};
use ctor::ctor;
use libc::{dlopen, dlsym, RTLD_LAZY};
use log::trace;
use objc2::{
    declare_class, extern_class,
    ffi::{class_replaceMethod, objc_class},
    msg_send_id, mutability,
    rc::{Allocated, Retained},
    runtime::{AnyClass, NSObject, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_app_kit::NSView;
use objc2_foundation::MainThreadMarker;

declare_class! {
    /// This is an objective C class whose methods shadow NSOpenGLContext's.
    /// When the nsgl_shim feature is enabled, a static initializer replaces the NSOpenGLContext's alloc implementation
    /// with an alloc implementation that allocates an OXGLOxideGlCtxShim instead (which essentially makes this class override NSOpenGLContext entirely)
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
            trace!("initialized context shim class");
            assert!(share.is_none(), "OxideGL does not support linked contexts!");
            let ctx_ptr = box_ctx(Context::new());
            let this = this.set_ivars(ctx_ptr);
            // Safety: superclass is NSObject and the init method exists on it
            unsafe {msg_send_id![super(this), init]}
        }
        #[method_id(initWithCGLPixelFormatObj:)]
        fn init_with_cgl_pf_obj(this: Allocated<Self>, _obj: *const c_void) -> Option<Retained<Self>> {
            trace!("initialized context shim class");
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
        #[method(setView:)]
        fn set_view(&self, view: Option<&NSView>) {
            dbg!(view);
            let ptr = *self.ivars();
            // take current context to avoid potential aliasing
            let ctx = CTX.take();
            if let Some(v) = view {
                // Safety: pointer is non null, points to an initialized and heap-allocated Context.
                // pointer cannot be aliased (since this class and CTX are the only places where the
                // pointer is actually read from, and we emptied CTX prior to creating this reference)
                unsafe {ptr.as_ref()}.set_view(&v.retain());
            }
            CTX.set(ctx);
        }

    }
}
unsafe extern "C" fn alloc_the_shim(_this: *const c_void, _cmd: Sel) -> *mut OXGLOxideGlCtxShim {
    // Wrap the Allocated RAII guard in a ManuallyDrop because we need our freshly allocated object
    // to outlive this scope (instead of being dealloced at the end of the scope and causing UB when the OBJC
    // runtime tries to use it later)
    let mut alloced = ManuallyDrop::new(
        MainThreadMarker::new()
            .unwrap()
            .alloc::<OXGLOxideGlCtxShim>(),
    );
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
        //TODO: entirely rawdog the objc runtime C API here instead of mixing safe and unsafe bindings
        static INSTALL_SWIZZLE_ONCE: Once = Once::new();
        INSTALL_SWIZZLE_ONCE.call_once(|| {
            trace!("installing Objective C method swizzle on [NSOpenGLContext alloc]");
            // Calling `class` ensures that the class actually gets loaded and registered before we try to do things with it
            let _this = OXGLOxideGlCtxShim::class();
            // alloc is a method on the class object so in order to set it we need to set a metaclass method
            let opengl_ctx_class = NSOpenGLContext::class().metaclass();

            // Safety: Caller ensures this method is only called from within
            // a static initializer
            unsafe {
                // Safety 1: the cast_mut on the objc_class pointer derived from the shared AnyClass reference
                // here is pretty sus and we probably shouldn't be using the safe API or references at all in the above code.
                // the mutable access is the primary reason that this function needs to be externally synchronized; We need to ensure that
                // our mutations to the class do not race with other threads potential usage of it

                // Safety 2: cls points to an loaded and initialized Objective C class object, the context this function is run from prevents races on this class
                // name points to a valid selector (created safely via the sel! macro)
                // imp points to a function with the platform C ABI and a signature matching the underlying implementation of [NSObject alloc]
                // types points to an Objective C method type encoding string that describes a method signature matching the implementation of [NSObject alloc]
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
                    // `@` - returns objc object pointer, `16`: 16 byte total parameter frame, `@0` - object pointer passed in
                    // at offset 0 in the parameter frame,`:8` - method selector passed in at offset 8 within the parameter frame (also cute face :3)
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
//Safety: Function pointers are safe to share between threads, DyldInterposeTuple cannot be constructed outside this module
unsafe impl Sync for DyldInterposeTuple {}

#[allow(non_snake_case)]
unsafe extern "C" fn CFBundleGetFunctionPointerForNameOverride(
    bundle: CFBundleRef,
    function_name: CFStringRef,
) -> *const c_void {
    thread_local! {
        static OXIDEGL_HANDLE: OnceCell<*mut c_void> = const { OnceCell::new() };
    }
    // Safety: eh it probably works
    unsafe {
        let bundle_name = CFBundleGetIdentifier(bundle);
        let comp_str = CFStringCreateWithCString(
            ptr::null(),
            c"com.apple.opengl".as_ptr(),
            kCFStringEncodingASCII,
        );
        if CFEqual(bundle_name.cast(), comp_str.cast()) == 1 {
            const BUF_SIZE: u32 = 1024;
            let mut buf = [0u8; BUF_SIZE as usize];
            //wrap only happens on 32 bit platforms, none of which implement Metal anyways
            #[allow(clippy::cast_possible_wrap)]
            if CFStringGetCString(
                function_name,
                ptr::from_mut(&mut buf).cast(),
                BUF_SIZE as isize,
                kCFStringEncodingASCII,
            ) == 0
            {
                panic!("Failed to get C String for NSString symbol name!");
            };
            let symbol =
                CStr::from_bytes_until_nul(&buf).expect("failed to create CStr from NSString");
            trace!(
                "Redirecting NSGL function lookup of {:?} to OxideGL",
                symbol
            );
            let handle = OXIDEGL_HANDLE
                .with(|v| *v.get_or_init(|| dlopen(c"liboxidegl.dylib".as_ptr(), RTLD_LAZY)));
            dlsym(handle, symbol.as_ptr())
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
    // Safety: we are living the good life (before main), so there are no other threads to race with on environment variables
    unsafe { oxidegl_platform_init() }
    // Need to actually use the static somewhere to keep the linker/rustc from stripping it from the binary, might as well put it here
    let _ = black_box(&DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE);
    // Safety: running from static ctor (equivalent to objc +load context)
    unsafe { OXGLOxideGlCtxShim::install_swizzle() }
}
