use dashmap::DashMap;
use objc2::mutability::IsMutable;
use objc2::rc::Id;
use state::{InitCell, LocalInitCell};
use std::borrow::BorrowMut;
use std::cell::{Cell, OnceCell, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Mutex, MutexGuard};

use crate::gl::gltypes::GLenum;

use self::context_state::OxideGLContextState;
use self::metal_view::{ContextMetalComponents, NSView};

mod context_state;

pub(crate) mod dispatch_unused;
pub(crate) mod get;
pub(crate) mod metal_view;

thread_local! {
    static CTX: Cell<Option<CtxRef>> = const {Cell::new(None)};
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CtxRef(u32);
pub static CTX_STORE: InitCell<DashMap<CtxRef, OxideGLContext>> = InitCell::new();

#[derive(Debug)]
#[repr(C)]
pub struct OxideGLContext {
    state: OxideGLContextState,
}

impl OxideGLContext {
    pub(crate) unsafe fn new(view: NSViewPtr) -> Self {
        Self {
            state: OxideGLContextState::new(view),
        }
    }
}
pub fn get_state<'a>() -> dashmap::mapref::one::RefMut<'a, CtxRef, OxideGLContext> {
    println!(
        "\n\nstack trace from get_state: {}",
        std::backtrace::Backtrace::force_capture().to_string()
    );
    dbg!(CTX_IDIOT.get());
    //Panic here
    dbg!(&CTX.get().unwrap());
    CTX_STORE.get().get_mut(&CTX.get().unwrap()).unwrap()
}

#[no_mangle]
extern "C" fn oxidegl_set_current_context(ctx: CtxRef) {
    println!(
        "stack trace in set_context: {}",
        std::backtrace::Backtrace::force_capture().to_string()
    );
    if ctx.0 == 0 {
        CTX.set(None);
    } else {
        CTX.set(Some(ctx));
    }
    println!("set context {:?}", ctx);
}

#[no_mangle]
extern "C" fn oxidegl_swap_buffers(ctx: CtxRef) {
    println!("oxideGl swap buffers called");
}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub(crate) struct NSViewPtr(Id<NSView>);
static COUNTER: AtomicU32 = AtomicU32::new(1);
#[no_mangle]
extern "C" fn oxidegl_create_context(
    view: *mut NSView,
    format: GLenum,
    typ: GLenum,
    depth_format: GLenum,
    depth_type: GLenum,
    stencil_format: GLenum,
    stencil_type: GLenum,
) -> CtxRef {
    let ptr = NSViewPtr(unsafe { Id::new(view).unwrap() });
    let this_id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    let ctx = unsafe { OxideGLContext::new(ptr) };
    CTX_STORE
        .get_or_init(|| DashMap::new())
        .insert(CtxRef(this_id), ctx);
    CtxRef(this_id)
}
