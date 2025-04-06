use crate::{
    context::{
        Context,
        debug::gl_debug,
        error::{GlFallible, gl_assert},
        state::PixelAlignedRect,
    },
    dispatch::gl_types::{GLenum, GLfloat, GLint, GLsizei, GLuint},
    enums::ErrorCode,
    util::run_if_changed,
};

impl Context {
    // glScissor ---------------

    /// ### Parameters
    /// `x`
    ///
    /// `y`
    ///
    /// > Specify the lower left corner of the scissor box. Initially (0, 0).
    ///
    /// `width`
    ///
    /// `height`
    ///
    /// > Specify the width and height of the scissor box. When a GL context is first
    /// > attached to a window, `width` and `height` are set to the dimensions of
    /// > that window.
    ///
    /// ### Description
    /// [**glScissor**](crate::context::Context::oxidegl_scissor) defines a rectangle,
    /// called the scissor box, in window coordinates. The first two arguments,
    /// `x` and `y`, specify the lower left corner of the box. `width` and `height`
    /// specify the width and height of the box.
    ///
    /// To enable and disable the scissor test, call [**glEnable**](crate::context::Context::oxidegl_enable)
    /// and [**glDisable**](crate::context::Context::oxidegl_disable) with argument
    /// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST). The test is initially
    /// disabled. While the test is enabled, only pixels that lie within the scissor
    /// box can be modified by drawing commands. Window coordinates have integer
    /// values at the shared corners of frame buffer pixels.
    ///
    /// When the scissor test is disabled, it is as though the scissor box includes
    /// the entire window.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_SCISSOR_BOX`](crate::enums::GL_SCISSOR_BOX)
    ///
    /// [**glIsEnabled**](crate::context::Context::oxidegl_is_enabled) with argument
    /// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST)
    pub fn oxidegl_scissor(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        panic!("command oxidegl_scissor not yet implemented");
    }

    /// ### Parameters
    /// `first`
    ///
    /// > Specifies the index of the first viewport whose scissor box to modify.
    ///
    /// `count`
    ///
    /// > Specifies the number of scissor boxes to modify.
    ///
    /// `v`
    ///
    /// > Specifies the address of an array containing the left, bottom, width and
    /// > height of each scissor box, in that order.
    ///
    /// ### Description
    /// [**glScissorArrayv**](crate::context::Context::oxidegl_scissor_arrayv)
    /// defines rectangles, called scissor boxes, in window coordinates for each
    /// viewport. `first` specifies the index of the first scissor box to modify
    /// and `count` specifies the number of scissor boxes to modify. `first` must
    /// be less than the value of [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS),
    /// and `first`+ `count` must be less than or equal to the value of [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS).
    /// `v` specifies the address of an array containing integers specifying the
    /// lower left corner of the scissor boxes, and the width and height of the
    /// scissor boxes, in that order.
    ///
    /// To enable and disable the scissor test, call [**glEnable**](crate::context::Context::oxidegl_enable)
    /// and [**glDisable**](crate::context::Context::oxidegl_disable) with argument
    /// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST). The test is initially
    /// disabled for all viewports. While the test is enabled, only pixels that
    /// lie within the scissor box can be modified by drawing commands. Window
    /// coordinates have integer values at the shared corners of frame buffer pixels.
    ///
    /// When the scissor test is disabled, it is as though the scissor box includes
    /// the entire window.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_SCISSOR_BOX`](crate::enums::GL_SCISSOR_BOX)
    ///
    /// [**glIsEnabled**](crate::context::Context::oxidegl_is_enabled) with argument
    /// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST)
    pub unsafe fn oxidegl_scissor_arrayv(
        &mut self,
        first: GLuint,
        count: GLsizei,
        v: *const GLint,
    ) {
        panic!("command oxidegl_scissor_arrayv not yet implemented");
    }

    // glViewport ---------------

