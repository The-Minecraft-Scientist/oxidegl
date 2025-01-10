use self::state::GLState;
use likely_stable::if_likely;
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use objc2_metal::MTLPixelFormat;
use platform::PlatformState;
use std::cell::Cell;
use std::panic;
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

pub(crate) mod debug;
pub(crate) mod framebuffer;
pub(crate) mod program;
pub(crate) mod shader;
pub(crate) mod state;
pub(crate) mod texture;

pub(crate) mod gl_object;
pub(crate) mod platform;

thread_local! {
    pub(crate) static CTX: Cell<Option<NonNull<Context>>> = const {Cell::new(None)};
}
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub(crate) gl_state: GLState,
    pub(crate) platform_state: PlatformState,
}

impl Context {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            gl_state: GLState::default(),
            platform_state: PlatformState::new(MTLPixelFormat::BGRA8Unorm_sRGB, None, None),
        }
    }
    pub fn set_view(&mut self, view: &Retained<NSView>, backing_scale_factor: f64) {
        self.platform_state.set_view(view, backing_scale_factor);
        // init scissor box/viewport now that we have an actual view
        let dims = self.platform_state.target_defaultfb_dims();
        self.gl_state.viewport.width = dims.0;
        self.gl_state.viewport.height = dims.1;

        self.gl_state.scissor_box.width = dims.0;
        self.gl_state.scissor_box.height = dims.1;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
// This function is only used by GL dispatch. It is always advantageous for it to be inlined in that usage
#[expect(clippy::inline_always)]
#[inline(always)]
#[expect(unused_mut, unused_variables, reason = "lint bug")]
pub fn with_ctx_mut<Ret, Func: for<'a> Fn(Pin<&'a mut Context>) -> Ret>(f: Func) -> Ret {
    // use if_likely to tell LLVM that it should optimize for the Some(ptr) case
    if_likely! {
        // take the current context pointer
        // (this effectively takes a single-threaded "lock" on the context which protects against
        // the user doing Weird Stuff and running multiple GL commands simultaneously)

        let Some(mut ptr) = CTX.take() => {

        // Safety: we are the exclusive accessor of ptr due to its thread locality and the fact that we called `take` on it previously
        // wrap the context reference in a pin to ensure it is not moved out of
        let p = Pin::new(unsafe { ptr.as_mut() });
        let ret = f(p);
        CTX.set(Some(ptr));
        ret
        } else {
            panic!("no context set!");
        }
    }
}
