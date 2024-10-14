use std::{ffi::c_void, ptr::NonNull};

use flexi_logger::Logger;
use log::{debug, info, trace};
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use crate::{
    context::{Context, CTX},
    dispatch::gl_types::GLenum,
};

#[no_mangle]
unsafe extern "C" fn oxidegl_set_current_context(ctx: Option<NonNull<Context>>) {
    set_context(ctx);
}
pub fn set_context(ctx: Option<NonNull<Context>>) {
    trace!("set context to {:?}", ctx);
    CTX.set(ctx);
}
pub fn swap_buffers() {
    debug!("swap buffers called");
}
#[no_mangle]
unsafe extern "C" fn oxidegl_swap_buffers(_ctx: Option<NonNull<Context>>) {
    swap_buffers();
}
#[must_use]
pub fn box_ctx(ctx: Context) -> NonNull<Context> {
    let p = Box::into_raw(Box::new(ctx));
    // Safety: Box guarantees that the pointer is non-null
    unsafe { NonNull::new_unchecked(p) }
}
#[no_mangle]
/// # Safety
/// This needs to be run as early as possible (ideally before the program spawns a thread other than the main thread)
pub unsafe extern "C" fn oxidegl_platform_init() {
    Logger::try_with_env_or_str("none, oxidegl=trace")
        .unwrap()
        .start()
        .unwrap();
    trace!("OxideGL Logger initialized");
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
    // It is retained because we need it to live until we've injected our layer. (which happens in PlatformState::new)
    let view = unsafe { Retained::retain(view).unwrap() };
    ctx.set_view(&view);
    debug!("Created context");
    box_ctx(ctx).as_ptr().cast()
}
