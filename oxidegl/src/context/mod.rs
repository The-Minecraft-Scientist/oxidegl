use objc2::rc::Id;
use objc2_app_kit::NSView;
use platform::PlatformState;
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::pin::Pin;
use std::ptr::NonNull;

use crate::gl::gltypes::GLenum;

use self::platform::MetalComponents;
use self::state::GLState;

#[allow(dead_code, unused_variables)]
pub(crate) mod commands;

#[allow(dead_code, unused_variables)]
pub(crate) mod unimplemented;

pub(crate) mod state;

pub(crate) mod platform;

thread_local! {
    static CTX: Cell<Option<CtxRef>> = const {Cell::new(None)};
}
#[derive(Debug)]
#[repr(C)]
pub struct OxideGLContext {
    gl_state: GLState,
    platform_state: PlatformState,
}

impl OxideGLContext {
    pub(crate) fn new(view: Id<NSView>) -> Self {
        Self {
            gl_state: GLState::new(),
            platform_state: PlatformState {
                metal: MetalComponents::new(view),
            },
        }
    }
}

#[inline(always)]
pub fn with_ctx<Ret, Func: for<'a> Fn(Pin<&'a mut OxideGLContext>) -> Ret>(f: Func) -> Ret {
    let mut opt = CTX.take().expect("No context set");
    let p = Pin::new(opt.deref_mut());
    let ret = f(p);
    CTX.set(Some(opt));
    ret
}

#[no_mangle]
unsafe extern "C" fn oxidegl_set_current_context(ctx: Option<CtxRef>) {
    CTX.set(ctx);
}

#[no_mangle]
unsafe extern "C" fn oxidegl_swap_buffers(ctx: CtxRef) {
    println!("oxideGl swap buffers called");
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub(crate) struct NSViewPtr(Id<NSView>);

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
    let ctx = unsafe { OxideGLContext::new(Id::new(view).unwrap()) };

    Box::into_raw(Box::new(ctx)).cast()
}

#[repr(transparent)]
pub struct CtxRef(NonNull<OxideGLContext>);
impl CtxRef {
    pub unsafe fn from_void(ptr: *mut c_void) -> Option<Self> {
        Some(Self(NonNull::new(ptr.cast())?))
    }
    pub fn as_box(self) -> Box<OxideGLContext> {
        unsafe { Box::from_raw(self.0.as_ptr()) }
    }
}
impl Deref for CtxRef {
    type Target = OxideGLContext;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for CtxRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl Drop for CtxRef {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.0.as_ptr()) };
        panic!("OxideGL just dropped your OpenGL Context!! This is really bad, time to quit!!")
    }
}
