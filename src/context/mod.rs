use objc2::mutability::IsMutable;
use objc2::rc::Id;
use state::LocalInitCell;
use std::borrow::BorrowMut;
use std::cell::{OnceCell, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU32;

use crate::gl::gltypes::GLenum;

use self::context_state::OxideGLContextState;
use self::metal_view::{ContextMetalComponents, NSView};

mod context_state;

pub(crate) mod dispatch_unused;
pub(crate) mod get;
pub(crate) mod metal_view;

static CTX: LocalInitCell<RefCell<Option<OxideGLContext>>> = LocalInitCell::new();
static COUNTER: AtomicU32 = AtomicU32::new(1);

#[derive(Debug)]
#[repr(C)]
pub struct OxideGLContext {
    state: OxideGLContextState,
    uid: u32,
}

impl OxideGLContext {
    pub(crate) fn new(uid: u32, view: NSViewPtr) -> Self {
        Self {
            state: OxideGLContextState::new(view),
            uid,
        }
    }
}
pub fn get_state<'a>() -> FuckedUpPtr<'a> {
    let m = CTX.get().borrow_mut();
    dbg!(std::thread::current().id());
    if m.is_some() {
        FuckedUpPtr { backing: m }
    } else {
        panic!("tried to get OxideGL state but state was not initialized!");
    }
}
pub struct FuckedUpPtr<'a> {
    backing: RefMut<'a, Option<OxideGLContext>>,
}
impl<'a> Deref for FuckedUpPtr<'a> {
    type Target = OxideGLContext;

    fn deref(&self) -> &Self::Target {
        //SAFETY: the only code that crates FuckedUpPtrs ensures this invariant
        unsafe { self.backing.as_ref().unwrap_unchecked() }
    }
}
impl<'a> DerefMut for FuckedUpPtr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        //SAFETY: the only code that crates FuckedUpPtrs ensures this invariant
        unsafe { self.backing.as_mut().unwrap_unchecked() }
    }
}
#[no_mangle]
extern "C" fn oxidegl_set_current_context(uid: u32) {
    println!("oxidegl_set_current_context called!");
}

#[no_mangle]
extern "C" fn oxidegl_swap_buffers(uid: u32) {
    println!("oxideGl swap buffers called");
}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub(crate) struct NSViewPtr(Id<NSView>);
//NSViewPtr is NOT actually Sync or Send. This is to make our TLS state storage happy.
unsafe impl Sync for NSViewPtr {}
unsafe impl Send for NSViewPtr {}
#[no_mangle]
extern "C" fn oxidegl_create_context(
    view: *mut NSView,
    format: GLenum,
    typ: GLenum,
    depth_format: GLenum,
    depth_type: GLenum,
    stencil_format: GLenum,
    stencil_type: GLenum,
) -> u32 {
    dbg!(std::thread::current().id());
    let ptr = NSViewPtr(unsafe { Id::new(view).unwrap() });
    println!("oxidegl_create_context called!");
    let uid = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    CTX.set(move || RefCell::new(None));
    *CTX.get().borrow_mut() = Some(OxideGLContext::new(uid, ptr));
    uid
}
