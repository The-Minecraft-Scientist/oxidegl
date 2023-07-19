use std::borrow::BorrowMut;
use std::cell::{OnceCell, RefCell, RefMut};

use state::LocalInitCell;

use crate::gl::gltypes::GLenum;

pub(crate) mod dispatch_unused;
pub(crate) mod get;
pub(crate) mod render;
static CTX: LocalInitCell<RefCell<OxideGLContext>> = LocalInitCell::new();

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OxideGLContext {
    uid: u32,
}

impl OxideGLContext {
    pub fn new(uid: u32) -> Self {
        Self { uid }
    }
}
pub fn get_state<'a>() -> RefMut<'a, OxideGLContext> {
    CTX.get().borrow_mut()
}
#[no_mangle]
extern "C" fn oxidegl_set_current_context(uid: u32) {
    println!("oxidegl_set_current_context called!");
}

#[no_mangle]
extern "C" fn oxidegl_swap_buffers(uid: u32) {
    todo!()
}

#[no_mangle]
extern "C" fn oxidegl_create_context(
    format: GLenum,
    typ: GLenum,
    depth_format: GLenum,
    depth_type: GLenum,
    stencil_format: GLenum,
    stencil_type: GLenum,
) -> u32 {
    println!("oxidegl_create_context called!");
    CTX.set(|| RefCell::new(OxideGLContext::new(2)));
    2
}
