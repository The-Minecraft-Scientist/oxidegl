use std::{ffi::c_void, marker::PhantomData, mem::MaybeUninit, ops::Deref, ptr};

pub type GLint = i32;
pub type PtrToConstPtrToConstVoid = *mut *const c_void;
pub type GLuint64 = u64;
pub type GLuint = u32;
pub type GLfloat = f32;
pub type GLboolean = bool;
pub type GLchar = u8;
pub type GLdouble = f64;
pub type GLsizeiptr = isize;
pub type VoidPtr = *mut c_void;
pub type ConstVoidPtr = *const c_void;
pub type GLushort = u16;
pub type GLsizei = i32;
pub type GLintptr = isize;
pub type GLenum = u32;
pub type PtrToVoidPtr = *mut *mut c_void;
pub type GLbitfield = u32;
pub type GLshort = i16;
pub type GLubyte = u8;
pub type GLDEBUGPROC = extern "C" fn(
    source: GLenum,
    typ: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *const c_void,
);
pub type GLbyte = i8;
pub type GLsync =
    extern "C" fn(_cl_context: *mut c_void, _cl_event: *mut c_void, flags: GLbitfield);
pub type GLint64 = i64;
pub type GLvoid = c_void;

pub trait UnsafeFromGLenum {
    unsafe fn unsafe_from_gl_enum(val: GLenum) -> Self;
}
/// Helper trait to convert from "raw" GLenums to wrappers around subsets of those that are valid for certain functions
pub trait GLenumExt<T> {
    unsafe fn into_enum(val: Self) -> T;
}

impl<T> GLenumExt<T> for GLenum
where
    T: UnsafeFromGLenum,
{
    unsafe fn into_enum(val: u32) -> T {
        unsafe { T::unsafe_from_gl_enum(val) }
    }
}

pub(crate) trait GlDstType: Copy {
    fn from_uint(val: u32) -> Self;
    fn from_ulong(val: u64) -> Self;
    fn from_int(val: i32) -> Self;
    fn from_long(val: i64) -> Self;
    fn from_float(val: f32) -> Self;
    fn from_double(val: f64) -> Self;
    fn from_bool(val: bool) -> Self;
}

trait SrcType<Dst: GlDstType>: Copy {
    fn cast(self) -> Dst;
}

pub trait StateQueryWrite<Dst: GlDstType> {
    type It;
    unsafe fn write_all(&self, ptr: *mut Dst)
    where
        Self::It: SrcType<Dst>;
}

impl<It, Dst: GlDstType> StateQueryWrite<Dst> for It {
    type It = Self;
    unsafe fn write_all(&self, ptr: *mut Dst)
    where
        Self::It: SrcType<Dst>,
    {
        // Safety: caller ensures that Dst is the correct type for the allocation being written to
        unsafe { ptr::write(ptr, self.cast()) }
    }
}

impl<It, Dst: GlDstType> StateQueryWrite<Dst> for [It] {
    type It = It;
    unsafe fn write_all(&self, mut ptr: *mut Dst)
    where
        Self::It: SrcType<Dst>,
    {
        for item in self {
            // Safety: Caller ensures that self is the correct length for the allocation being written to,
            // and that Dst is the correct type for the allocation being written to
            unsafe { ptr::write(ptr, item.cast()) }
            unsafe { ptr = ptr.add(1) }
        }
    }
}

impl<Dst: GlDstType> SrcType<Dst> for u32 {
    fn cast(self) -> Dst {
        Dst::from_uint(self)
    }
}

impl GlDstType for u32 {
    fn from_int(val: i32) -> Self {
        val.abs() as Self
    }
    fn from_long(val: i64) -> Self {
        val.abs() as Self
    }
    fn from_uint(val: u32) -> Self {
        val
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }

    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    fn from_bool(val: bool) -> Self {
        val as Self
    }
}
