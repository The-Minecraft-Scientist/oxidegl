use crate::{
    context::Context,
    dispatch::gl_types::{GLdouble, GLfloat, GLint},
    enums::ClearBufferMask,
};

impl Context {
    /// ### Parameters
    /// `mask`
    ///
    /// > Bitwise OR of masks that indicate the buffers to be cleared. The three
    /// > masks are [`GL_COLOR_BUFFER_BIT`](crate::enums::GL_COLOR_BUFFER_BIT), [`GL_DEPTH_BUFFER_BIT`](crate::enums::GL_DEPTH_BUFFER_BIT),
    /// > and [`GL_STENCIL_BUFFER_BIT`](crate::enums::GL_STENCIL_BUFFER_BIT).
    ///
    /// ### Description
    /// [**glClear**](crate::context::Context::oxidegl_clear) sets the bitplane
    /// area of the window to values previously selected by [**glClearColor**](crate::context::Context::oxidegl_clear_color),
    /// [**glClearDepth**](crate::context::Context::oxidegl_clear_depth), and [**glClearStencil**](crate::context::Context::oxidegl_clear_stencil).
    /// Multiple color buffers can be cleared simultaneously by selecting more
    /// than one buffer at a time using [**glDrawBuffer**](crate::context::Context::oxidegl_draw_buffer).
    ///
    /// The pixel ownership test, the scissor test, dithering, and the buffer writemasks
    /// affect the operation of [**glClear**](crate::context::Context::oxidegl_clear).
    /// The scissor box bounds the cleared region. Alpha function, blend function,
    /// logical operation, stenciling, texture mapping, and depth-buffering are
    /// ignored by [**glClear**](crate::context::Context::oxidegl_clear).
    ///
    /// [**glClear**](crate::context::Context::oxidegl_clear) takes a single argument
    /// that is the bitwise OR of several values indicating which buffer is to
    /// be cleared.
    ///
    /// The values are as follows:
    ///
    /// [`GL_COLOR_BUFFER_BIT`](crate::enums::GL_COLOR_BUFFER_BIT)
    ///
    /// > Indicates the buffers currently enabled for color writing.
    ///
    /// [`GL_DEPTH_BUFFER_BIT`](crate::enums::GL_DEPTH_BUFFER_BIT)
    ///
    /// > Indicates the depth buffer.
    ///
    /// [`GL_STENCIL_BUFFER_BIT`](crate::enums::GL_STENCIL_BUFFER_BIT)
    ///
    /// > Indicates the stencil buffer.
    ///
    /// The value to which each buffer is cleared depends on the setting of the
    /// clear value for that buffer.
    ///
    /// ### Notes
    /// If a buffer is not present, then a [**glClear**](crate::context::Context::oxidegl_clear)
    /// directed at that buffer has no effect.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_DEPTH_CLEAR_VALUE`](crate::enums::GL_DEPTH_CLEAR_VALUE)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_COLOR_CLEAR_VALUE`](crate::enums::GL_COLOR_CLEAR_VALUE)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_STENCIL_CLEAR_VALUE`](crate::enums::GL_STENCIL_CLEAR_VALUE)

    pub fn oxidegl_clear(&mut self, mask: ClearBufferMask) {
        self.gl_state.clear_values.mask = mask;
    }
    /// ### Parameters
    /// `red`
    ///
    /// `green`
    ///
    /// `blue`
    ///
    /// `alpha`
    ///
    /// > Specify the red, green, blue, and alpha values used when the color buffers
    /// > are cleared. The initial values are all 0.
    ///
    /// ### Description
    /// [**glClearColor**](crate::context::Context::oxidegl_clear_color) specifies
    /// the red, green, blue, and alpha values used by [**glClear**](crate::context::Context::oxidegl_clear)
    /// to clear the color buffers. Values specified by [**glClearColor**](crate::context::Context::oxidegl_clear_color)
    /// are clamped to the range `[inlineq]`
    ///
    /// ### Notes
    /// The type of the `red`, `green`, `blue`, and `alpha` parameters was changed
    /// from `GLclampf` to `GLfloat`. This change is transparent to user code and is
    /// described in detail on the [**removedTypes**](crate::context::Context::oxideremoved_types)
    /// page.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_COLOR_CLEAR_VALUE`](crate::enums::GL_COLOR_CLEAR_VALUE)

    pub fn oxidegl_clear_color(
        &mut self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        self.gl_state.clear_values.color = [red, green, blue, alpha];
    }
    /// ### Parameters
    /// `s`
    ///
    /// > Specifies the index used when the stencil buffer is cleared. The initial
    /// > value is 0.
    ///
    /// ### Description
    /// [**glClearStencil**](crate::context::Context::oxidegl_clear_stencil) specifies
    /// the index used by [**glClear**](crate::context::Context::oxidegl_clear)
    /// to clear the stencil buffer. `s` is masked with `[inlineq]` `[inlineq]`
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_STENCIL_CLEAR_VALUE`](crate::enums::GL_STENCIL_CLEAR_VALUE)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_STENCIL_BITS`](crate::enums::GL_STENCIL_BITS)
    #[expect(
        clippy::cast_sign_loss,
        reason = "we want a bitcast anyways, the numeric value doesnt matter all that much"
    )]
    pub fn oxidegl_clear_stencil(&mut self, s: GLint) {
        self.gl_state.clear_values.stencil = s as u32;
    }
}

/// ### Parameters
/// `depth`
///
/// > Specifies the depth value used when the depth buffer is cleared. The initial
/// > value is 1.
///
/// ### Description
/// [**glClearDepth**](crate::context::Context::oxidegl_clear_depth) specifies
/// the depth value used by [**glClear**](crate::context::Context::oxidegl_clear)
/// to clear the depth buffer. Values specified by [**glClearDepth**](crate::context::Context::oxidegl_clear_depth)
/// are clamped to the range `[inlineq]`
///
/// ### Notes
/// The type of the `depth` parameter was changed from `GLclampf` to `GLfloat`
/// for [**glClearDepthf**](crate::context::Context::oxidegl_clear_depthf)
/// and from `GLclampd` to `GLdouble` for [**glClearDepth**](crate::context::Context::oxidegl_clear_depth).
/// This change is transparent to user code and is described in detail on
/// the [**removedTypes**](crate::context::Context::oxideremoved_types) page.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_DEPTH_CLEAR_VALUE`](crate::enums::GL_DEPTH_CLEAR_VALUE)
impl Context {
    // Metal depth buffer is 32 bits so we might as well just truncate here instead of storing an f64
    #[allow(clippy::cast_possible_truncation)]
    pub fn oxidegl_clear_depth(&mut self, depth: GLdouble) {
        self.gl_state.clear_values.depth = depth as f32;
    }
    pub fn oxidegl_clear_depthf(&mut self, d: GLfloat) {
        self.gl_state.clear_values.depth = d;
    }
}
