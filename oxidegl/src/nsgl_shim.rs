use std::{
    cell::OnceCell,
    ffi::{c_void, CStr},
    hint::black_box,
    mem::{self, ManuallyDrop, MaybeUninit},
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
    ffi::{
        class_addIvar, class_addProperty, class_getClassMethod, class_replaceMethod,
        class_setIvarLayout, method_getTypeEncoding, objc_alloc, objc_class,
        objc_getAssociatedObject, objc_object, objc_property, objc_setAssociatedObject,
        object_setClass, OBJC_ASSOCIATION_COPY, OBJC_ASSOCIATION_RETAIN,
        OBJC_ASSOCIATION_RETAIN_NONATOMIC,
    },
    msg_send_id, mutability,
    rc::{Allocated, Retained},
    runtime::{AnyClass, ClassBuilder, NSObject, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_app_kit::NSView;
declare_class!(
    struct OXGLOxideGLCtxAssociatedObject;
    unsafe impl ClassType for OXGLOxideGLCtxAssociatedObject {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "OXGLOxideGLCtxAssociatedObject";
    }
    impl DeclaredClass for OXGLOxideGLCtxAssociatedObject {
        type Ivars = NonNull<c_void>;
    }
    unsafe impl OXGLOxideGLCtxAssociatedObject {
        #[method_id(initWithCtx:)]
        fn init_with_ctx(this: Allocated<Self>, ctx: NonNull<c_void>) -> Option<Retained<Self>> {
            let this = this.set_ivars(ctx);
            // Safety: superclass is NSObject and the init method exists on it
            unsafe {msg_send_id![super(this), init]}
        }
        #[method(getCtx)]
        fn get_ctx(&self) -> NonNull<c_void> {
            *self.ivars()
        }
    }
);
impl OXGLOxideGLCtxAssociatedObject {
    fn new_with_ctx(ctx: NonNull<Context>) -> Retained<OXGLOxideGLCtxAssociatedObject> {
        let alloc = OXGLOxideGLCtxAssociatedObject::alloc();
        unsafe { msg_send_id![alloc, initWithCtx:ctx.as_ptr().cast::<c_void>()] }
    }
}

declare_class! {
    /// This is an objective C class whose methods shadow NSOpenGLContext's.
    /// When the nsgl_shim feature is enabled, a static initializer replaces the NSOpenGLContext's alloc implementation
    /// with an alloc implementation that allocates an OXGLOxideGlCtxShim instead (which essentially makes this class override NSOpenGLContext entirely)
    struct OXGLOxideGlCtxShim;
    unsafe impl ClassType for OXGLOxideGlCtxShim {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "OXGLOxideGlCtxShim";
    }
    impl DeclaredClass for OXGLOxideGlCtxShim {
    }
    unsafe impl OXGLOxideGlCtxShim {
        #[method_id(initWithFormat:shareContext:)]
        fn init_with_format_share_ctx(this: Allocated<Self>, _format: &AnyClass, share: Option<&Self>) -> Option<Retained<Self>> {
            trace!("initialized OBJC context shim");

            assert!(share.is_none(), "OxideGL does not support linked contexts!");

            let this = this.set_ivars(());
            // Safety: superclass is NSObject and the init method exists on it
            let this: Option<Retained<Self>> = unsafe {msg_send_id![super(this), init]};
            let this = this.unwrap();
            let ctx_ptr = box_ctx(Context::new());
            unsafe {this.set_assoc_obj(OXGLOxideGLCtxAssociatedObject::new_with_ctx(ctx_ptr.cast())) };
            Some(this)

        }
        #[method_id(initWithCGLPixelFormatObj:)]
        fn init_with_cgl_pf_obj(this: Allocated<Self>, _obj: *const c_void) -> Option<Retained<Self>> {
            trace!("initialized OBJC context shim");
            let this = this.set_ivars(());
            // Safety: superclass is NSObject and the init method exists on it
            let this: Option<Retained<Self>> = unsafe {msg_send_id![super(this), init]};
            let this = this.unwrap();
            let ctx_ptr = box_ctx(Context::new());
            unsafe {this.set_assoc_obj(OXGLOxideGLCtxAssociatedObject::new_with_ctx(ctx_ptr.cast())) };
            Some(this)
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
            let obj = unsafe {self.get_assoc_obj()};
            set_context(Some(obj.ivars().cast()));
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
            let obj = unsafe {self.get_assoc_obj()};
            let ptr = obj.ivars().cast::<Context>();
            // take current context to avoid potential aliasing references
            let ctx = CTX.take();
            if let Some(v) = view {
                // Safety: pointer is non null, points to an initialized and heap-allocated Context.
                // pointer cannot have aliasing Rust references (since this class and CTX are the only places where the
                // pointer is actually read from, and we emptied CTX prior to creating this reference)
                unsafe {ptr.as_ref()}.set_view(&v.retain());
            }
            CTX.set(ctx);
        }

    }
}
// unsafe extern "C" fn alloc_the_shim(this: &AnyClass, _cmd: Sel) -> *mut objc_object {
//     unsafe { objc_alloc(ptr::from_ref(this).cast()) }
// }

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
    unsafe fn set_assoc_obj(&self, mut obj: Retained<OXGLOxideGLCtxAssociatedObject>) {
        unsafe {
            objc_setAssociatedObject(
                ptr::from_ref(self).cast_mut().cast(),
                sel!(oxglAssocObj).as_ptr().cast(),
                Retained::as_mut_ptr(&mut obj).cast(),
                OBJC_ASSOCIATION_RETAIN,
            );
        };
    }
    unsafe fn get_assoc_obj(&self) -> &OXGLOxideGLCtxAssociatedObject {
        let ptr = unsafe {
            objc_getAssociatedObject(
                ptr::from_ref(self).cast_mut().cast(),
                sel!(oxglAssocObj).as_ptr().cast(),
            )
        };
        unsafe {
            ptr.cast_mut()
                .cast::<OXGLOxideGLCtxAssociatedObject>()
                .as_ref()
        }
        .expect("OxideGL context assoc objc not set!")
    }
    /// # Safety
    /// Must be called from a serial objc context (e.g. a static initializer or +load method)
    #[allow(clippy::too_many_lines)]
    unsafe fn butcher_ns_opengl() {
        static BUTCHER_ONCE: Once = Once::new();
        BUTCHER_ONCE.call_once(|| {
            trace!("Clobbering NSOpenGLContext");
            // Calling `class` ensures that the class actually gets loaded and registered before we try to do things with it
            let opengl_ctx_class_ptr = ptr::from_ref(NSOpenGLContext::class())
                .cast_mut()
                .cast::<objc_class>();
            let opengl_ctx_class_metaclass_ptr =
                ptr::from_ref(NSOpenGLContext::class().metaclass())
                    .cast_mut()
                    .cast::<objc_class>();
            // Safety: Caller ensures this method is only called from within
            // a static initializer
            #[allow(clippy::missing_transmute_annotations)]
            // Replace all of the relevant methods on
            unsafe {
                let sel_ptr = sel!(initWithFormat:shareContext:).as_ptr();

                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::init_with_format_share_ctx as unsafe extern "C" fn(_, _, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(initWithCGLPixelFormatObj:).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::init_with_cgl_pf_obj as unsafe extern "C" fn(_, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(initWithCGLPixelFormatObj:).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::init_with_cgl_pf_obj as unsafe extern "C" fn(_, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );

                let sel_ptr = sel!(setValues:forParameter:).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::set_values as unsafe extern "C" fn(_, _, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(getValues:forParameter:).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::get_values as unsafe extern "C" fn(_, _, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(makeCurrentContext).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::make_current as unsafe extern "C" fn(_, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr: *const objc2::ffi::objc_selector = sel!(flushBuffer).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::flush_buffer as unsafe extern "C" fn(_, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr: *const objc2::ffi::objc_selector = sel!(setView:).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::set_view as unsafe extern "C" fn(_, _, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr: *const objc2::ffi::objc_selector = sel!(clearCurrentContext).as_ptr();
                class_replaceMethod(
                    opengl_ctx_class_metaclass_ptr,
                    sel_ptr,
                    Some(mem::transmute(
                        Self::clear_current as unsafe extern "C" fn(_, _) -> _,
                    )),
                    method_getTypeEncoding(class_getClassMethod(
                        opengl_ctx_class_metaclass_ptr,
                        sel_ptr,
                    )),
                );
            };
        });
    }
}

thread_local! {
    static OXIDEGL_HANDLE: OnceCell<*mut c_void> = const { OnceCell::new() };
}
unsafe fn get_oxidegl_handle() -> *mut c_void {
    OXIDEGL_HANDLE
        .with(|v| *v.get_or_init(|| unsafe { dlopen(c"liboxidegl.dylib".as_ptr(), RTLD_LAZY) }))
}
unsafe extern "C" fn dlopen_override(filename: *const i8, flags: i32) -> *mut c_void {
    let str = unsafe { CStr::from_ptr(filename) };
    if str.to_str().is_ok_and(|s| s.contains("libGL.dylib")) {
        trace!("substituted OxideGL dlopen handle for libGL.dylib");
        unsafe { get_oxidegl_handle() }
    } else {
        unsafe { dlopen(filename, flags) }
    }
}
#[allow(non_snake_case)]
/// This function overrides the default implementation of `CFBundleGetFunctionPointerForName`, which is used
/// by consumers of NSGL to look up all of the openGL command functions. When called, it checks if the bundle name for
/// the function being looked up is "com.apple.opengl", and if so, redirects the lookup to a dlsym on the `OxideGL` dylib.
unsafe extern "C" fn CFBundleGetFunctionPointerForNameOverride(
    bundle: CFBundleRef,
    function_name: CFStringRef,
) -> *const c_void {
    trace!("CFBundleLookup called");
    // Safety: eh it probably works
    unsafe {
        let bundle_name = CFBundleGetIdentifier(bundle);
        let comp_str = CFStringCreateWithCString(
            ptr::null(),
            c"com.apple.opengl".as_ptr(),
            kCFStringEncodingASCII,
        );
        // Detect opengl bundle name
        if CFEqual(bundle_name.cast(), comp_str.cast()) == 1 {
            //Redirect function lookup for openGL functions
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

#[repr(C)]
struct DyldInterposeTuple {
    replacement: *const c_void,
    replacee: *const c_void,
}
// Safety: Function pointers are safe to share between threads, DyldInterposeTuple cannot be constructed outside this module
// This is only read (by dyld during life-before-main) from the main thread anyways,
// rustc requires Sync for all statics and this keeps it from complaining
unsafe impl Sync for DyldInterposeTuple {}

// ...
// I love linker magic
#[link_section = "__DATA,__interpose"]
#[allow(private_interfaces)]
/// Installs a pointer to the above override function into a special section in the binary that tells dyld to actually do the override
static DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE: DyldInterposeTuple =
    DyldInterposeTuple {
        replacement: CFBundleGetFunctionPointerForNameOverride as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
        replacee: CFBundleGetFunctionPointerForName as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
    };
#[link_section = "__DATA,__interpose"]
#[allow(private_interfaces)]
static DYLD_LIBC_DLOPEN_INTERPOSE: DyldInterposeTuple = DyldInterposeTuple {
    replacement: dlopen_override as unsafe extern "C" fn(_, _) -> _ as *const c_void,
    replacee: dlopen as unsafe extern "C" fn(_, _) -> _ as *const c_void,
};

#[ctor]
fn ctor() {
    // Safety: we are living the good life (before main), so there are no other threads to race on environment variables with
    unsafe { oxidegl_platform_init() }
    // Need to actually use the static somewhere to keep the linker/rustc from stripping it from the binary, might as well put it here
    let _ = black_box(&DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE);
    let _ = black_box(&DYLD_LIBC_DLOPEN_INTERPOSE);
    // Safety: running from static ctor (equivalent to objc +load context)
    unsafe { OXGLOxideGlCtxShim::butcher_ns_opengl() }
}
