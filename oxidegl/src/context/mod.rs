use objc2::rc::Id;
use objc2_app_kit::NSView;
use platform::PlatformState;
use state::NeedsRefreshBits;
use std::cell::Cell;
use std::pin::Pin;
use std::ptr::NonNull;

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
    pub(crate) static CTX: Cell<Option<NonNull<Context>>> = const {Cell::new(None)};
}
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    dirty_components: NeedsRefreshBits,
    gl_state: GLState,
    platform_state: PlatformState,
}
impl Context {
    #[inline]
    pub(crate) fn dirty_render_pass(&mut self) {
        self.dirty_components |= NeedsRefreshBits::RENDER_PASS;
    }
    #[inline]
    pub(crate) fn dirty_buffers(&mut self) {
        self.dirty_components |= NeedsRefreshBits::BUFFERS;
    }
}

impl Context {
    pub(crate) fn new(view: &Id<NSView>) -> Self {
        Self {
            dirty_components: NeedsRefreshBits::empty(),
            gl_state: GLState::default(),
            platform_state: PlatformState {
                metal: MetalComponents::new(view),
            },
        }
    }
}
// This function is only used by GL dispatch. It is always advantageous for it to be inlined in that usage
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn with_ctx<Ret, Func: for<'a> Fn(Pin<&'a mut Context>) -> Ret>(f: Func) -> Ret {
    let mut ptr: NonNull<Context> = CTX.take().expect("No context set");

    //SAFETY: we are the exclusive accessor of ptr due to its thread locality and the fact that we called `take` on it previously
    let p = Pin::new(unsafe { ptr.as_mut() });

    let ret = f(p);
    CTX.set(Some(ptr));
    ret
}
