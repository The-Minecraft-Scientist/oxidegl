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
    trace!("set context to {:?}", ctx);
    CTX.set(ctx);
}

#[no_mangle]
unsafe extern "C" fn oxidegl_swap_buffers(_ctx: Option<NonNull<Context>>) {
    debug!("swap buffers called");
}
#[no_mangle]
unsafe extern "C" fn oxidegl_platform_init() {
    Logger::try_with_env_or_str("none, oxidegl=trace")
        .unwrap()
        .start()
        .unwrap();
    trace!("OxideGL Logger initialized");
    info!("OxideGL {}", Context::VERSION_INFO);
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
    // Safety: caller ensures ptr is a pointer to a valid, initialized NSView.
    // It is retained because we need it to live until we've injected our layer. (which happens in PlatformState::new)
    let ctx = unsafe { Context::new(&Retained::retain(view).unwrap()) };
    debug!("Created context");
    Box::into_raw(Box::new(ctx)).cast()
}
