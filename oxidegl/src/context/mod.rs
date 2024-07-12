use log::{debug, trace, Log};
use objc2::rc::{Id, Retained};
use objc2_app_kit::NSView;
use platform::PlatformState;
use std::cell::Cell;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::compiler_fence;
use std::thread::LocalKey;

use crate::dispatch::gl_types::GLenum;

use self::platform::MetalComponents;
use self::state::GLState;

#[allow(
    dead_code,
    unused_variables,
    clippy::wildcard_imports,
    clippy::too_many_arguments,
    clippy::unused_self,
    clippy::similar_names,
    clippy::missing_safety_doc
)]
pub(crate) mod commands;

pub(crate) mod state;

pub(crate) mod platform;

thread_local! {
    static CTX: Cell<Option<NonNull<Context>>> = const {Cell::new(None)};
}
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    gl_state: GLState,
    platform_state: PlatformState,
}

impl Context {
    pub(crate) fn new(view: &Id<NSView>) -> Self {
        Self {
            gl_state: GLState::new(),
            platform_state: PlatformState {
                metal: MetalComponents::new(view),
            },
        }
    }
}
// This function is only used by GL dispatch. It is always advantageous for it to be inlined in that usage
#[allow(clippy::inline_always)]
//#[inline(always)]
pub fn with_ctx<Ret, Func: for<'a> Fn(Pin<&'a mut Context>) -> Ret>(f: Func) -> Ret {
    let mut ptr: NonNull<Context> = CTX.take().expect("No context set");

    //SAFETY: we are the exclusive accessor of ptr due to its thread locality and the fact that we called `take` on it previously
    let p = Pin::new(unsafe { ptr.as_mut() });

    let ret = f(p);
    CTX.set(Some(ptr));
    compiler_fence(std::sync::atomic::Ordering::SeqCst);
    ret
}

#[no_mangle]
unsafe extern "C" fn oxidegl_set_current_context(ctx: Option<NonNull<Context>>) {
    trace!("set context to {:?}", ctx);
    CTX.set(ctx);
}

#[no_mangle]
unsafe extern "C" fn oxidegl_swap_buffers(_ctx: Option<NonNull<Context>>) {
    debug!("swap buffers called");
}
#[no_mangle]
unsafe extern "C" fn oxidegl_platform_init() {
    simple_logger::init_with_env().expect("failed to initialize OxideGL's logger!");
    debug!("OxideGL Logger initialized");
}

#[no_mangle]
unsafe extern "C" fn oxidegl_create_context(
    view: *mut NSView,
    format: GLenum,
    typ: GLenum,
    depth_format: GLenum,
    depth_type: GLenum,
    stencil_format: GLenum,
    stencil_type: GLenum,
) -> *mut c_void {
    // Safety: caller ensures ptr is a pointer to a valid, initialized NSView.
    // It is retained because we need it to live until we've injected our layer. (which happens in PlatformState::new)
    let ctx = unsafe { Context::new(&Retained::retain(view).unwrap()) };
    debug!("Created context");
    Box::into_raw(Box::new(ctx)).cast()
}
