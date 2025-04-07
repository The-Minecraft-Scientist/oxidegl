use crate::{
    context::{CTX, Context},
    entry_point::{box_ctx, set_context, swap_buffers},
};
use core_foundation_sys::{
    base::CFEqual,
    bundle::{CFBundleGetFunctionPointerForName, CFBundleGetIdentifier, CFBundleRef},
    string::{CFStringCreateWithCString, CFStringGetCString, CFStringRef, kCFStringEncodingASCII},
};
use libc::{RTLD_LAZY, dlopen, dlsym};
use log::trace;
use objc2::{
    AllocAnyThread, ClassType, DeclaredClass, define_class, extern_class,
    ffi::{
        OBJC_ASSOCIATION_RETAIN, class_getClassMethod, class_replaceMethod, method_getTypeEncoding,
        objc_getAssociatedObject, objc_setAssociatedObject,
    },
    msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, NSObject, Sel},
    sel,
};
use objc2_foundation::NSObjectProtocol;
use std::{
    cell::OnceCell,
    ffi::{CStr, c_void},
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
    sync::Once,
};

// Sometimes when I'm bored I click onto this file and read what I've wrote. It reads like an abyss
// and sometimes, it feels like the abyss is staring back, questioning my sanity for writing this.

use objc2_app_kit::NSView;
define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = NonNull<c_void>]
    #[name = "OXGLOxideGLCtxAssociatedObject"]
    struct OXGLOxideGLCtxAssociatedObject;

    impl OXGLOxideGLCtxAssociatedObject {
        #[unsafe(method(getCtx))]
        fn get_ctx(&self) -> NonNull<c_void> {
            *self.ivars()
        }
    }
);
impl OXGLOxideGLCtxAssociatedObject {
    fn new_with_ctx(ctx: NonNull<Context>) -> Option<Retained<OXGLOxideGLCtxAssociatedObject>> {
        let this = Self::alloc().set_ivars(ctx.cast());

        // Safety: init method exists on super
        unsafe { msg_send![super(this), init] }
    }
}

