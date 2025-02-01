use std::{ffi::c_void, ptr::NonNull, sync::Once};

use log::{debug, info};
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use crate::{
    context::{
        debug::{self, gl_trace},
        with_ctx_mut, Context, CTX,
    },
    dispatch::gl_types::GLenum,
};

#[no_mangle]
unsafe extern "C" fn oxidegl_set_current_context(ctx: Option<NonNull<Context>>) {
    set_context(ctx);
}
pub fn set_context(ctx: Option<NonNull<Context>>) {
    if let Some(mut prev) = CTX.take() {
        // early return if we are setting the same context again
        if ctx == Some(prev) {
            return;
        }
        // Safety: we are the exclusive accessor of this context, as we just removed it from CTX
        unsafe { prev.as_mut() }.uninstall_debug_state();
    }
    if let Some(mut c) = ctx {
        // Safety: we are the exclusive accessor of this context, because it has not been installed to CTX yet
        unsafe { c.as_mut() }.install_debug_state();
        CTX.set(ctx);
    }
    if ctx.is_some() {
        gl_trace!("set context to {:?}", ctx);
    }
}
pub fn swap_buffers() {
    with_ctx_mut(|mut ctx| {
        // thank god for deref patterns
        let Context {
            gl_state: state,
            platform_state: platform,
        } = &mut *ctx;
        platform.swap_buffers(state);
    });
    gl_trace!("swap buffers");
}
#[no_mangle]
unsafe extern "C" fn oxidegl_swap_buffers(_ctx: Option<NonNull<Context>>) {
    swap_buffers();
}
#[must_use]
pub(crate) fn box_ctx(ctx: Context) -> NonNull<Context> {
    let p = Box::into_raw(Box::new(ctx));
    // Safety: Box guarantees that the pointer is non-null
    unsafe { NonNull::new_unchecked(p) }
}
#[no_mangle]
/// # Safety
/// This needs to be run as early as possible (ideally before the program spawns a thread other than the main thread)
pub unsafe extern "C" fn oxidegl_platform_init() {
    // wrap in a Once to make sure we don't init twice
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        debug::init_logger();
        info!("OxideGL {}", Context::VERSION_INFO);
        #[cfg(debug_assertions)]
        // Safety: We pray that we aren't racing with anyone else's code writing env vars.
        // This isn't *too* bad because we're running on the main thread, which is where
        // a majority of the writes occur in practice.
        unsafe {
            use std::env::set_var;

            set_var("MTL_DEBUG_LAYER", "1");
            set_var("MTL_SHADER_VALIDATION", "1");
            set_var("MTL_DEBUG_LAYER_VALIDATE_UNRETAINED_RESOURCES", "0x4");
            set_var("RUST_BACKTRACE", "1");
        }
    });
}

#[no_mangle]
/// # Safety
/// This function must be called EXACTLY once per context, or it will result in a double free
unsafe extern "C" fn oxidegl_destroy_context(ctx: Option<NonNull<Context>>) {
    if let Some(c) = ctx {
        // Safety: caller ensures c points to a valid Context struct, allocated via a Box with the same allocator
        drop(unsafe { Box::from_raw(c.as_ptr()) });
    }
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
    let mut ctx = Context::new();
    // Safety: caller ensures ptr is a pointer to a valid, initialized NSView.
    let view = unsafe { Retained::retain(view).unwrap() };

    ctx.set_view(&view);
    debug!("Created context");
    box_ctx(ctx).as_ptr().cast()
}
