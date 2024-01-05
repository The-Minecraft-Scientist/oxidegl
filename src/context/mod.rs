use objc2::mutability::IsMutable;
use objc2::rc::Id;
use state::LocalInitCell;
use std::borrow::BorrowMut;
use std::cell::{Cell, OnceCell, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU32};

use crate::gl::gltypes::GLenum;

use self::context_state::OxideGLContextState;
use self::metal_view::{ContextMetalComponents, NSView};

mod context_state;

pub(crate) mod dispatch_unused;
pub(crate) mod get;
pub(crate) mod metal_view;

thread_local! {
    pub static CTX: Cell<Option<NonNull<OxideGLContext>>> = const {Cell::new(None)};
}

#[derive(Debug)]
#[repr(C)]
pub struct OxideGLContext {
    state: OxideGLContextState,
}

impl OxideGLContext {
    pub(crate) fn new(view: NSViewPtr) -> Self {
        Self {
            state: OxideGLContextState::new(view),
        }
    }
}
static CTX_INUSE: AtomicBool = AtomicBool::new(false);
pub fn get_state() -> CtxPtr {
    println!(
        "\n\nstack trace from get_state: {}",
        std::backtrace::Backtrace::force_capture().to_string()
    );
    let c = CTX.with(|a| a.get()).take().unwrap();
    println!("got context {:?}", &c);
    CTX_INUSE.store(true, std::sync::atomic::Ordering::Release);
    CtxPtr(c)
}
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct CtxPtr(NonNull<OxideGLContext>);
impl Deref for CtxPtr {
    type Target = OxideGLContext;
    fn deref(&self) -> &Self::Target {
        if CTX_INUSE.load(std::sync::atomic::Ordering::Acquire) {
            panic!("tried to create aliasing references to CTX");
        }
        unsafe { self.0.as_ref() }
    }
}
impl Drop for CtxPtr {
    fn drop(&mut self) {
        CTX_INUSE.store(false, std::sync::atomic::Ordering::Release);
    }
}
impl DerefMut for CtxPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if CTX_INUSE.load(std::sync::atomic::Ordering::Acquire) {
            panic!("tried to create aliasing mutable references to CTX");
        }
        unsafe { self.0.as_mut() }
    }
}

#[no_mangle]
extern "C" fn oxidegl_set_current_context(ctx: Option<NonNull<OxideGLContext>>) {
    println!(
        "stace trace in set_context: {}",
        std::backtrace::Backtrace::force_capture().to_string()
    );
    CTX.set(ctx);
    println!("set context {:?}", ctx);
}

#[no_mangle]
extern "C" fn oxidegl_swap_buffers(ctx: CtxPtr) {
    println!("oxideGl swap buffers called");
}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub(crate) struct NSViewPtr(Id<NSView>);

#[no_mangle]
extern "C" fn oxidegl_create_context(
    view: *mut NSView,
    format: GLenum,
    typ: GLenum,
    depth_format: GLenum,
    depth_type: GLenum,
    stencil_format: GLenum,
    stencil_type: GLenum,
) -> *mut OxideGLContext {
    let ptr = NSViewPtr(unsafe { Id::new(view).unwrap() });
    let b = Box::new(OxideGLContext::new(ptr));
    let p = Box::into_raw(b);
    let ptr = unsafe { NonNull::new_unchecked(p) };
    p
}