struct OXGLOxideGlCtxShim {
    _inhabited: MaybeUninit<()>,
}
impl OXGLOxideGlCtxShim {
    unsafe extern "C-unwind" fn init_with_format_share_ctx(
        this: *mut NSOpenGLContext,
        _sel: Sel,
        _format: *const AnyClass,
        share: Option<NonNull<NSOpenGLContext>>,
    ) -> objc2::rc::Retained<objc2_foundation::NSObject> {
        trace!("initialized OBJC context shim");

        assert!(share.is_none(), "OxideGL does not support linked contexts!");

        // Safety: superclass is NSObject and the init method exists on it
        // Safety: Allocated<T> is guaranteed to have the same layout/ABI as *mut T
        let this = NSObject::init(unsafe {
            mem::transmute::<*mut NSOpenGLContext, objc2::rc::Allocated<objc2_foundation::NSObject>>(
                this,
            )
        });
        let ctx_ptr = box_ctx(Context::new());
        Self::set_assoc_obj(
            Retained::as_ptr(&this).cast_mut().cast(),
            OXGLOxideGLCtxAssociatedObject::new_with_ctx(ctx_ptr.cast())
                .expect("failed to create associated object for context storage"),
        );
        this
    }
    unsafe extern "C-unwind" fn init_with_cgl_pf_obj(
        this: *mut NSOpenGLContext,
        _sel: Sel,
        _obj: *const c_void,
    ) -> objc2::rc::Retained<objc2_foundation::NSObject> {
        trace!("initialized OBJC context shim");

        // Safety: superclass is NSObject and the init method exists on it
        // Safety: Allocated<T> is guaranteed to have the same layout/ABI as *mut T
        let this = NSObject::init(unsafe {
            mem::transmute::<*mut NSOpenGLContext, objc2::rc::Allocated<objc2_foundation::NSObject>>(
                this,
            )
        });
        let ctx_ptr = box_ctx(Context::new());
        Self::set_assoc_obj(
            Retained::as_ptr(&this).cast_mut().cast(),
            OXGLOxideGLCtxAssociatedObject::new_with_ctx(ctx_ptr.cast())
                .expect("failed to create associated object for context storage"),
        );
        this
    }
    unsafe extern "C-unwind" fn set_values(
        _this: *mut NSOpenGLContext,
        _sel: Sel,
        _values: *const i32,
        _parameter: isize,
    ) {
    }
    unsafe extern "C-unwind" fn get_values(
        _this: *mut NSOpenGLContext,
        _sel: Sel,
        values: *mut i32,
        parameter: isize,
    ) {
        let outv = match parameter {
            // NSGLParamSwapInterval | NSGLParamSurfaceOrder
            222 | 235 => 1,
            _ => panic!(
                "tried to get NSGL context parameters from oxidegl nsgl shim (param code {parameter})"
            ),
        };
        // Safety: caller ensures pointer is valid
        unsafe { *values = outv };
    }
    unsafe extern "C-unwind" fn make_current(this: *mut NSOpenGLContext, _sel: Sel) {
        let obj = Self::get_assoc_obj(this.cast());
        set_context(Some(obj.ivars().cast()));
    }
    unsafe extern "C-unwind" fn clear_current(_: *const AnyClass, _sel: Sel) {
        set_context(None);
    }
    unsafe extern "C-unwind" fn flush_buffer(_this: *mut NSOpenGLContext, _sel: Sel) {
        swap_buffers();
    }
    unsafe extern "C-unwind" fn set_view(
        this: *mut NSOpenGLContext,
        _sel: Sel,
        view: Option<NonNull<NSView>>,
    ) {
        if let Some(v) = view {
            let obj = Self::get_assoc_obj(this.cast());
            let mut ptr = obj.ivars().cast::<Context>();

            // take current context to avoid potential aliasing references
            let ctx = CTX.take();

            // Safety: pointer is non null, points to an initialized and heap-allocated Context.
            // pointer cannot have aliasing Rust references (since this class and CTX are the only places where the
            // pointer is actually read from, and we emptied CTX prior to creating this reference)
            unsafe { ptr.as_mut() }.set_view(&unsafe { Retained::retain(v.as_ptr()) }.unwrap());
            CTX.set(ctx);
        }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[unsafe(super(NSObject))]
    pub(crate) struct NSOpenGLContext;
);
// Safety: NSOpenGLContext conforms to its superclass protocol, NSObjectProtocol
unsafe impl NSObjectProtocol for NSOpenGLContext {}

fn get_sel(sel: Sel) -> *const c_void {
    // Safety: Sel is a repr(transparent) wrapper on NonNull<c_void> which has a compatible layout for transmutes with
    unsafe { mem::transmute(sel) }
}
impl OXGLOxideGlCtxShim {
    fn set_assoc_obj(this: *mut AnyObject, assoc: Retained<OXGLOxideGLCtxAssociatedObject>) {
        // Safety: self is a valid objective-C object, obj is a retained pointer to an initialized associated object
        unsafe {
            objc_setAssociatedObject(
                this,
                get_sel(sel!(oxglAssocObj)),
                Retained::into_raw(assoc.into()),
                OBJC_ASSOCIATION_RETAIN,
            );
        };
    }
    fn get_assoc_obj(this: *mut AnyObject) -> Retained<OXGLOxideGLCtxAssociatedObject> {
        // Safety: self is a valid objective c object
        let ptr = unsafe { objc_getAssociatedObject(this, get_sel(sel!(oxglAssocObj))) };
        // Safety: ptr is either null (not set) or a pointer to an associated object as set by set_assoc_obj
        unsafe { Retained::retain(ptr.cast_mut().cast::<OXGLOxideGLCtxAssociatedObject>()) }
            .expect("OxideGL context assoc objc not set!")
    }
    /// # Safety
    /// Must be called from a serial objc context (e.g. a static initializer or +load method)
    #[allow(unused)]
    unsafe fn clobber_ns_opengl() {
        static CLOBBER_ONCE: Once = Once::new();
        CLOBBER_ONCE.call_once(|| {
            trace!("Clobbering NSOpenGLContext");
            // Calling `class` ensures that the class actually gets loaded and registered before we try to do things with it
            let opengl_ctx_class_ptr = ptr::from_ref(NSOpenGLContext::class())
                .cast_mut()
                .cast::<AnyClass>();
            let opengl_ctx_class_metaclass_ptr =
                ptr::from_ref(NSOpenGLContext::class().metaclass())
                    .cast_mut()
                    .cast::<AnyClass>();
            // Safety: Caller ensures this method is only called from within
            // a static initializer
            #[allow(clippy::missing_transmute_annotations)]
            // Replace all of the relevant methods on NSOpenGLContext context and its metaclass
            unsafe {
                let sel_ptr = sel!(initWithFormat:shareContext:);

                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(
                        Self::init_with_format_share_ctx
                            as unsafe extern "C-unwind" fn(_, _, _, _) -> _,
                    ),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(initWithCGLPixelFormatObj:);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(
                        Self::init_with_cgl_pf_obj as unsafe extern "C-unwind" fn(_, _, _) -> _,
                    ),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(initWithCGLPixelFormatObj:);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(
                        Self::init_with_cgl_pf_obj as unsafe extern "C-unwind" fn(_, _, _) -> _,
                    ),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );

                let sel_ptr = sel!(setValues:forParameter:);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(
                        Self::set_values as unsafe extern "C-unwind" fn(_, _, _, _) -> _,
                    ),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(getValues:forParameter:);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(
                        Self::get_values as unsafe extern "C-unwind" fn(_, _, _, _) -> _,
                    ),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(makeCurrentContext);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(Self::make_current as unsafe extern "C-unwind" fn(_, _) -> _),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(flushBuffer);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(Self::flush_buffer as unsafe extern "C-unwind" fn(_, _) -> _),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(setView:);
                class_replaceMethod(
                    opengl_ctx_class_ptr,
                    sel_ptr,
                    mem::transmute(Self::set_view as unsafe extern "C-unwind" fn(_, _, _) -> _),
                    method_getTypeEncoding(class_getClassMethod(opengl_ctx_class_ptr, sel_ptr)),
                );
                let sel_ptr = sel!(clearCurrentContext);
                class_replaceMethod(
                    opengl_ctx_class_metaclass_ptr,
                    sel_ptr,
                    mem::transmute(Self::clear_current as unsafe extern "C-unwind" fn(_, _) -> _),
                    method_getTypeEncoding(class_getClassMethod(
                        opengl_ctx_class_metaclass_ptr,
                        sel_ptr,
                    )),
                );
            }
        });
    }
}
thread_local! {
    static OXIDEGL_HANDLE: OnceCell<*mut c_void> = const { OnceCell::new() };
}
fn get_oxidegl_handle() -> *mut c_void {
    OXIDEGL_HANDLE.with(|v| {
        *v.get_or_init(|| {
            //Safety: arguments to dlopen are valid, pointer is checked for null
            let handle = unsafe { dlopen(c"liboxidegl.dylib".as_ptr(), RTLD_LAZY) };
            assert!(
                !handle.is_null(),
                "OxideGL NSGL shim: failed to dlopen oxidegl"
            );
            handle
        })
    })
}
unsafe extern "C" fn dlopen_override(filename: *const i8, flags: i32) -> *mut c_void {
    if !filename.is_null()
    // Safety: caller ensures filename points to a valid, nul-terminated C string if non-null
        && unsafe { CStr::from_ptr(filename) }
            .to_str()
            .is_ok_and(|s| s.contains("libGL.dylib") || s.contains("OpenGL.framework/OpenGL"))
    {
        trace!(
            "intercepted dlopen of libGL.dylib/OpenGL.framework, returning oxidegl handle instead"
        );

        get_oxidegl_handle()
    } else {
        // Safety: caller ensures arguments to dlopen are correct
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
            }
            let symbol =
                CStr::from_bytes_until_nul(&buf).expect("failed to create CStr from NSString");
            trace!(
                "Redirecting NSGL function lookup of {symbol:?} to OxideGL"
            );
            dlsym(get_oxidegl_handle(), symbol.as_ptr())
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

#[allow(private_interfaces)]
#[unsafe(link_section = "__DATA,__interpose")]
#[used]
pub static DYLD_CF_BUNDLE_GET_FUNCTION_PTR_FOR_NAME_INTERPOSE: DyldInterposeTuple =
    DyldInterposeTuple {
        replacement: CFBundleGetFunctionPointerForNameOverride as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
        replacee: CFBundleGetFunctionPointerForName as unsafe extern "C" fn(_, _) -> _
            as *const c_void,
    };

#[allow(private_interfaces)]
#[unsafe(link_section = "__DATA,__interpose")]
#[used]
pub static DYLD_LIBC_DLOPEN_INTERPOSE: DyldInterposeTuple = DyldInterposeTuple {
    replacement: dlopen_override as unsafe extern "C" fn(_, _) -> _ as *const c_void,
    replacee: dlopen as unsafe extern "C" fn(_, _) -> _ as *const c_void,
};

// our crimes break Rust libtest for some reason (probably due to nasal demon summoning) so we need to not commit them if this is a test build
#[cfg(not(test))]
mod ctor {
    use ctor::ctor;

    use crate::{entry_point::oxidegl_platform_init, nsgl_shim::OXGLOxideGlCtxShim};
    #[ctor]
    fn ctor() {
        println!(
            "OxideGL running static constructor. Ensure liboxidegl is loaded BEFORE main is run. Loading liboxidegl after main may cause nasal demons to spontaneously appear"
        );
        // Safety: we are living the good life (before main), so there are no other threads to race on environment variables with
        unsafe { oxidegl_platform_init() }
        // Safety: running from static ctor (equivalent to objc +load context)
        unsafe { OXGLOxideGlCtxShim::clobber_ns_opengl() }
    }
}
