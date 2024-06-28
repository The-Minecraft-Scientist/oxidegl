/// ### Parameters
/// `pname`
///
/// > Specifies the parameter value to be returned for non-indexed versions of
/// > [**glGet**](crate::context::OxideGLContext::oxidegl_get). The symbolic
/// > constants in the list below are accepted.
///
/// `target`
///
/// > Specifies the parameter value to be returned for indexed versions of [**glGet**](crate::context::OxideGLContext::oxidegl_get).
/// > The symbolic constants in the list below are accepted.
///
/// `index`
///
/// > Specifies the index of the particular element being queried.
///
/// `data`
///
/// > Returns the value or values of the specified parameter.
///
/// ### Description
/// These commands return values for simple state variables in GL. `pname`
/// is a symbolic constant indicating the state variable to be returned, and
/// `data` is a pointer to an array of the indicated type in which to place
/// the returned data.
///
/// Type conversion is performed if `data` has a different type than the state
/// variable value being requested. If [**glGetBooleanv**](crate::context::OxideGLContext::oxidegl_get_booleanv)
/// is called, a floating-point (or integer) value is converted to [`GL_FALSE`](crate::enums::GL_FALSE)
/// if and only if it is 0.0 (or 0). Otherwise, it is converted to [`GL_TRUE`](crate::enums::GL_TRUE).
/// If [**glGetIntegerv**](crate::context::OxideGLContext::oxidegl_get_integerv)
/// is called, boolean values are returned as [`GL_TRUE`](crate::enums::GL_TRUE)
/// or [`GL_FALSE`](crate::enums::GL_FALSE), and most floating-point values
/// are rounded to the nearest integer value. Floating-point colors and normals,
/// however, are returned with a linear mapping that maps 1.0 to the most positive
/// representable integer value and `[inlineq]` [**glGetFloatv**](crate::context::OxideGLContext::oxidegl_get_floatv)
/// or [**glGetDoublev**](crate::context::OxideGLContext::oxidegl_get_doublev)
/// is called, boolean values are returned as [`GL_TRUE`](crate::enums::GL_TRUE)
/// or [`GL_FALSE`](crate::enums::GL_FALSE), and integer values are converted
/// to floating-point values.
///
/// The following symbolic constants are accepted by `pname`:
///
/// [`GL_ACTIVE_TEXTURE`](crate::enums::GL_ACTIVE_TEXTURE)
///
/// > `data` returns a single value indicating the active multitexture unit.
/// > The initial value is [`GL_TEXTURE0`](crate::enums::GL_TEXTURE0). See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_ALIASED_LINE_WIDTH_RANGE`](crate::enums::GL_ALIASED_LINE_WIDTH_RANGE)
///
/// > `data` returns a pair of values indicating the range of widths supported
/// > for aliased lines. See [**glLineWidth**](crate::context::OxideGLContext::oxidegl_line_width).
///
/// [`GL_ARRAY_BUFFER_BINDING`](crate::enums::GL_ARRAY_BUFFER_BINDING)
///
/// > `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. The initial
/// > value is 0. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer).
///
/// [`GL_BLEND`](crate::enums::GL_BLEND)
///
/// > `data` returns a single boolean value indicating whether blending is enabled.
/// > The initial value is [`GL_FALSE`](crate::enums::GL_FALSE). See [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func).
///
/// [`GL_BLEND_COLOR`](crate::enums::GL_BLEND_COLOR)
///
/// > `data` returns four values, the red, green, blue, and alpha values which
/// > are the components of the blend color. See [**glBlendColor**](crate::context::OxideGLContext::oxidegl_blend_color).
///
/// [`GL_BLEND_DST_ALPHA`](crate::enums::GL_BLEND_DST_ALPHA)
///
/// > `data` returns one value, the symbolic constant identifying the alpha destination
/// > blend function. The initial value is [`GL_ZERO`](crate::enums::GL_ZERO).
/// > See [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func)
/// > and [**glBlendFuncSeparate**](crate::context::OxideGLContext::oxidegl_blend_func_separate).
///
/// [`GL_BLEND_DST_RGB`](crate::enums::GL_BLEND_DST_RGB)
///
/// > `data` returns one value, the symbolic constant identifying the RGB destination
/// > blend function. The initial value is [`GL_ZERO`](crate::enums::GL_ZERO).
/// > See [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func)
/// > and [**glBlendFuncSeparate**](crate::context::OxideGLContext::oxidegl_blend_func_separate).
///
/// [`GL_BLEND_EQUATION_RGB`](crate::enums::GL_BLEND_EQUATION_RGB)
///
/// > `data` returns one value, a symbolic constant indicating whether the RGB
/// > blend equation is [`GL_FUNC_ADD`](crate::enums::GL_FUNC_ADD), [`GL_FUNC_SUBTRACT`](crate::enums::GL_FUNC_SUBTRACT),
/// > [`GL_FUNC_REVERSE_SUBTRACT`](crate::enums::GL_FUNC_REVERSE_SUBTRACT), [`GL_MIN`](crate::enums::GL_MIN)
/// > or [`GL_MAX`](crate::enums::GL_MAX). See [**glBlendEquationSeparate**](crate::context::OxideGLContext::oxidegl_blend_equation_separate).
///
/// [`GL_BLEND_EQUATION_ALPHA`](crate::enums::GL_BLEND_EQUATION_ALPHA)
///
/// > `data` returns one value, a symbolic constant indicating whether the Alpha
/// > blend equation is [`GL_FUNC_ADD`](crate::enums::GL_FUNC_ADD), [`GL_FUNC_SUBTRACT`](crate::enums::GL_FUNC_SUBTRACT),
/// > [`GL_FUNC_REVERSE_SUBTRACT`](crate::enums::GL_FUNC_REVERSE_SUBTRACT), [`GL_MIN`](crate::enums::GL_MIN)
/// > or [`GL_MAX`](crate::enums::GL_MAX). See [**glBlendEquationSeparate**](crate::context::OxideGLContext::oxidegl_blend_equation_separate).
///
/// [`GL_BLEND_SRC_ALPHA`](crate::enums::GL_BLEND_SRC_ALPHA)
///
/// > `data` returns one value, the symbolic constant identifying the alpha source
/// > blend function. The initial value is [`GL_ONE`](crate::enums::GL_ONE). See
/// > [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func) and
/// > [**glBlendFuncSeparate**](crate::context::OxideGLContext::oxidegl_blend_func_separate).
///
/// [`GL_BLEND_SRC_RGB`](crate::enums::GL_BLEND_SRC_RGB)
///
/// > `data` returns one value, the symbolic constant identifying the RGB source
/// > blend function. The initial value is [`GL_ONE`](crate::enums::GL_ONE). See
/// > [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func) and
/// > [**glBlendFuncSeparate**](crate::context::OxideGLContext::oxidegl_blend_func_separate).
///
/// [`GL_COLOR_CLEAR_VALUE`](crate::enums::GL_COLOR_CLEAR_VALUE)
///
/// > `data` returns four values: the red, green, blue, and alpha values used
/// > to clear the color buffers. Integer values, if requested, are linearly
/// > mapped from the internal floating-point representation such that 1.0 returns
/// > the most positive representable integer value, and `[inlineq]` [**glClearColor**](crate::context::OxideGLContext::oxidegl_clear_color).
///
/// [`GL_COLOR_LOGIC_OP`](crate::enums::GL_COLOR_LOGIC_OP)
///
/// > `data` returns a single boolean value indicating whether a fragment's RGBA
/// > color values are merged into the framebuffer using a logical operation.
/// > The initial value is [`GL_FALSE`](crate::enums::GL_FALSE). See [**glLogicOp**](crate::context::OxideGLContext::oxidegl_logic_op).
///
/// [`GL_COLOR_WRITEMASK`](crate::enums::GL_COLOR_WRITEMASK)
///
/// > `data` returns four boolean values: the red, green, blue, and alpha write
/// > enables for the color buffers. The initial value is( [`GL_TRUE`](crate::enums::GL_TRUE),
/// > [`GL_TRUE`](crate::enums::GL_TRUE), [`GL_TRUE`](crate::enums::GL_TRUE),
/// > [`GL_TRUE`](crate::enums::GL_TRUE) ). See [**glColorMask**](crate::context::OxideGLContext::oxidegl_color_mask).
///
/// [`GL_COMPRESSED_TEXTURE_FORMATS`](crate::enums::GL_COMPRESSED_TEXTURE_FORMATS)
///
/// > `data` returns a list of symbolic constants of length [`GL_NUM_COMPRESSED_TEXTURE_FORMATS`](crate::enums::GL_NUM_COMPRESSED_TEXTURE_FORMATS)
/// > indicating which compressed texture formats are available. See [**glCompressedTexImage2D**](crate::context::OxideGLContext::oxidegl_compressed_tex_image2_d).
///
/// [`GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a compute shader.
///
/// [`GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum total number of active shader storage
/// > blocks that may be accessed by all active shaders.
///
/// [`GL_MAX_COMPUTE_UNIFORM_BLOCKS`](crate::enums::GL_MAX_COMPUTE_UNIFORM_BLOCKS)
///
/// > `data` returns one value, the maximum number of uniform blocks per compute
/// > shader. The value must be at least 14. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS)
///
/// > `data` returns one value, the maximum supported texture image units that
/// > can be used to access texture maps from the compute shader. The value must
/// > be at least 16. See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_MAX_COMPUTE_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMPUTE_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the maximum number of individual floating-point,
/// > integer, or boolean values that can be held in uniform variable storage
/// > for a compute shader. The value must be at least 1024. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_COMPUTE_ATOMIC_COUNTERS`](crate::enums::GL_MAX_COMPUTE_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to compute shaders.
///
/// [`GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS`](crate::enums::GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS)
///
/// > `data` returns a single value, the maximum number of atomic counter buffers
/// > that may be accessed by a compute shader.
///
/// [`GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the number of words for compute shader uniform
/// > variables in all uniform blocks (including default). The value must be
/// > at least 1. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS)
///
/// > `data` returns one value, the number of invocations in a single local work
/// > group (i.e., the product of the three dimensions) that may be dispatched
/// > to a compute shader.
///
/// [`GL_MAX_COMPUTE_WORK_GROUP_COUNT`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_COUNT)
///
/// > Accepted by the indexed versions of [**glGet**](crate::context::OxideGLContext::oxidegl_get).
/// > `data` the maximum number of work groups that may be dispatched to a compute
/// > shader. Indices 0, 1, and 2 correspond to the X, Y and Z dimensions, respectively.
///
/// [`GL_MAX_COMPUTE_WORK_GROUP_SIZE`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_SIZE)
///
/// > Accepted by the indexed versions of [**glGet**](crate::context::OxideGLContext::oxidegl_get).
/// > `data` the maximum size of a work groups that may be used during compilation
/// > of a compute shader. Indices 0, 1, and 2 correspond to the X, Y and Z dimensions,
/// > respectively.
///
/// [`GL_DISPATCH_INDIRECT_BUFFER_BINDING`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER_BINDING)
///
/// > `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_DISPATCH_INDIRECT_BUFFER`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. The initial
/// > value is 0. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer).
///
/// [`GL_MAX_DEBUG_GROUP_STACK_DEPTH`](crate::enums::GL_MAX_DEBUG_GROUP_STACK_DEPTH)
///
/// > `data` returns a single value, the maximum depth of the debug message group
/// > stack.
///
/// [`GL_DEBUG_GROUP_STACK_DEPTH`](crate::enums::GL_DEBUG_GROUP_STACK_DEPTH)
///
/// > `data` returns a single value, the current depth of the debug message group
/// > stack.
///
/// [`GL_CONTEXT_FLAGS`](crate::enums::GL_CONTEXT_FLAGS)
///
/// > `data` returns one value, the flags with which the context was created
/// > (such as debugging functionality).
///
/// [`GL_CULL_FACE`](crate::enums::GL_CULL_FACE)
///
/// > `data` returns a single boolean value indicating whether polygon culling
/// > is enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE). See
/// > [**glCullFace**](crate::context::OxideGLContext::oxidegl_cull_face).
///
/// [`GL_CULL_FACE_MODE`](crate::enums::GL_CULL_FACE_MODE)
///
/// > `data` returns a single value indicating the mode of polygon culling. The
/// > initial value is [`GL_BACK`](crate::enums::GL_BACK). See [**glCullFace**](crate::context::OxideGLContext::oxidegl_cull_face).
///
/// [`GL_CURRENT_PROGRAM`](crate::enums::GL_CURRENT_PROGRAM)
///
/// > `data` returns one value, the name of the program object that is currently
/// > active, or 0 if no program object is active. See [**glUseProgram**](crate::context::OxideGLContext::oxidegl_use_program).
///
/// [`GL_DEPTH_CLEAR_VALUE`](crate::enums::GL_DEPTH_CLEAR_VALUE)
///
/// > `data` returns one value, the value that is used to clear the depth buffer.
/// > Integer values, if requested, are linearly mapped from the internal floating-point
/// > representation such that 1.0 returns the most positive representable integer
/// > value, and `[inlineq]` [**glClearDepth**](crate::context::OxideGLContext::oxidegl_clear_depth).
///
/// [`GL_DEPTH_FUNC`](crate::enums::GL_DEPTH_FUNC)
///
/// > `data` returns one value, the symbolic constant that indicates the depth
/// > comparison function. The initial value is [`GL_LESS`](crate::enums::GL_LESS).
/// > See [**glDepthFunc**](crate::context::OxideGLContext::oxidegl_depth_func).
///
/// [`GL_DEPTH_RANGE`](crate::enums::GL_DEPTH_RANGE)
///
/// > `data` returns two values: the near and far mapping limits for the depth
/// > buffer. Integer values, if requested, are linearly mapped from the internal
/// > floating-point representation such that 1.0 returns the most positive representable
/// > integer value, and `[inlineq]` [**glDepthRange**](crate::context::OxideGLContext::oxidegl_depth_range).
///
/// [`GL_DEPTH_TEST`](crate::enums::GL_DEPTH_TEST)
///
/// > `data` returns a single boolean value indicating whether depth testing
/// > of fragments is enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glDepthFunc**](crate::context::OxideGLContext::oxidegl_depth_func)
/// > and [**glDepthRange**](crate::context::OxideGLContext::oxidegl_depth_range).
///
/// [`GL_DEPTH_WRITEMASK`](crate::enums::GL_DEPTH_WRITEMASK)
///
/// > `data` returns a single boolean value indicating if the depth buffer is
/// > enabled for writing. The initial value is [`GL_TRUE`](crate::enums::GL_TRUE).
/// > See [**glDepthMask**](crate::context::OxideGLContext::oxidegl_depth_mask).
///
/// [`GL_DITHER`](crate::enums::GL_DITHER)
///
/// > `data` returns a single boolean value indicating whether dithering of fragment
/// > colors and indices is enabled. The initial value is [`GL_TRUE`](crate::enums::GL_TRUE).
///
/// [`GL_DOUBLEBUFFER`](crate::enums::GL_DOUBLEBUFFER)
///
/// > `data` returns a single boolean value indicating whether double buffering
/// > is supported.
///
/// [`GL_DRAW_BUFFER`](crate::enums::GL_DRAW_BUFFER)
///
/// > `data` returns one value, a symbolic constant indicating which buffers
/// > are being drawn to. See [**glDrawBuffer**](crate::context::OxideGLContext::oxidegl_draw_buffer).
/// > The initial value is [`GL_BACK`](crate::enums::GL_BACK) if there are back
/// > buffers, otherwise it is [`GL_FRONT`](crate::enums::GL_FRONT).
///
/// [`GL_DRAW_BUFFER`](crate::enums::GL_DRAW_BUFFER) *i*
///
/// > `data` returns one value, a symbolic constant indicating which buffers
/// > are being drawn to by the corresponding output color. See [**glDrawBuffers**](crate::context::OxideGLContext::oxidegl_draw_buffers).
/// > The initial value of [`GL_DRAW_BUFFER0`](crate::enums::GL_DRAW_BUFFER0)
/// > is [`GL_BACK`](crate::enums::GL_BACK) if there are back buffers, otherwise
/// > it is [`GL_FRONT`](crate::enums::GL_FRONT). The initial values of draw
/// > buffers for all other output colors is [`GL_NONE`](crate::enums::GL_NONE).
///
/// [`GL_DRAW_FRAMEBUFFER_BINDING`](crate::enums::GL_DRAW_FRAMEBUFFER_BINDING)
///
/// > `data` returns one value, the name of the framebuffer object currently
/// > bound to the [`GL_DRAW_FRAMEBUFFER`](crate::enums::GL_DRAW_FRAMEBUFFER)
/// > target. If the default framebuffer is bound, this value will be zero. The
/// > initial value is zero. See [**glBindFramebuffer**](crate::context::OxideGLContext::oxidegl_bind_framebuffer).
///
/// [`GL_READ_FRAMEBUFFER_BINDING`](crate::enums::GL_READ_FRAMEBUFFER_BINDING)
///
/// > `data` returns one value, the name of the framebuffer object currently
/// > bound to the [`GL_READ_FRAMEBUFFER`](crate::enums::GL_READ_FRAMEBUFFER)
/// > target. If the default framebuffer is bound, this value will be zero. The
/// > initial value is zero. See [**glBindFramebuffer**](crate::context::OxideGLContext::oxidegl_bind_framebuffer).
///
/// [`GL_ELEMENT_ARRAY_BUFFER_BINDING`](crate::enums::GL_ELEMENT_ARRAY_BUFFER_BINDING)
///
/// > `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. The initial
/// > value is 0. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer).
///
/// [`GL_FRAGMENT_SHADER_DERIVATIVE_HINT`](crate::enums::GL_FRAGMENT_SHADER_DERIVATIVE_HINT)
///
/// > `data` returns one value, a symbolic constant indicating the mode of the
/// > derivative accuracy hint for fragment shaders. The initial value is [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
/// > See [**glHint**](crate::context::OxideGLContext::oxidegl_hint).
///
/// [`GL_IMPLEMENTATION_COLOR_READ_FORMAT`](crate::enums::GL_IMPLEMENTATION_COLOR_READ_FORMAT)
///
/// > `data` returns a single GLenum value indicating the implementation's preferred
/// > pixel data format. See [**glReadPixels**](crate::context::OxideGLContext::oxidegl_read_pixels).
///
/// [`GL_IMPLEMENTATION_COLOR_READ_TYPE`](crate::enums::GL_IMPLEMENTATION_COLOR_READ_TYPE)
///
/// > `data` returns a single GLenum value indicating the implementation's preferred
/// > pixel data type. See [**glReadPixels**](crate::context::OxideGLContext::oxidegl_read_pixels).
///
/// [`GL_LINE_SMOOTH`](crate::enums::GL_LINE_SMOOTH)
///
/// > `data` returns a single boolean value indicating whether antialiasing of
/// > lines is enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glLineWidth**](crate::context::OxideGLContext::oxidegl_line_width).
///
/// [`GL_LINE_SMOOTH_HINT`](crate::enums::GL_LINE_SMOOTH_HINT)
///
/// > `data` returns one value, a symbolic constant indicating the mode of the
/// > line antialiasing hint. The initial value is [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
/// > See [**glHint**](crate::context::OxideGLContext::oxidegl_hint).
///
/// [`GL_LINE_WIDTH`](crate::enums::GL_LINE_WIDTH)
///
/// > `data` returns one value, the line width as specified with [**glLineWidth**](crate::context::OxideGLContext::oxidegl_line_width).
/// > The initial value is 1.
///
/// [`GL_LAYER_PROVOKING_VERTEX`](crate::enums::GL_LAYER_PROVOKING_VERTEX)
///
/// > `data` returns one value, the implementation dependent specifc vertex of
/// > a primitive that is used to select the rendering layer. If the value returned
/// > is equivalent to [`GL_PROVOKING_VERTEX`](crate::enums::GL_PROVOKING_VERTEX),
/// > then the vertex selection follows the convention specified by [**glProvokingVertex**](crate::context::OxideGLContext::oxidegl_provoking_vertex).
/// > If the value returned is equivalent to [`GL_FIRST_VERTEX_CONVENTION`](crate::enums::GL_FIRST_VERTEX_CONVENTION),
/// > then the selection is always taken from the first vertex in the primitive.
/// > If the value returned is equivalent to [`GL_LAST_VERTEX_CONVENTION`](crate::enums::GL_LAST_VERTEX_CONVENTION),
/// > then the selection is always taken from the last vertex in the primitive.
/// > If the value returned is equivalent to [`GL_UNDEFINED_VERTEX`](crate::enums::GL_UNDEFINED_VERTEX),
/// > then the selection is not guaranteed to be taken from any specific vertex
/// > in the primitive.
///
/// [`GL_LOGIC_OP_MODE`](crate::enums::GL_LOGIC_OP_MODE)
///
/// > `data` returns one value, a symbolic constant indicating the selected logic
/// > operation mode. The initial value is [`GL_COPY`](crate::enums::GL_COPY).
/// > See [**glLogicOp**](crate::context::OxideGLContext::oxidegl_logic_op).
///
/// [`GL_MAJOR_VERSION`](crate::enums::GL_MAJOR_VERSION)
///
/// > `data` returns one value, the major version number of the OpenGL API supported
/// > by the current context.
///
/// [`GL_MAX_3D_TEXTURE_SIZE`](crate::enums::GL_MAX_3D_TEXTURE_SIZE)
///
/// > `data` returns one value, a rough estimate of the largest 3D texture that
/// > the GL can handle. The value must be at least 64. Use [`GL_PROXY_TEXTURE_3D`](crate::enums::GL_PROXY_TEXTURE_3D)
/// > to determine if a texture is too large. See [**glTexImage3D**](crate::context::OxideGLContext::oxidegl_tex_image3_d).
///
/// [`GL_MAX_ARRAY_TEXTURE_LAYERS`](crate::enums::GL_MAX_ARRAY_TEXTURE_LAYERS)
///
/// > `data` returns one value. The value indicates the maximum number of layers
/// > allowed in an array texture, and must be at least 256. See [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
///
/// [`GL_MAX_CLIP_DISTANCES`](crate::enums::GL_MAX_CLIP_DISTANCES)
///
/// > `data` returns one value, the maximum number of application-defined clipping
/// > distances. The value must be at least 8.
///
/// [`GL_MAX_COLOR_TEXTURE_SAMPLES`](crate::enums::GL_MAX_COLOR_TEXTURE_SAMPLES)
///
/// > `data` returns one value, the maximum number of samples in a color multisample
/// > texture.
///
/// [`GL_MAX_COMBINED_ATOMIC_COUNTERS`](crate::enums::GL_MAX_COMBINED_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to all active shaders.
///
/// [`GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the number of words for fragment shader uniform
/// > variables in all uniform blocks (including default). The value must be
/// > at least 1. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the number of words for geometry shader uniform
/// > variables in all uniform blocks (including default). The value must be
/// > at least 1. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS)
///
/// > `data` returns one value, the maximum supported texture image units that
/// > can be used to access texture maps from the vertex shader and the fragment
/// > processor combined. If both the vertex shader and the fragment processing
/// > stage access the same texture image unit, then that counts as using two
/// > texture image units against this limit. The value must be at least 48.
/// > See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_MAX_COMBINED_UNIFORM_BLOCKS`](crate::enums::GL_MAX_COMBINED_UNIFORM_BLOCKS)
///
/// > `data` returns one value, the maximum number of uniform blocks per program.
/// > The value must be at least 70. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the number of words for vertex shader uniform
/// > variables in all uniform blocks (including default). The value must be
/// > at least 1. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_CUBE_MAP_TEXTURE_SIZE`](crate::enums::GL_MAX_CUBE_MAP_TEXTURE_SIZE)
///
/// > `data` returns one value. The value gives a rough estimate of the largest
/// > cube-map texture that the GL can handle. The value must be at least 1024.
/// > Use [`GL_PROXY_TEXTURE_CUBE_MAP`](crate::enums::GL_PROXY_TEXTURE_CUBE_MAP)
/// > to determine if a texture is too large. See [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
///
/// [`GL_MAX_DEPTH_TEXTURE_SAMPLES`](crate::enums::GL_MAX_DEPTH_TEXTURE_SAMPLES)
///
/// > `data` returns one value, the maximum number of samples in a multisample
/// > depth or depth-stencil texture.
///
/// [`GL_MAX_DRAW_BUFFERS`](crate::enums::GL_MAX_DRAW_BUFFERS)
///
/// > `data` returns one value, the maximum number of simultaneous outputs that
/// > may be written in a fragment shader. The value must be at least 8. See
/// > [**glDrawBuffers**](crate::context::OxideGLContext::oxidegl_draw_buffers).
///
/// [`GL_MAX_DUAL_SOURCE_DRAW_BUFFERS`](crate::enums::GL_MAX_DUAL_SOURCE_DRAW_BUFFERS)
///
/// > `data` returns one value, the maximum number of active draw buffers when
/// > using dual-source blending. The value must be at least 1. See [**glBlendFunc**](crate::context::OxideGLContext::oxidegl_blend_func)
/// > and [**glBlendFuncSeparate**](crate::context::OxideGLContext::oxidegl_blend_func_separate).
///
/// [`GL_MAX_ELEMENTS_INDICES`](crate::enums::GL_MAX_ELEMENTS_INDICES)
///
/// > `data` returns one value, the recommended maximum number of vertex array
/// > indices. See [**glDrawRangeElements**](crate::context::OxideGLContext::oxidegl_draw_range_elements).
///
/// [`GL_MAX_ELEMENTS_VERTICES`](crate::enums::GL_MAX_ELEMENTS_VERTICES)
///
/// > `data` returns one value, the recommended maximum number of vertex array
/// > vertices. See [**glDrawRangeElements**](crate::context::OxideGLContext::oxidegl_draw_range_elements).
///
/// [`GL_MAX_FRAGMENT_ATOMIC_COUNTERS`](crate::enums::GL_MAX_FRAGMENT_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to fragment shaders.
///
/// [`GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a fragment shader.
///
/// [`GL_MAX_FRAGMENT_INPUT_COMPONENTS`](crate::enums::GL_MAX_FRAGMENT_INPUT_COMPONENTS)
///
/// > `data` returns one value, the maximum number of components of the inputs
/// > read by the fragment shader, which must be at least 128.
///
/// [`GL_MAX_FRAGMENT_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the maximum number of individual floating-point,
/// > integer, or boolean values that can be held in uniform variable storage
/// > for a fragment shader. The value must be at least 1024. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_FRAGMENT_UNIFORM_VECTORS`](crate::enums::GL_MAX_FRAGMENT_UNIFORM_VECTORS)
///
/// > `data` returns one value, the maximum number of individual 4-vectors of
/// > floating-point, integer, or boolean values that can be held in uniform
/// > variable storage for a fragment shader. The value is equal to the value
/// > of [`GL_MAX_FRAGMENT_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS)
/// > divided by 4 and must be at least 256. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_FRAGMENT_UNIFORM_BLOCKS`](crate::enums::GL_MAX_FRAGMENT_UNIFORM_BLOCKS)
///
/// > `data` returns one value, the maximum number of uniform blocks per fragment
/// > shader. The value must be at least 12. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_MAX_FRAMEBUFFER_WIDTH`](crate::enums::GL_MAX_FRAMEBUFFER_WIDTH)
///
/// > `data` returns one value, the maximum width for a framebuffer that has
/// > no attachments, which must be at least 16384. See [**glFramebufferParameter**](crate::context::OxideGLContext::oxidegl_framebuffer_parameter).
///
/// [`GL_MAX_FRAMEBUFFER_HEIGHT`](crate::enums::GL_MAX_FRAMEBUFFER_HEIGHT)
///
/// > `data` returns one value, the maximum height for a framebuffer that has
/// > no attachments, which must be at least 16384. See [**glFramebufferParameter**](crate::context::OxideGLContext::oxidegl_framebuffer_parameter).
///
/// [`GL_MAX_FRAMEBUFFER_LAYERS`](crate::enums::GL_MAX_FRAMEBUFFER_LAYERS)
///
/// > `data` returns one value, the maximum number of layers for a framebuffer
/// > that has no attachments, which must be at least 2048. See [**glFramebufferParameter**](crate::context::OxideGLContext::oxidegl_framebuffer_parameter).
///
/// [`GL_MAX_FRAMEBUFFER_SAMPLES`](crate::enums::GL_MAX_FRAMEBUFFER_SAMPLES)
///
/// > `data` returns one value, the maximum samples in a framebuffer that has
/// > no attachments, which must be at least 4. See [**glFramebufferParameter**](crate::context::OxideGLContext::oxidegl_framebuffer_parameter).
///
/// [`GL_MAX_GEOMETRY_ATOMIC_COUNTERS`](crate::enums::GL_MAX_GEOMETRY_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to geometry shaders.
///
/// [`GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a geometry shader.
///
/// [`GL_MAX_GEOMETRY_INPUT_COMPONENTS`](crate::enums::GL_MAX_GEOMETRY_INPUT_COMPONENTS)
///
/// > `data` returns one value, the maximum number of components of inputs read
/// > by a geometry shader, which must be at least 64.
///
/// [`GL_MAX_GEOMETRY_OUTPUT_COMPONENTS`](crate::enums::GL_MAX_GEOMETRY_OUTPUT_COMPONENTS)
///
/// > `data` returns one value, the maximum number of components of outputs written
/// > by a geometry shader, which must be at least 128.
///
/// [`GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS)
///
/// > `data` returns one value, the maximum supported texture image units that
/// > can be used to access texture maps from the geometry shader. The value
/// > must be at least 16. See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_MAX_GEOMETRY_UNIFORM_BLOCKS`](crate::enums::GL_MAX_GEOMETRY_UNIFORM_BLOCKS)
///
/// > `data` returns one value, the maximum number of uniform blocks per geometry
/// > shader. The value must be at least 12. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_MAX_GEOMETRY_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the maximum number of individual floating-point,
/// > integer, or boolean values that can be held in uniform variable storage
/// > for a geometry shader. The value must be at least 1024. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_INTEGER_SAMPLES`](crate::enums::GL_MAX_INTEGER_SAMPLES)
///
/// > `data` returns one value, the maximum number of samples supported in integer
/// > format multisample buffers.
///
/// [`GL_MIN_MAP_BUFFER_ALIGNMENT`](crate::enums::GL_MIN_MAP_BUFFER_ALIGNMENT)
///
/// > `data` returns one value, the minimum alignment in basic machine units
/// > of pointers returned from [**glMapBuffer**](crate::context::OxideGLContext::oxidegl_map_buffer)
/// > and [**glMapBufferRange**](crate::context::OxideGLContext::oxidegl_map_buffer_range).
/// > This value must be a power of two and must be at least 64.
///
/// [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH)
///
/// > `data` returns one value, the maximum length of a label that may be assigned
/// > to an object. See [**glObjectLabel**](crate::context::OxideGLContext::oxidegl_object_label)
/// > and [**glObjectPtrLabel**](crate::context::OxideGLContext::oxidegl_object_ptr_label).
///
/// [`GL_MAX_PROGRAM_TEXEL_OFFSET`](crate::enums::GL_MAX_PROGRAM_TEXEL_OFFSET)
///
/// > `data` returns one value, the maximum texel offset allowed in a texture
/// > lookup, which must be at least 7.
///
/// [`GL_MIN_PROGRAM_TEXEL_OFFSET`](crate::enums::GL_MIN_PROGRAM_TEXEL_OFFSET)
///
/// > `data` returns one value, the minimum texel offset allowed in a texture
/// > lookup, which must be at most -8.
///
/// [`GL_MAX_RECTANGLE_TEXTURE_SIZE`](crate::enums::GL_MAX_RECTANGLE_TEXTURE_SIZE)
///
/// > `data` returns one value. The value gives a rough estimate of the largest
/// > rectangular texture that the GL can handle. The value must be at least
/// > 1024. Use [`GL_PROXY_TEXTURE_RECTANGLE`](crate::enums::GL_PROXY_TEXTURE_RECTANGLE)
/// > to determine if a texture is too large. See [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
///
/// [`GL_MAX_RENDERBUFFER_SIZE`](crate::enums::GL_MAX_RENDERBUFFER_SIZE)
///
/// > `data` returns one value. The value indicates the maximum supported size
/// > for renderbuffers. See [**glFramebufferRenderbuffer**](crate::context::OxideGLContext::oxidegl_framebuffer_renderbuffer).
///
/// [`GL_MAX_SAMPLE_MASK_WORDS`](crate::enums::GL_MAX_SAMPLE_MASK_WORDS)
///
/// > `data` returns one value, the maximum number of sample mask words.
///
/// [`GL_MAX_SERVER_WAIT_TIMEOUT`](crate::enums::GL_MAX_SERVER_WAIT_TIMEOUT)
///
/// > `data` returns one value, the maximum [**glWaitSync**](crate::context::OxideGLContext::oxidegl_wait_sync)
/// > timeout interval.
///
/// [`GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS`](crate::enums::GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS)
///
/// > `data` returns one value, the maximum number of shader storage buffer binding
/// > points on the context, which must be at least 8.
///
/// [`GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS`](crate::enums::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to tessellation control shaders.
///
/// [`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS`](crate::enums::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to tessellation evaluation shaders.
///
/// [`GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a tessellation control shader.
///
/// [`GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a tessellation evaluation shader.
///
/// [`GL_MAX_TEXTURE_BUFFER_SIZE`](crate::enums::GL_MAX_TEXTURE_BUFFER_SIZE)
///
/// > `data` returns one value. The value gives the maximum number of texels
/// > allowed in the texel array of a texture buffer object. Value must be at
/// > least 65536.
///
/// [`GL_MAX_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_TEXTURE_IMAGE_UNITS)
///
/// > `data` returns one value, the maximum supported texture image units that
/// > can be used to access texture maps from the fragment shader. The value
/// > must be at least 16. See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_MAX_TEXTURE_LOD_BIAS`](crate::enums::GL_MAX_TEXTURE_LOD_BIAS)
///
/// > `data` returns one value, the maximum, absolute value of the texture level-of-detail
/// > bias. The value must be at least 2.0.
///
/// [`GL_MAX_TEXTURE_SIZE`](crate::enums::GL_MAX_TEXTURE_SIZE)
///
/// > `data` returns one value. The value gives a rough estimate of the largest
/// > texture that the GL can handle. The value must be at least 1024. Use a
/// > proxy texture target such as [`GL_PROXY_TEXTURE_1D`](crate::enums::GL_PROXY_TEXTURE_1D)
/// > or [`GL_PROXY_TEXTURE_2D`](crate::enums::GL_PROXY_TEXTURE_2D) to determine
/// > if a texture is too large. See [**glTexImage1D**](crate::context::OxideGLContext::oxidegl_tex_image1_d)
/// > and [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
///
/// [`GL_MAX_UNIFORM_BUFFER_BINDINGS`](crate::enums::GL_MAX_UNIFORM_BUFFER_BINDINGS)
///
/// > `data` returns one value, the maximum number of uniform buffer binding
/// > points on the context, which must be at least 36.
///
/// [`GL_MAX_UNIFORM_BLOCK_SIZE`](crate::enums::GL_MAX_UNIFORM_BLOCK_SIZE)
///
/// > `data` returns one value, the maximum size in basic machine units of a
/// > uniform block, which must be at least 16384.
///
/// [`GL_MAX_UNIFORM_LOCATIONS`](crate::enums::GL_MAX_UNIFORM_LOCATIONS)
///
/// > `data` returns one value, the maximum number of explicitly assignable uniform
/// > locations, which must be at least 1024.
///
/// [`GL_MAX_VARYING_COMPONENTS`](crate::enums::GL_MAX_VARYING_COMPONENTS)
///
/// > `data` returns one value, the number components for varying variables,
/// > which must be at least 60.
///
/// [`GL_MAX_VARYING_VECTORS`](crate::enums::GL_MAX_VARYING_VECTORS)
///
/// > `data` returns one value, the number 4-vectors for varying variables, which
/// > is equal to the value of [`GL_MAX_VARYING_COMPONENTS`](crate::enums::GL_MAX_VARYING_COMPONENTS)
/// > and must be at least 15.
///
/// [`GL_MAX_VARYING_FLOATS`](crate::enums::GL_MAX_VARYING_FLOATS)
///
/// > `data` returns one value, the maximum number of interpolators available
/// > for processing varying variables used by vertex and fragment shaders. This
/// > value represents the number of individual floating-point values that can
/// > be interpolated; varying variables declared as vectors, matrices, and arrays
/// > will all consume multiple interpolators. The value must be at least 32.
///
/// [`GL_MAX_VERTEX_ATOMIC_COUNTERS`](crate::enums::GL_MAX_VERTEX_ATOMIC_COUNTERS)
///
/// > `data` returns a single value, the maximum number of atomic counters available
/// > to vertex shaders.
///
/// [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS)
///
/// > `data` returns one value, the maximum number of 4-component generic vertex
/// > attributes accessible to a vertex shader. The value must be at least 16.
/// > See [**glVertexAttrib**](crate::context::OxideGLContext::oxidegl_vertex_attrib).
///
/// [`GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS)
///
/// > `data` returns one value, the maximum number of active shader storage blocks
/// > that may be accessed by a vertex shader.
///
/// [`GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS)
///
/// > `data` returns one value, the maximum supported texture image units that
/// > can be used to access texture maps from the vertex shader. The value may
/// > be at least 16. See [**glActiveTexture**](crate::context::OxideGLContext::oxidegl_active_texture).
///
/// [`GL_MAX_VERTEX_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_VERTEX_UNIFORM_COMPONENTS)
///
/// > `data` returns one value, the maximum number of individual floating-point,
/// > integer, or boolean values that can be held in uniform variable storage
/// > for a vertex shader. The value must be at least 1024. See [**glUniform**](crate::context::OxideGLContext::oxidegl_uniform).
///
/// [`GL_MAX_VERTEX_UNIFORM_VECTORS`](crate::enums::GL_MAX_VERTEX_UNIFORM_VECTORS)
///
/// > `data` returns one value, the maximum number of 4-vectors that may be held
/// > in uniform variable storage for the vertex shader. The value of [`GL_MAX_VERTEX_UNIFORM_VECTORS`](crate::enums::GL_MAX_VERTEX_UNIFORM_VECTORS)
/// > is equal to the value of [`GL_MAX_VERTEX_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_VERTEX_UNIFORM_COMPONENTS)
/// > and must be at least 256.
///
/// [`GL_MAX_VERTEX_OUTPUT_COMPONENTS`](crate::enums::GL_MAX_VERTEX_OUTPUT_COMPONENTS)
///
/// > `data` returns one value, the maximum number of components of output written
/// > by a vertex shader, which must be at least 64.
///
/// [`GL_MAX_VERTEX_UNIFORM_BLOCKS`](crate::enums::GL_MAX_VERTEX_UNIFORM_BLOCKS)
///
/// > `data` returns one value, the maximum number of uniform blocks per vertex
/// > shader. The value must be at least 12. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_MAX_VIEWPORT_DIMS`](crate::enums::GL_MAX_VIEWPORT_DIMS)
///
/// > `data` returns two values: the maximum supported width and height of the
/// > viewport. These must be at least as large as the visible dimensions of
/// > the display being rendered to. See [**glViewport**](crate::context::OxideGLContext::oxidegl_viewport).
///
/// [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS)
///
/// > `data` returns one value, the maximum number of simultaneous viewports
/// > that are supported. The value must be at least 16. See [**glViewportIndexed**](crate::context::OxideGLContext::oxidegl_viewport_indexed).
///
/// [`GL_MINOR_VERSION`](crate::enums::GL_MINOR_VERSION)
///
/// > `data` returns one value, the minor version number of the OpenGL API supported
/// > by the current context.
///
/// [`GL_NUM_COMPRESSED_TEXTURE_FORMATS`](crate::enums::GL_NUM_COMPRESSED_TEXTURE_FORMATS)
///
/// > `data` returns a single integer value indicating the number of available
/// > compressed texture formats. The minimum value is 4. See [**glCompressedTexImage2D**](crate::context::OxideGLContext::oxidegl_compressed_tex_image2_d).
///
/// [`GL_NUM_EXTENSIONS`](crate::enums::GL_NUM_EXTENSIONS)
///
/// > `data` returns one value, the number of extensions supported by the GL
/// > implementation for the current context. See [**glGetString**](crate::context::OxideGLContext::oxidegl_get_string).
///
/// [`GL_NUM_PROGRAM_BINARY_FORMATS`](crate::enums::GL_NUM_PROGRAM_BINARY_FORMATS)
///
/// > `data` returns one value, the number of program binary formats supported
/// > by the implementation.
///
/// [`GL_NUM_SHADER_BINARY_FORMATS`](crate::enums::GL_NUM_SHADER_BINARY_FORMATS)
///
/// > `data` returns one value, the number of binary shader formats supported
/// > by the implementation. If this value is greater than zero, then the implementation
/// > supports loading binary shaders. If it is zero, then the loading of binary
/// > shaders by the implementation is not supported.
///
/// [`GL_PACK_ALIGNMENT`](crate::enums::GL_PACK_ALIGNMENT)
///
/// > `data` returns one value, the byte alignment used for writing pixel data
/// > to memory. The initial value is 4. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_IMAGE_HEIGHT`](crate::enums::GL_PACK_IMAGE_HEIGHT)
///
/// > `data` returns one value, the image height used for writing pixel data
/// > to memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_LSB_FIRST`](crate::enums::GL_PACK_LSB_FIRST)
///
/// > `data` returns a single boolean value indicating whether single-bit pixels
/// > being written to memory are written first to the least significant bit
/// > of each unsigned byte. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_ROW_LENGTH`](crate::enums::GL_PACK_ROW_LENGTH)
///
/// > `data` returns one value, the row length used for writing pixel data to
/// > memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_SKIP_IMAGES`](crate::enums::GL_PACK_SKIP_IMAGES)
///
/// > `data` returns one value, the number of pixel images skipped before the
/// > first pixel is written into memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_SKIP_PIXELS`](crate::enums::GL_PACK_SKIP_PIXELS)
///
/// > `data` returns one value, the number of pixel locations skipped before
/// > the first pixel is written into memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_SKIP_ROWS`](crate::enums::GL_PACK_SKIP_ROWS)
///
/// > `data` returns one value, the number of rows of pixel locations skipped
/// > before the first pixel is written into memory. The initial value is 0.
/// > See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PACK_SWAP_BYTES`](crate::enums::GL_PACK_SWAP_BYTES)
///
/// > `data` returns a single boolean value indicating whether the bytes of two-byte
/// > and four-byte pixel indices and components are swapped before being written
/// > to memory. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE). See
/// > [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_PIXEL_PACK_BUFFER_BINDING`](crate::enums::GL_PIXEL_PACK_BUFFER_BINDING)
///
/// > `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_PIXEL_PACK_BUFFER`](crate::enums::GL_PIXEL_PACK_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. The initial
/// > value is 0. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer).
///
/// [`GL_PIXEL_UNPACK_BUFFER_BINDING`](crate::enums::GL_PIXEL_UNPACK_BUFFER_BINDING)
///
/// > `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_PIXEL_UNPACK_BUFFER`](crate::enums::GL_PIXEL_UNPACK_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. The initial
/// > value is 0. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer).
///
/// [`GL_POINT_FADE_THRESHOLD_SIZE`](crate::enums::GL_POINT_FADE_THRESHOLD_SIZE)
///
/// > `data` returns one value, the point size threshold for determining the
/// > point size. See [**glPointParameter**](crate::context::OxideGLContext::oxidegl_point_parameter).
///
/// [`GL_PRIMITIVE_RESTART_INDEX`](crate::enums::GL_PRIMITIVE_RESTART_INDEX)
///
/// > `data` returns one value, the current primitive restart index. The initial
/// > value is 0. See [**glPrimitiveRestartIndex**](crate::context::OxideGLContext::oxidegl_primitive_restart_index).
///
/// [`GL_PROGRAM_BINARY_FORMATS`](crate::enums::GL_PROGRAM_BINARY_FORMATS)
///
/// > `data` an array of [`GL_NUM_PROGRAM_BINARY_FORMATS`](crate::enums::GL_NUM_PROGRAM_BINARY_FORMATS)
/// > values, indicating the proram binary formats supported by the implementation.
///
/// [`GL_PROGRAM_PIPELINE_BINDING`](crate::enums::GL_PROGRAM_PIPELINE_BINDING)
///
/// > `data` a single value, the name of the currently bound program pipeline
/// > object, or zero if no program pipeline object is bound. See [**glBindProgramPipeline**](crate::context::OxideGLContext::oxidegl_bind_program_pipeline).
///
/// [`GL_PROGRAM_POINT_SIZE`](crate::enums::GL_PROGRAM_POINT_SIZE)
///
/// > `data` returns a single boolean value indicating whether vertex program
/// > point size mode is enabled. If enabled, then the point size is taken from
/// > the shader built-in [**glPointSize**](crate::context::OxideGLContext::oxidegl_point_size).
/// > The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
///
/// [`GL_PROVOKING_VERTEX`](crate::enums::GL_PROVOKING_VERTEX)
///
/// > `data` returns one value, the currently selected provoking vertex convention.
/// > The initial value is [`GL_LAST_VERTEX_CONVENTION`](crate::enums::GL_LAST_VERTEX_CONVENTION).
/// > See [**glProvokingVertex**](crate::context::OxideGLContext::oxidegl_provoking_vertex).
///
/// [`GL_POINT_SIZE`](crate::enums::GL_POINT_SIZE)
///
/// > `data` returns one value, the point size as specified by [**glPointSize**](crate::context::OxideGLContext::oxidegl_point_size).
/// > The initial value is 1.
///
/// [`GL_POINT_SIZE_GRANULARITY`](crate::enums::GL_POINT_SIZE_GRANULARITY)
///
/// > `data` returns one value, the size difference between adjacent supported
/// > sizes for antialiased points. See [**glPointSize**](crate::context::OxideGLContext::oxidegl_point_size).
///
/// [`GL_POINT_SIZE_RANGE`](crate::enums::GL_POINT_SIZE_RANGE)
///
/// > `data` returns two values: the smallest and largest supported sizes for
/// > antialiased points. The smallest size must be at most 1, and the largest
/// > size must be at least 1. See [**glPointSize**](crate::context::OxideGLContext::oxidegl_point_size).
///
/// [`GL_POLYGON_OFFSET_FACTOR`](crate::enums::GL_POLYGON_OFFSET_FACTOR)
///
/// > `data` returns one value, the scaling factor used to determine the variable
/// > offset that is added to the depth value of each fragment generated when
/// > a polygon is rasterized. The initial value is 0. See [**glPolygonOffset**](crate::context::OxideGLContext::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_UNITS`](crate::enums::GL_POLYGON_OFFSET_UNITS)
///
/// > `data` returns one value. This value is multiplied by an implementation-specific
/// > value and then added to the depth value of each fragment generated when
/// > a polygon is rasterized. The initial value is 0. See [**glPolygonOffset**](crate::context::OxideGLContext::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_FILL`](crate::enums::GL_POLYGON_OFFSET_FILL)
///
/// > `data` returns a single boolean value indicating whether polygon offset
/// > is enabled for polygons in fill mode. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPolygonOffset**](crate::context::OxideGLContext::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_LINE`](crate::enums::GL_POLYGON_OFFSET_LINE)
///
/// > `data` returns a single boolean value indicating whether polygon offset
/// > is enabled for polygons in line mode. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPolygonOffset**](crate::context::OxideGLContext::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_POINT`](crate::enums::GL_POLYGON_OFFSET_POINT)
///
/// > `data` returns a single boolean value indicating whether polygon offset
/// > is enabled for polygons in point mode. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPolygonOffset**](crate::context::OxideGLContext::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_SMOOTH`](crate::enums::GL_POLYGON_SMOOTH)
///
/// > `data` returns a single boolean value indicating whether antialiasing of
/// > polygons is enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPolygonMode**](crate::context::OxideGLContext::oxidegl_polygon_mode).
///
/// [`GL_POLYGON_SMOOTH_HINT`](crate::enums::GL_POLYGON_SMOOTH_HINT)
///
/// > `data` returns one value, a symbolic constant indicating the mode of the
/// > polygon antialiasing hint. The initial value is [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
/// > See [**glHint**](crate::context::OxideGLContext::oxidegl_hint).
///
/// [`GL_READ_BUFFER`](crate::enums::GL_READ_BUFFER)
///
/// > `data` returns one value, a symbolic constant indicating which color buffer
/// > is selected for reading. The initial value is [`GL_BACK`](crate::enums::GL_BACK)
/// > if there is a back buffer, otherwise it is [`GL_FRONT`](crate::enums::GL_FRONT).
/// > See [**glReadPixels**](crate::context::OxideGLContext::oxidegl_read_pixels).
///
/// [`GL_RENDERBUFFER_BINDING`](crate::enums::GL_RENDERBUFFER_BINDING)
///
/// > `data` returns a single value, the name of the renderbuffer object currently
/// > bound to the target [`GL_RENDERBUFFER`](crate::enums::GL_RENDERBUFFER).
/// > If no renderbuffer object is bound to this target, 0 is returned. The
/// > initial value is 0. See [**glBindRenderbuffer**](crate::context::OxideGLContext::oxidegl_bind_renderbuffer).
///
/// [`GL_SAMPLE_BUFFERS`](crate::enums::GL_SAMPLE_BUFFERS)
///
/// > `data` returns a single integer value indicating the number of sample buffers
/// > associated with the framebuffer. See [**glSampleCoverage**](crate::context::OxideGLContext::oxidegl_sample_coverage).
///
/// [`GL_SAMPLE_COVERAGE_VALUE`](crate::enums::GL_SAMPLE_COVERAGE_VALUE)
///
/// > `data` returns a single positive floating-point value indicating the current
/// > sample coverage value. See [**glSampleCoverage**](crate::context::OxideGLContext::oxidegl_sample_coverage).
///
/// [`GL_SAMPLE_COVERAGE_INVERT`](crate::enums::GL_SAMPLE_COVERAGE_INVERT)
///
/// > `data` returns a single boolean value indicating if the temporary coverage
/// > value should be inverted. See [**glSampleCoverage**](crate::context::OxideGLContext::oxidegl_sample_coverage).
///
/// [`GL_SAMPLE_MASK_VALUE`](crate::enums::GL_SAMPLE_MASK_VALUE)
///
/// > `params` returns one value indicating the current sample mask value. See
/// > [**glSampleMaski**](crate::context::OxideGLContext::oxidegl_sample_maski).
///
/// [`GL_SAMPLER_BINDING`](crate::enums::GL_SAMPLER_BINDING)
///
/// > `data` returns a single value, the name of the sampler object currently
/// > bound to the active texture unit. The initial value is 0. See [**glBindSampler**](crate::context::OxideGLContext::oxidegl_bind_sampler).
///
/// [`GL_SAMPLES`](crate::enums::GL_SAMPLES)
///
/// > `data` returns a single integer value indicating the coverage mask size.
/// > See [**glSampleCoverage**](crate::context::OxideGLContext::oxidegl_sample_coverage).
///
/// [`GL_SCISSOR_BOX`](crate::enums::GL_SCISSOR_BOX)
///
/// > `data` returns four values: the `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
/// > [**glScissor**](crate::context::OxideGLContext::oxidegl_scissor).
///
/// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST)
///
/// > `data` returns a single boolean value indicating whether scissoring is
/// > enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE). See
/// > [**glScissor**](crate::context::OxideGLContext::oxidegl_scissor).
///
/// [`GL_SHADER_COMPILER`](crate::enums::GL_SHADER_COMPILER)
///
/// > `data` returns a single boolean value indicating whether an online shader
/// > compiler is present in the implementation. All desktop OpenGL implementations
/// > must support online shader compilations, and therefore the value of [`GL_SHADER_COMPILER`](crate::enums::GL_SHADER_COMPILER)
/// > will always be [`GL_TRUE`](crate::enums::GL_TRUE).
///
/// [`GL_SHADER_STORAGE_BUFFER_BINDING`](crate::enums::GL_SHADER_STORAGE_BUFFER_BINDING)
///
/// > When used with non-indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegerv**](crate::context::OxideGLContext::oxidegl_get_integerv)
/// > ), `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_SHADER_STORAGE_BUFFER`](crate::enums::GL_SHADER_STORAGE_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. When used
/// > with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegeri_v**](crate::context::OxideGLContext::oxidegl_get_integeri_v)
/// > ), `data` returns a single value, the name of the buffer object bound to
/// > the indexed shader storage buffer binding points. The initial value is
/// > 0 for all targets. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer),
/// > [**glBindBufferBase**](crate::context::OxideGLContext::oxidegl_bind_buffer_base),
/// > and [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT`](crate::enums::GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT)
///
/// > `data` returns a single value, the minimum required alignment for shader
/// > storage buffer sizes and offset. The initial value is 1. See [**glShaderStorageBlockBinding**](crate::context::OxideGLContext::oxidegl_shader_storage_block_binding).
///
/// [`GL_SHADER_STORAGE_BUFFER_START`](crate::enums::GL_SHADER_STORAGE_BUFFER_START)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the start offset of the binding range
/// > for each indexed shader storage buffer binding. The initial value is 0
/// > for all bindings. See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_SHADER_STORAGE_BUFFER_SIZE`](crate::enums::GL_SHADER_STORAGE_BUFFER_SIZE)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the size of the binding range for each
/// > indexed shader storage buffer binding. The initial value is 0 for all bindings.
/// > See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_SMOOTH_LINE_WIDTH_RANGE`](crate::enums::GL_SMOOTH_LINE_WIDTH_RANGE)
///
/// > `data` returns a pair of values indicating the range of widths supported
/// > for smooth (antialiased) lines. See [**glLineWidth**](crate::context::OxideGLContext::oxidegl_line_width).
///
/// [`GL_SMOOTH_LINE_WIDTH_GRANULARITY`](crate::enums::GL_SMOOTH_LINE_WIDTH_GRANULARITY)
///
/// > `data` returns a single value indicating the level of quantization applied
/// > to smooth line width parameters.
///
/// [`GL_STENCIL_BACK_FAIL`](crate::enums::GL_STENCIL_BACK_FAIL)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken for back-facing polygons when the stencil test fails. The initial
/// > value is [`GL_KEEP`](crate::enums::GL_KEEP). See [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_BACK_FUNC`](crate::enums::GL_STENCIL_BACK_FUNC)
///
/// > `data` returns one value, a symbolic constant indicating what function
/// > is used for back-facing polygons to compare the stencil reference value
/// > with the stencil buffer value. The initial value is [`GL_ALWAYS`](crate::enums::GL_ALWAYS).
/// > See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_BACK_PASS_DEPTH_FAIL`](crate::enums::GL_STENCIL_BACK_PASS_DEPTH_FAIL)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken for back-facing polygons when the stencil test passes, but the depth
/// > test fails. The initial value is [`GL_KEEP`](crate::enums::GL_KEEP). See
/// > [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_BACK_PASS_DEPTH_PASS`](crate::enums::GL_STENCIL_BACK_PASS_DEPTH_PASS)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken for back-facing polygons when the stencil test passes and the depth
/// > test passes. The initial value is [`GL_KEEP`](crate::enums::GL_KEEP). See
/// > [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_BACK_REF`](crate::enums::GL_STENCIL_BACK_REF)
///
/// > `data` returns one value, the reference value that is compared with the
/// > contents of the stencil buffer for back-facing polygons. The initial value
/// > is 0. See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_BACK_VALUE_MASK`](crate::enums::GL_STENCIL_BACK_VALUE_MASK)
///
/// > `data` returns one value, the mask that is used for back-facing polygons
/// > to mask both the stencil reference value and the stencil buffer value before
/// > they are compared. The initial value is all 1's. See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_BACK_WRITEMASK`](crate::enums::GL_STENCIL_BACK_WRITEMASK)
///
/// > `data` returns one value, the mask that controls writing of the stencil
/// > bitplanes for back-facing polygons. The initial value is all 1's. See [**glStencilMaskSeparate**](crate::context::OxideGLContext::oxidegl_stencil_mask_separate).
///
/// [`GL_STENCIL_CLEAR_VALUE`](crate::enums::GL_STENCIL_CLEAR_VALUE)
///
/// > `data` returns one value, the index to which the stencil bitplanes are
/// > cleared. The initial value is 0. See [**glClearStencil**](crate::context::OxideGLContext::oxidegl_clear_stencil).
///
/// [`GL_STENCIL_FAIL`](crate::enums::GL_STENCIL_FAIL)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken when the stencil test fails. The initial value is [`GL_KEEP`](crate::enums::GL_KEEP).
/// > See [**glStencilOp**](crate::context::OxideGLContext::oxidegl_stencil_op).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_FUNC`](crate::enums::GL_STENCIL_FUNC)
///
/// > `data` returns one value, a symbolic constant indicating what function
/// > is used to compare the stencil reference value with the stencil buffer
/// > value. The initial value is [`GL_ALWAYS`](crate::enums::GL_ALWAYS). See
/// > [**glStencilFunc**](crate::context::OxideGLContext::oxidegl_stencil_func).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_PASS_DEPTH_FAIL`](crate::enums::GL_STENCIL_PASS_DEPTH_FAIL)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken when the stencil test passes, but the depth test fails. The initial
/// > value is [`GL_KEEP`](crate::enums::GL_KEEP). See [**glStencilOp**](crate::context::OxideGLContext::oxidegl_stencil_op).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_PASS_DEPTH_PASS`](crate::enums::GL_STENCIL_PASS_DEPTH_PASS)
///
/// > `data` returns one value, a symbolic constant indicating what action is
/// > taken when the stencil test passes and the depth test passes. The initial
/// > value is [`GL_KEEP`](crate::enums::GL_KEEP). See [**glStencilOp**](crate::context::OxideGLContext::oxidegl_stencil_op).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilOpSeparate**](crate::context::OxideGLContext::oxidegl_stencil_op_separate).
///
/// [`GL_STENCIL_REF`](crate::enums::GL_STENCIL_REF)
///
/// > `data` returns one value, the reference value that is compared with the
/// > contents of the stencil buffer. The initial value is 0. See [**glStencilFunc**](crate::context::OxideGLContext::oxidegl_stencil_func).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_TEST`](crate::enums::GL_STENCIL_TEST)
///
/// > `data` returns a single boolean value indicating whether stencil testing
/// > of fragments is enabled. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glStencilFunc**](crate::context::OxideGLContext::oxidegl_stencil_func)
/// > and [**glStencilOp**](crate::context::OxideGLContext::oxidegl_stencil_op).
///
/// [`GL_STENCIL_VALUE_MASK`](crate::enums::GL_STENCIL_VALUE_MASK)
///
/// > `data` returns one value, the mask that is used to mask both the stencil
/// > reference value and the stencil buffer value before they are compared.
/// > The initial value is all 1's. See [**glStencilFunc**](crate::context::OxideGLContext::oxidegl_stencil_func).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilFuncSeparate**](crate::context::OxideGLContext::oxidegl_stencil_func_separate).
///
/// [`GL_STENCIL_WRITEMASK`](crate::enums::GL_STENCIL_WRITEMASK)
///
/// > `data` returns one value, the mask that controls writing of the stencil
/// > bitplanes. The initial value is all 1's. See [**glStencilMask**](crate::context::OxideGLContext::oxidegl_stencil_mask).
/// > This stencil state only affects non-polygons and front-facing polygons.
/// > Back-facing polygons use separate stencil state. See [**glStencilMaskSeparate**](crate::context::OxideGLContext::oxidegl_stencil_mask_separate).
///
/// [`GL_STEREO`](crate::enums::GL_STEREO)
///
/// > `data` returns a single boolean value indicating whether stereo buffers
/// > (left and right) are supported.
///
/// [`GL_SUBPIXEL_BITS`](crate::enums::GL_SUBPIXEL_BITS)
///
/// > `data` returns one value, an estimate of the number of bits of subpixel
/// > resolution that are used to position rasterized geometry in window coordinates.
/// > The value must be at least 4.
///
/// [`GL_TEXTURE_BINDING_1D`](crate::enums::GL_TEXTURE_BINDING_1D)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_1D`](crate::enums::GL_TEXTURE_1D). The initial
/// > value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_1D_ARRAY`](crate::enums::GL_TEXTURE_BINDING_1D_ARRAY)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_1D_ARRAY`](crate::enums::GL_TEXTURE_1D_ARRAY).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_2D`](crate::enums::GL_TEXTURE_BINDING_2D)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_2D`](crate::enums::GL_TEXTURE_2D). The initial
/// > value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_2D_ARRAY`](crate::enums::GL_TEXTURE_BINDING_2D_ARRAY)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_2D_ARRAY`](crate::enums::GL_TEXTURE_2D_ARRAY).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_3D`](crate::enums::GL_TEXTURE_BINDING_3D)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_3D`](crate::enums::GL_TEXTURE_3D). The initial
/// > value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_BUFFER`](crate::enums::GL_TEXTURE_BINDING_BUFFER)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_BUFFER`](crate::enums::GL_TEXTURE_BUFFER). The
/// > initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_CUBE_MAP`](crate::enums::GL_TEXTURE_BINDING_CUBE_MAP)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_CUBE_MAP`](crate::enums::GL_TEXTURE_CUBE_MAP).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_BINDING_RECTANGLE`](crate::enums::GL_TEXTURE_BINDING_RECTANGLE)
///
/// > `data` returns a single value, the name of the texture currently bound
/// > to the target [`GL_TEXTURE_RECTANGLE`](crate::enums::GL_TEXTURE_RECTANGLE).
/// > The initial value is 0. See [**glBindTexture**](crate::context::OxideGLContext::oxidegl_bind_texture).
///
/// [`GL_TEXTURE_COMPRESSION_HINT`](crate::enums::GL_TEXTURE_COMPRESSION_HINT)
///
/// > `data` returns a single value indicating the mode of the texture compression
/// > hint. The initial value is [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
///
/// [`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT`](crate::enums::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT)
///
/// > `data` returns a single value, the minimum required alignment for texture
/// > buffer sizes and offset. The initial value is 1. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_TIMESTAMP`](crate::enums::GL_TIMESTAMP)
///
/// > `data` returns a single value, the 64-bit value of the current GL time.
/// > See [**glQueryCounter**](crate::context::OxideGLContext::oxidegl_query_counter).
///
/// [`GL_TRANSFORM_FEEDBACK_BUFFER_BINDING`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_BINDING)
///
/// > When used with non-indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegerv**](crate::context::OxideGLContext::oxidegl_get_integerv)
/// > ), `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. When used
/// > with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegeri_v**](crate::context::OxideGLContext::oxidegl_get_integeri_v)
/// > ), `data` returns a single value, the name of the buffer object bound to
/// > the indexed transform feedback attribute stream. The initial value is 0
/// > for all targets. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer),
/// > [**glBindBufferBase**](crate::context::OxideGLContext::oxidegl_bind_buffer_base),
/// > and [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_TRANSFORM_FEEDBACK_BUFFER_START`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_START)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the start offset of the binding range
/// > for each transform feedback attribute stream. The initial value is 0 for
/// > all streams. See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_TRANSFORM_FEEDBACK_BUFFER_SIZE`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_SIZE)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the size of the binding range for each
/// > transform feedback attribute stream. The initial value is 0 for all streams.
/// > See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_UNIFORM_BUFFER_BINDING`](crate::enums::GL_UNIFORM_BUFFER_BINDING)
///
/// > When used with non-indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegerv**](crate::context::OxideGLContext::oxidegl_get_integerv)
/// > ), `data` returns a single value, the name of the buffer object currently
/// > bound to the target [`GL_UNIFORM_BUFFER`](crate::enums::GL_UNIFORM_BUFFER).
/// > If no buffer object is bound to this target, 0 is returned. When used
/// > with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegeri_v**](crate::context::OxideGLContext::oxidegl_get_integeri_v)
/// > ), `data` returns a single value, the name of the buffer object bound to
/// > the indexed uniform buffer binding point. The initial value is 0 for all
/// > targets. See [**glBindBuffer**](crate::context::OxideGLContext::oxidegl_bind_buffer),
/// > [**glBindBufferBase**](crate::context::OxideGLContext::oxidegl_bind_buffer_base),
/// > and [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT`](crate::enums::GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT)
///
/// > `data` returns a single value, the minimum required alignment for uniform
/// > buffer sizes and offset. The initial value is 1. See [**glUniformBlockBinding**](crate::context::OxideGLContext::oxidegl_uniform_block_binding).
///
/// [`GL_UNIFORM_BUFFER_SIZE`](crate::enums::GL_UNIFORM_BUFFER_SIZE)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the size of the binding range for each
/// > indexed uniform buffer binding. The initial value is 0 for all bindings.
/// > See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_UNIFORM_BUFFER_START`](crate::enums::GL_UNIFORM_BUFFER_START)
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetInteger64i_v**](crate::context::OxideGLContext::oxidegl_get_integer64i_v)
/// > ), `data` returns a single value, the start offset of the binding range
/// > for each indexed uniform buffer binding. The initial value is 0 for all
/// > bindings. See [**glBindBufferRange**](crate::context::OxideGLContext::oxidegl_bind_buffer_range).
///
/// [`GL_UNPACK_ALIGNMENT`](crate::enums::GL_UNPACK_ALIGNMENT)
///
/// > `data` returns one value, the byte alignment used for reading pixel data
/// > from memory. The initial value is 4. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_IMAGE_HEIGHT`](crate::enums::GL_UNPACK_IMAGE_HEIGHT)
///
/// > `data` returns one value, the image height used for reading pixel data
/// > from memory. The initial is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_LSB_FIRST`](crate::enums::GL_UNPACK_LSB_FIRST)
///
/// > `data` returns a single boolean value indicating whether single-bit pixels
/// > being read from memory are read first from the least significant bit of
/// > each unsigned byte. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_ROW_LENGTH`](crate::enums::GL_UNPACK_ROW_LENGTH)
///
/// > `data` returns one value, the row length used for reading pixel data from
/// > memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_SKIP_IMAGES`](crate::enums::GL_UNPACK_SKIP_IMAGES)
///
/// > `data` returns one value, the number of pixel images skipped before the
/// > first pixel is read from memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_SKIP_PIXELS`](crate::enums::GL_UNPACK_SKIP_PIXELS)
///
/// > `data` returns one value, the number of pixel locations skipped before
/// > the first pixel is read from memory. The initial value is 0. See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_SKIP_ROWS`](crate::enums::GL_UNPACK_SKIP_ROWS)
///
/// > `data` returns one value, the number of rows of pixel locations skipped
/// > before the first pixel is read from memory. The initial value is 0. See
/// > [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_UNPACK_SWAP_BYTES`](crate::enums::GL_UNPACK_SWAP_BYTES)
///
/// > `data` returns a single boolean value indicating whether the bytes of two-byte
/// > and four-byte pixel indices and components are swapped after being read
/// > from memory. The initial value is [`GL_FALSE`](crate::enums::GL_FALSE).
/// > See [**glPixelStore**](crate::context::OxideGLContext::oxidegl_pixel_store).
///
/// [`GL_VERTEX_ARRAY_BINDING`](crate::enums::GL_VERTEX_ARRAY_BINDING)
///
/// > `data` returns a single value, the name of the vertex array object currently
/// > bound to the context. If no vertex array object is bound to the context,
/// > 0 is returned. The initial value is 0. See [**glBindVertexArray**](crate::context::OxideGLContext::oxidegl_bind_vertex_array).
///
/// [`GL_VERTEX_BINDING_DIVISOR`](crate::enums::GL_VERTEX_BINDING_DIVISOR)
///
/// > Accepted by the indexed forms. `data` returns a single integer value representing
/// > the instance step divisor of the first element in the bound buffer's data
/// > store for vertex attribute bound to `index`.
///
/// [`GL_VERTEX_BINDING_OFFSET`](crate::enums::GL_VERTEX_BINDING_OFFSET)
///
/// > Accepted by the indexed forms. `data` returns a single integer value representing
/// > the byte offset of the first element in the bound buffer's data store for
/// > vertex attribute bound to `index`.
///
/// [`GL_VERTEX_BINDING_STRIDE`](crate::enums::GL_VERTEX_BINDING_STRIDE)
///
/// > Accepted by the indexed forms. `data` returns a single integer value representing
/// > the byte offset between the start of each element in the bound buffer's
/// > data store for vertex attribute bound to `index`.
///
/// [`GL_VERTEX_BINDING_BUFFER`](crate::enums::GL_VERTEX_BINDING_BUFFER)
///
/// > Accepted by the indexed forms. `data` returns a single integer value representing
/// > the name of the buffer bound to vertex binding `index`.
///
/// [`GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET`](crate::enums::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET)
///
/// > `data` returns a single integer value containing the maximum offset that
/// > may be added to a vertex binding offset.
///
/// [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS)
///
/// > `data` returns a single integer value containing the maximum number of
/// > vertex buffers that may be bound.
///
/// [`GL_VIEWPORT`](crate::enums::GL_VIEWPORT)
///
/// > When used with non-indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegerv**](crate::context::OxideGLContext::oxidegl_get_integerv)
/// > ), `data` returns four values: the `[inlineq]` `[inlineq]` `[inlineq]`
/// > `[inlineq]` [**glViewport**](crate::context::OxideGLContext::oxidegl_viewport).
///
/// > When used with indexed variants of [**glGet**](crate::context::OxideGLContext::oxidegl_get)
/// > (such as [**glGetIntegeri_v**](crate::context::OxideGLContext::oxidegl_get_integeri_v)
/// > ), `data` returns four values: the `[inlineq]` `[inlineq]` `[inlineq]`
/// > `[inlineq]` [**glViewportIndexedf**](crate::context::OxideGLContext::oxidegl_viewport_indexedf).
///
/// [`GL_VIEWPORT_BOUNDS_RANGE`](crate::enums::GL_VIEWPORT_BOUNDS_RANGE)
///
/// > `data` returns two values, the minimum and maximum viewport bounds range.
/// > The minimum range should be at least \[-32768, 32767\].
///
/// [`GL_VIEWPORT_INDEX_PROVOKING_VERTEX`](crate::enums::GL_VIEWPORT_INDEX_PROVOKING_VERTEX)
///
/// > `data` returns one value, the implementation dependent specifc vertex of
/// > a primitive that is used to select the viewport index. If the value returned
/// > is equivalent to [`GL_PROVOKING_VERTEX`](crate::enums::GL_PROVOKING_VERTEX),
/// > then the vertex selection follows the convention specified by [**glProvokingVertex**](crate::context::OxideGLContext::oxidegl_provoking_vertex).
/// > If the value returned is equivalent to [`GL_FIRST_VERTEX_CONVENTION`](crate::enums::GL_FIRST_VERTEX_CONVENTION),
/// > then the selection is always taken from the first vertex in the primitive.
/// > If the value returned is equivalent to [`GL_LAST_VERTEX_CONVENTION`](crate::enums::GL_LAST_VERTEX_CONVENTION),
/// > then the selection is always taken from the last vertex in the primitive.
/// > If the value returned is equivalent to [`GL_UNDEFINED_VERTEX`](crate::enums::GL_UNDEFINED_VERTEX),
/// > then the selection is not guaranteed to be taken from any specific vertex
/// > in the primitive.
///
/// [`GL_VIEWPORT_SUBPIXEL_BITS`](crate::enums::GL_VIEWPORT_SUBPIXEL_BITS)
///
/// > `data` returns a single value, the number of bits of sub-pixel precision
/// > which the GL uses to interpret the floating point viewport bounds. The
/// > minimum value is 0.
///
/// [`GL_MAX_ELEMENT_INDEX`](crate::enums::GL_MAX_ELEMENT_INDEX)
///
/// > `data` returns a single value, the maximum index that may be specified
/// > during the transfer of generic vertex attributes to the GL.
///
/// Many of the boolean parameters can also be queried more easily using [**glIsEnabled**](crate::context::OxideGLContext::oxidegl_is_enabled).
///
/// ### Notes
/// The following parameters return the associated value for the active texture
/// unit: [`GL_TEXTURE_1D`](crate::enums::GL_TEXTURE_1D), [`GL_TEXTURE_BINDING_1D`](crate::enums::GL_TEXTURE_BINDING_1D),
/// [`GL_TEXTURE_2D`](crate::enums::GL_TEXTURE_2D), [`GL_TEXTURE_BINDING_2D`](crate::enums::GL_TEXTURE_BINDING_2D),
/// [`GL_TEXTURE_3D`](crate::enums::GL_TEXTURE_3D) and [`GL_TEXTURE_BINDING_3D`](crate::enums::GL_TEXTURE_BINDING_3D).
///
/// [`GL_MAX_VIEWPORTS`](crate::enums::GL_MAX_VIEWPORTS), [`GL_VIEWPORT_SUBPIXEL_BITS`](crate::enums::GL_VIEWPORT_SUBPIXEL_BITS),
/// [`GL_VIEWPORT_BOUNDS_RANGE`](crate::enums::GL_VIEWPORT_BOUNDS_RANGE), [`GL_LAYER_PROVOKING_VERTEX`](crate::enums::GL_LAYER_PROVOKING_VERTEX),
/// and [`GL_VIEWPORT_INDEX_PROVOKING_VERTEX`](crate::enums::GL_VIEWPORT_INDEX_PROVOKING_VERTEX)
/// are available only if the GL version is 4.1 or greater.
///
/// [`GL_MAX_VERTEX_ATOMIC_COUNTERS`](crate::enums::GL_MAX_VERTEX_ATOMIC_COUNTERS),
/// [`GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS`](crate::enums::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS),
/// [`GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS`](crate::enums::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS),
/// [`GL_MAX_GEOMETRY_ATOMIC_COUNTERS`](crate::enums::GL_MAX_GEOMETRY_ATOMIC_COUNTERS),
/// [`GL_MAX_FRAMGENT_ATOMIC_COUNTERS`](crate::enums::GL_MAX_FRAMGENT_ATOMIC_COUNTERS),
/// and [`GL_MIN_MAP_BUFFER_ALIGNMENT`](crate::enums::GL_MIN_MAP_BUFFER_ALIGNMENT)
/// are accepted by `pname` only if the GL version is 4.2 or greater.
///
/// [`GL_MAX_ELEMENT_INDEX`](crate::enums::GL_MAX_ELEMENT_INDEX) is accepted
/// by `pname` only if the GL version is 4.3 or greater.
///
/// [`GL_MAX_COMPUTE_UNIFORM_BLOCKS`](crate::enums::GL_MAX_COMPUTE_UNIFORM_BLOCKS),
/// [`GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS`](crate::enums::GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS),
/// [`GL_MAX_COMPUTE_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMPUTE_UNIFORM_COMPONENTS),
/// [`GL_MAX_COMPUTE_ATOMIC_COUNTERS`](crate::enums::GL_MAX_COMPUTE_ATOMIC_COUNTERS),
/// [`GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS`](crate::enums::GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS),
/// [`GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS`](crate::enums::GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS),
/// [`GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS),
/// [`GL_MAX_COMPUTE_WORK_GROUP_COUNT`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_COUNT),
/// and [`GL_MAX_COMPUTE_WORK_GROUP_SIZE`](crate::enums::GL_MAX_COMPUTE_WORK_GROUP_SIZE)
/// and [`GL_DISPATCH_INDIRECT_BUFFER_BINDING`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER_BINDING)
/// are available only if the GL version is 4.3 or greater.
///
/// [`GL_MAX_DEBUG_GROUP_STACK_DEPTH`](crate::enums::GL_MAX_DEBUG_GROUP_STACK_DEPTH),
/// [`GL_DEBUG_GROUP_STACK_DEPTH`](crate::enums::GL_DEBUG_GROUP_STACK_DEPTH)
/// and [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH) are accepted
/// only if the GL version is 4.3 or greater.
///
/// [`GL_MAX_UNIFORM_LOCATIONS`](crate::enums::GL_MAX_UNIFORM_LOCATIONS) is
/// accepted only if the GL version is 4.3 or greater.
///
/// [`GL_MAX_FRAMEBUFFER_WIDTH`](crate::enums::GL_MAX_FRAMEBUFFER_WIDTH), [`GL_MAX_FRAMEBUFFER_HEIGHT`](crate::enums::GL_MAX_FRAMEBUFFER_HEIGHT),
/// [`GL_MAX_FRAMEBUFFER_LAYERS`](crate::enums::GL_MAX_FRAMEBUFFER_LAYERS),
/// and [`GL_MAX_FRAMEBUFFER_SAMPLES`](crate::enums::GL_MAX_FRAMEBUFFER_SAMPLES)
/// are available only if the GL version is 4.3 or greater.
///
/// [`GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS),
/// [`GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS),
/// [`GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS),
/// [`GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS),
/// [`GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS),
/// and [`GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS`](crate::enums::GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS)
/// are available only if the GL version is 4.3 or higher.
///
/// [`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT`](crate::enums::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT)
/// is available only if the GL version is 4.3 or greater.
///
/// [`GL_VERTEX_BINDING_DIVISOR`](crate::enums::GL_VERTEX_BINDING_DIVISOR),
/// [`GL_VERTEX_BINDING_OFFSET`](crate::enums::GL_VERTEX_BINDING_OFFSET), [`GL_VERTEX_BINDING_STRIDE`](crate::enums::GL_VERTEX_BINDING_STRIDE),
/// [`GL_VERTEX_BINDING_BUFFER`](crate::enums::GL_VERTEX_BINDING_BUFFER), [`GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET`](crate::enums::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET)
/// and [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS)
/// are available only if the GL version is 4.3 or greater.
pub mod get;
#[allow(clippy::missing_safety_doc)]
pub mod unimplemented;
