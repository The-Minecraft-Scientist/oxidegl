#![allow(clippy::inline_always)]
use std::{convert::Infallible, hint::unreachable_unchecked, mem, panic::Location, ptr};

use crate::{
    context::debug::gl_err,
    dispatch::gl_types::{GLboolean, GLenum, GLint, GLsync},
    enums::{
        ErrorCode, GL_INVALID_ENUM, GL_INVALID_FRAMEBUFFER_OPERATION, GL_INVALID_OPERATION,
        GL_INVALID_VALUE, GL_OUT_OF_MEMORY, GL_STACK_OVERFLOW, GL_STACK_UNDERFLOW,
    },
};
#[expect(clippy::derivable_impls, reason = "avoid modifying generated code")]
impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::NoError
    }
}
const ERR_OFFSET: u32 = 1279;
#[repr(u8)]
#[expect(clippy::cast_possible_truncation, reason = "manually checked consts")]
#[derive(Clone, Copy, Debug)]
// shrink this value to be byte-sized for dubious reasons
pub(crate) enum GlError {
    InvalidEnum = (GL_INVALID_ENUM - ERR_OFFSET) as u8,
    InvalidValue = (GL_INVALID_VALUE - ERR_OFFSET) as u8,
    InvalidOperation = (GL_INVALID_OPERATION - ERR_OFFSET) as u8,
    StackOverflow = (GL_STACK_OVERFLOW - ERR_OFFSET) as u8,
    StackUnderflow = (GL_STACK_UNDERFLOW - ERR_OFFSET) as u8,
    OutOfMemory = (GL_OUT_OF_MEMORY - ERR_OFFSET) as u8,
    InvalidFramebufferOperation = (GL_INVALID_FRAMEBUFFER_OPERATION - ERR_OFFSET) as u8,
}
impl GlError {
    #[inline]
    /// Wraps this [`GlError`] in a [`GlFallibleError`] for use in the [`Err`] variant of [`GlFallible`]. Used to bypass the From impl that logs unhandled errors (which is automatically
    /// invoked by [`FromResidual`] when using ? to coalesce [`GlFallible`])
    pub(crate) fn e(self) -> GlFallibleError {
        GlFallibleError {
            #[cfg(not(feature = "unsound_noerror"))]
            err: self,
        }
    }
}
impl From<GlError> for ErrorCode {
    #[inline]
    fn from(value: GlError) -> Self {
        // Safety: all variants of GlError correspond to a member discriminant of ErrorCode when added back to ERR_OFFSET
        unsafe { mem::transmute(u32::from(value as u8) + ERR_OFFSET) }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct GlFallibleError {
    #[cfg(not(feature = "unsound_noerror"))]
    err: GlError,
}
impl GlFallibleError {
    #[inline]
    pub(crate) fn get(self) -> ErrorCode {
        #[cfg(feature = "unsound_noerror")]
        // Safety: user opts into and thus takes responsibility for any potential unsoundess/UB by enabling this feature
        unsafe {
            std::hint::unreachable_unchecked()
        }
        #[cfg(not(feature = "unsound_noerror"))]
        self.err.into()
    }
}
#[cfg(not(feature = "unsound_noerror"))]
impl From<GlError> for GlFallibleError {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn from(err: GlError) -> Self {
        #[cfg(debug_assertions)]
        {
            let l = Location::caller();
            gl_err!(src: Api, ty: Error, "GlError thrown at {}:{}", l.file(), l.line());
        };
        Self { err }
    }
}
#[cfg(feature = "unsound_noerror")]
impl From<GlError> for GlFallibleError {
    #[inline]
    fn from(_: GlError) -> Self {
        // Safety: user opts into and thus takes responsibility for any potential unsoundess/UB by enabling this feature
        unsafe { std::hint::unreachable_unchecked() }
    }
}
/// Result of a possibly fallible GL command
pub(crate) trait GlResult<T, E>: Sized {
    /// Convert this possibly-fallible value into a `Result<T, Self::Error>`
    fn into_result(self) -> Result<T, E>;
    /// Normalize this possibly-fallible value into a `Result<T, GlErrorInternal>`.
    /// This function is only used to get the right error type inference in generated code.
    /// prefer using [`Self::into_result`](GlResult::into_result) where possible because it preserves
    /// the Error type
    #[inline]
    fn normalize(self) -> GlFallible<T>
    where
        E: Into<GlFallibleError>,
    {
        self.into_result().map_err(Into::into)
    }
}

impl<T> GlResult<T, Infallible> for T {
    #[inline]
    fn into_result(self) -> Result<T, Infallible> {
        Ok(self)
    }
}
impl<T> GlResult<T, GlFallibleError> for GlFallible<T> {
    #[inline]
    fn into_result(self) -> Result<T, GlFallibleError> {
        self
    }
}
impl From<Infallible> for ErrorCode {
    fn from(_: Infallible) -> Self {
        // Safety: a value of type Infallible cannot ever exist
        unsafe { unreachable_unchecked() }
    }
}
impl From<Infallible> for GlFallibleError {
    fn from(_: Infallible) -> Self {
        // Safety: a value of type Infallible cannot ever exist
        unsafe { unreachable_unchecked() }
    }
}
impl From<GlFallibleError> for ErrorCode {
    fn from(value: GlFallibleError) -> Self {
        value.get()
    }
}
pub type GlFallible<T = ()> = Result<T, GlFallibleError>;

/// Trait that declares the (constant) value to be returned when a GL command that returns a value of type T fails
pub(crate) trait GetErrorReturnValue<T> {
    #[inline]
    fn get() -> T {
        Self::VAL
    }
    const VAL: T;
}
impl GetErrorReturnValue<()> for GlFallibleError {
    const VAL: () = ();
}
impl GetErrorReturnValue<GLenum> for GlFallibleError {
    const VAL: GLenum = 0;
}
impl GetErrorReturnValue<GLboolean> for GlFallibleError {
    const VAL: GLboolean = false;
}
impl GetErrorReturnValue<GLint> for GlFallibleError {
    const VAL: GLint = 0;
}
impl GetErrorReturnValue<f32> for GlFallibleError {
    const VAL: f32 = 0.0;
}
impl<T> GetErrorReturnValue<*const T> for GlFallibleError {
    const VAL: *const T = ptr::null();
}
impl<T> GetErrorReturnValue<*mut T> for GlFallibleError {
    const VAL: *mut T = ptr::null_mut();
}
impl GetErrorReturnValue<GLsync> for GlFallibleError {
    const VAL: GLsync = None;
}
impl<T: Sized> GetErrorReturnValue<T> for Infallible {
    fn get() -> T {
        unreachable!()
    }
    const VAL: T = panic!("tried to get default/errored return value with error type Infallible");
}

macro_rules! gl_assert {
    ( $test:expr, $errno:ident ) => {
        if !($test) {
            crate::context::debug::gl_err!(src: Api, ty: Error, ::std::concat!(::std::stringify!($errno), " caused by failiure of assertion \"", ::std::stringify!($test), "\""));
            return ::std::result::Result::Err(crate::context::error::GlError::$errno.e());
        }
    };
    ( $test:expr, $errno:ident, $($msg:tt)* ) => {
        if !($test) {
            crate::context::debug::gl_err!(src: Api, ty: Error, ::std::concat!(::std::stringify!($errno), " caused by failiure of assertion \"", ::std::stringify!($test), "\": {}"), ::std::format!($($msg)*) );
            return ::std::result::Result::Err(crate::context::error::GlError::$errno.e());
        }
    };
}
pub(crate) use gl_assert;
