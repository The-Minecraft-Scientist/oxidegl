use objc2::mutability::IsMutable;
use objc2::rc::Id;
use state::LocalInitCell;
use std::borrow::BorrowMut;
use std::cell::{Cell, OnceCell, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::AtomicU32;

use crate::gl::gltypes::GLenum;

use self::context_state::OxideGLContextState;
use self::metal_view::{ContextMetalComponents, NSView};

mod context_state;

pub(crate) mod dispatch_unused;
pub(crate) mod get;
pub(crate) mod metal_view;

thread_local! {
    pub static CTX: Cell<Option<CtxPtr>> = Cell::new(None);
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
pub fn get_state() -> CtxPtr {
    CTX.get().unwrap()
}
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct CtxPtr(NonNull<OxideGLContext>);
impl Deref for CtxPtr {
    type Target = OxideGLContext;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for CtxPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[no_mangle]
extern "C" fn oxidegl_set_current_context(ctx: CtxPtr) {
    CTX.set(Some(ctx));
    println!("set context {:?}", ctx.0);
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
    Box::into_raw(b)
}
