use self::state::GLState;
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use objc2_metal::MTLPixelFormat;
use platform::PlatformState;
use std::cell::Cell;
use std::pin::Pin;
use std::ptr::NonNull;

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

pub(crate) mod framebuffer;
pub(crate) mod program;
pub(crate) mod shader;
pub(crate) mod state;
pub(crate) mod texture;

pub(crate) mod platform;

thread_local! {
    pub(crate) static CTX: Cell<Option<NonNull<Context>>> = const {Cell::new(None)};
}
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    gl_state: GLState,
    platform_state: PlatformState,
}
impl Context {}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            gl_state: GLState::default(),
            platform_state: PlatformState::new(MTLPixelFormat::BGRA8Unorm),
        }
    }
    pub fn set_view(&self, view: &Retained<NSView>) {
        self.platform_state.set_view(view);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
// This function is only used by GL dispatch. It is always advantageous for it to be inlined in that usage
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn with_ctx<Ret, Func: for<'a> Fn(Pin<&'a mut Context>) -> Ret>(f: Func) -> Ret {
    let mut ptr: NonNull<Context> = CTX.take().expect("No context set");
    // Safety: we are the exclusive accessor of ptr due to its thread locality and the fact that we called `take` on it previously
    let p = Pin::new(unsafe { ptr.as_mut() });

    let ret = f(p);
    CTX.set(Some(ptr));
    ret
}