    /// ### Parameters
    /// `x`
    ///
    /// `y`
    ///
    /// > Specify the lower left corner of the viewport rectangle, in pixels. The
    /// > initial value is (0,0).
    ///
    /// `width`
    ///
    /// `height`
    ///
    /// > Specify the width and height of the viewport. When a GL context is first
    /// > attached to a window, `width` and `height` are set to the dimensions of
    /// > that window.
    ///
    /// ### Description
    /// [**glViewport**](crate::context::Context::oxidegl_viewport) specifies the
    /// affine transformation of `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
    ///
    ///
    ///
    /// Viewport width and height are silently clamped to a range that depends
    /// on the implementation. To query this range, call [**glGet**](crate::context::Context::oxidegl_get)
    /// with argument [`GL_MAX_VIEWPORT_DIMS`](crate::enums::GL_MAX_VIEWPORT_DIMS).
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_VIEWPORT`](crate::enums::GL_VIEWPORT)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_VIEWPORT_DIMS`](crate::enums::GL_MAX_VIEWPORT_DIMS)
    #[expect(clippy::cast_sign_loss)]
    pub fn oxidegl_viewport(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> GlFallible {
        gl_debug!("glViewport, x {x} y {y} width {width} height {height}");
        gl_assert!(width >= 0 && height >= 0, InvalidValue);
        debug_assert!(x >= 0 && y >= 0, "negative base coordinate in glViewport");

        run_if_changed!(self.gl_state.viewport;= PixelAlignedRect {
            x: x as u32,
            y: y as u32,
            width: width as u32,
            height: height as u32,
        } => self.update_encoder());
        Ok(())
    }
    /// ### Parameters
    /// `first`
    ///
    /// > Specify the first viewport to set.
    ///
    /// `count`
    ///
    /// > Specify the number of viewports to set.
    ///
    /// `v`
    ///
    /// > Specify the address of an array containing the viewport parameters.
    ///
    /// ### Description
    /// [**glViewportArrayv**](crate::context::Context::oxidegl_viewport_arrayv)
    /// specifies the parameters for multiple viewports simulataneously. `first`
    /// specifies the index of the first viewport to modify and `count` specifies
    /// the number of viewports to modify. `first` must be less than the value
    /// of [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS), and `first`+ `count`
    /// must be less than or equal to the value of [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS).
    /// Viewports whose indices lie outside the range \[ `first`, `first`+ `count`)
    /// are not modified. `v` contains the address of an array of floating point
    /// values specifying the left( `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
    /// `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
    /// `[inlineq]` `[inlineq]`
    ///
    ///
    ///
    /// The location of the viewport's bottom left corner, given by( `[inlineq]`
    /// `[inlineq]` `[inlineq]` `[inlineq]` [**glGet**](crate::context::Context::oxidegl_get)
    /// with argument [`GL_VIEWPORT_BOUNDS_RANGE`](crate::enums::GL_VIEWPORT_BOUNDS_RANGE).
    /// Viewport width and height are silently clamped to a range that depends
    /// on the implementation. To query this range, call [**glGet**](crate::context::Context::oxidegl_get)
    /// with argument [`GL_MAX_VIEWPORT_DIMS`](crate::enums::GL_MAX_VIEWPORT_DIMS).
    ///
    /// The precision with which the GL interprets the floating point viewport
    /// bounds is implementation-dependent and may be determined by querying the
    /// impementation-defined constant [`GL_VIEWPORT_SUBPIXEL_BITS`](crate::enums::GL_VIEWPORT_SUBPIXEL_BITS).
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_VIEWPORT`](crate::enums::GL_VIEWPORT)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_VIEWPORT_DIMS`](crate::enums::GL_MAX_VIEWPORT_DIMS)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_VIEWPORT_BOUNDS_RANGE`](crate::enums::GL_VIEWPORT_BOUNDS_RANGE)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_VIEWPORT_SUBPIXEL_BITS`](crate::enums::GL_VIEWPORT_SUBPIXEL_BITS)
    pub unsafe fn oxidegl_viewport_arrayv(
        &mut self,
        first: GLuint,
        count: GLsizei,
        v: *const GLfloat,
    ) {
        panic!("command oxidegl_viewport_arrayv not yet implemented");
    }
    /// ### Description
    /// [**glGetError**](crate::context::Context::oxidegl_get_error) returns the
    /// value of the error flag. Each detectable error is assigned a numeric code
    /// and symbolic name. When an error occurs, the error flag is set to the appropriate
    /// error code value. No other errors are recorded until [**glGetError**](crate::context::Context::oxidegl_get_error)
    /// is called, the error code is returned, and the flag is reset to [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR).
    /// If a call to [**glGetError**](crate::context::Context::oxidegl_get_error)
    /// returns [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR), there has been no detectable
    /// error since the last call to [**glGetError**](crate::context::Context::oxidegl_get_error),
    /// or since the GL was initialized.
    ///
    /// To allow for distributed implementations, there may be several error flags.
    /// If any single error flag has recorded an error, the value of that flag
    /// is returned and that flag is reset to [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR)
    /// when [**glGetError**](crate::context::Context::oxidegl_get_error) is called.
    /// If more than one flag has recorded an error, [**glGetError**](crate::context::Context::oxidegl_get_error)
    /// returns and clears an arbitrary error flag value. Thus, [**glGetError**](crate::context::Context::oxidegl_get_error)
    /// should always be called in a loop, until it returns [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR),
    /// if all error flags are to be reset.
    ///
    /// Initially, all error flags are set to [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR).
    ///
    /// The following errors are currently defined:
    ///
    /// [`GL_NO_ERROR`](crate::enums::GL_NO_ERROR)
    ///
    /// > No error has been recorded. The value of this symbolic constant is guaranteed
    /// > to be 0.
    ///
    /// [`GL_INVALID_ENUM`](crate::enums::GL_INVALID_ENUM)
    ///
    /// > An unacceptable value is specified for an enumerated argument. The offending
    /// > command is ignored and has no other side effect than to set the error flag.
    ///
    /// [`GL_INVALID_VALUE`](crate::enums::GL_INVALID_VALUE)
    ///
    /// > A numeric argument is out of range. The offending command is ignored and
    /// > has no other side effect than to set the error flag.
    ///
    /// [`GL_INVALID_OPERATION`](crate::enums::GL_INVALID_OPERATION)
    ///
    /// > The specified operation is not allowed in the current state. The offending
    /// > command is ignored and has no other side effect than to set the error flag.
    ///
    /// [`GL_INVALID_FRAMEBUFFER_OPERATION`](crate::enums::GL_INVALID_FRAMEBUFFER_OPERATION)
    ///
    /// > The framebuffer object is not complete. The offending command is ignored
    /// > and has no other side effect than to set the error flag.
    ///
    /// [`GL_OUT_OF_MEMORY`](crate::enums::GL_OUT_OF_MEMORY)
    ///
    /// > There is not enough memory left to execute the command. The state of the
    /// > GL is undefined, except for the state of the error flags, after this error
    /// > is recorded.
    ///
    /// [`GL_STACK_UNDERFLOW`](crate::enums::GL_STACK_UNDERFLOW)
    ///
    /// > An attempt has been made to perform an operation that would cause an internal
    /// > stack to underflow.
    ///
    /// [`GL_STACK_OVERFLOW`](crate::enums::GL_STACK_OVERFLOW)
    ///
    /// > An attempt has been made to perform an operation that would cause an internal
    /// > stack to overflow.
    ///
    /// When an error flag is set, results of a GL operation are undefined only
    /// if [`GL_OUT_OF_MEMORY`](crate::enums::GL_OUT_OF_MEMORY) has occurred. In
    /// all other cases, the command generating the error is ignored and has no
    /// effect on the GL state or frame buffer contents. If the generating command
    /// returns a value, it returns 0. If [**glGetError**](crate::context::Context::oxidegl_get_error)
    /// itself generates an error, it returns 0.
    pub(crate) fn oxidegl_get_error(&mut self) -> GLenum {
        let r = self.gl_state.error;
        self.gl_state.error = ErrorCode::NoError;
        r.into()
    }
}
