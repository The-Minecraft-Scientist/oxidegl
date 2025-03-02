use std::ffi::c_void;

pub type GLint = i32;
pub type GLuint64 = u64;
pub type GLuint = u32;
pub type GLfloat = f32;
pub type GLboolean = bool;
pub type GLchar = std::ffi::c_char;
pub type GLdouble = f64;
pub type GLsizeiptr = isize;
pub type GLushort = u16;
pub type GLsizei = i32;
pub type GLintptr = isize;
pub type GLenum = u32;
pub type GLbitfield = u32;
pub type GLshort = i16;
pub type GLubyte = u8;
pub type GLDEBUGPROC = Option<
    unsafe extern "C" fn(
        source: GLenum,
        typ: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const c_void,
    ),
>;
pub type GLbyte = i8;
pub type GLsync =
    Option<extern "C" fn(_cl_context: *mut c_void, _cl_event: *mut c_void, flags: GLbitfield)>;

pub type GLint64 = i64;
pub type GLvoid = c_void;
