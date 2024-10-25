use crate::{
    context::{
        debug::{gl_debug, gl_trace},
        shader::{GlslShaderInternal, Shader, ShaderInternal},
        state::ObjectName,
        Context,
    },
    dispatch::gl_types::{GLchar, GLint, GLsizei, GLuint},
    enums::{ShaderParameterName, ShaderType},
};

impl Context {
    /// ### Parameters
    /// `shaderType`
    ///
    /// > Specifies the type of shader to be created. Must be one of [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER),
    /// > [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER), [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER),
    /// > [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER),
    /// > [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER), or [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER).
    ///
    /// ### Description
    /// [**glCreateShader**](crate::context::Context::oxidegl_create_shader) creates
    /// an empty shader object and returns a non-zero value by which it can be
    /// referenced. A shader object is used to maintain the source code strings
    /// that define a shader. `shaderType` indicates the type of shader to be created.
    /// Five types of shader are supported. A shader of type [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER)
    /// is a shader that is intended to run on the programmable compute processor.
    /// A shader of type [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER) is
    /// a shader that is intended to run on the programmable vertex processor.
    /// A shader of type [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER)
    /// is a shader that is intended to run on the programmable tessellation processor
    /// in the control stage. A shader of type [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER)
    /// is a shader that is intended to run on the programmable tessellation processor
    /// in the evaluation stage. A shader of type [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER)
    /// is a shader that is intended to run on the programmable geometry processor.
    /// A shader of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER)
    /// is a shader that is intended to run on the programmable fragment processor.
    ///
    /// When created, a shader object's [`GL_SHADER_TYPE`](crate::enums::GL_SHADER_TYPE)
    /// parameter is set to either [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER),
    /// [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER), [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER),
    /// [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER),
    /// [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER) or [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER),
    /// depending on the value of `shaderType`.
    ///
    /// ### Notes
    /// Like buffer and texture objects, the name space for shader objects may
    /// be shared across a set of contexts, as long as the server sides of the
    /// contexts share the same address space. If the name space is shared across
    /// contexts, any attached objects and the data associated with those attached
    /// objects are shared as well.
    ///
    /// Applications are responsible for providing the synchronization across API
    /// calls when objects are accessed from different execution threads.
    ///
    /// [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER) is available only
    /// if the GL version is 4.3 or higher.
    ///
    /// ### Associated Gets
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with a valid
    /// shader object and the parameter to be queried
    ///
    /// [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log)
    /// with a valid shader object
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    /// with a valid shader object
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)
    pub fn oxidegl_create_shader(&mut self, r#type: ShaderType) -> GLuint {
        self.gl_state
            .shader_list
            .new_obj(|name| Shader::new_text_default(name, r#type))
            .to_raw()
    }
    /// ### Parameters
    /// `shader`
    ///
    /// > Specifies the handle of the shader object whose source code is to be replaced.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements in the `string` and `length` arrays.
    ///
    /// `string`
    ///
    /// > Specifies an array of pointers to strings containing the source code to
    /// > be loaded into the shader.
    ///
    /// `length`
    ///
    /// > Specifies an array of string lengths.
    ///
    /// ### Description
    /// [**glShaderSource**](crate::context::Context::oxidegl_shader_source) sets
    /// the source code in `shader` to the source code in the array of strings
    /// specified by `string`. Any source code previously stored in the shader
    /// object is completely replaced. The number of strings in the array is specified
    /// by `count`. If `length` is [`NULL`](crate::enums::NULL), each string is
    /// assumed to be null terminated. If `length` is a value other than [`NULL`](crate::enums::NULL),
    /// it points to an array containing a string length for each of the corresponding
    /// elements of `string`. Each element in the `length` array may contain the
    /// length of the corresponding string (the null character is not counted as
    /// part of the string length) or a value less than 0 to indicate that the
    /// string is null terminated. The source code strings are not scanned or parsed
    /// at this time; they are simply copied into the specified shader object.
    ///
    /// ### Notes
    /// OpenGL copies the shader source code strings when [**glShaderSource**](crate::context::Context::oxidegl_shader_source)
    /// is called, so an application may free its copy of the source code strings
    /// immediately after the function returns.
    ///
    /// ### Associated Gets
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with arguments
    /// `shader` and [`GL_SHADER_SOURCE_LENGTH`](crate::enums::GL_SHADER_SOURCE_LENGTH)
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    /// with argument `shader`
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)
    ///

    pub unsafe fn oxidegl_shader_source(
        &mut self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        // Safety: Caller ensures array length is correct and shader strings are valid
        let sources = unsafe { core::slice::from_raw_parts(string, count as usize) };

        let lengths = if length.is_null() {
            None
        } else {
            // Safety: Caller ensures array length and pointer are correct.
            Some(unsafe { core::slice::from_raw_parts(length, count as usize) })
        };
        let mut strings = Vec::with_capacity(count as usize);
        for (i, &string) in sources.iter().enumerate() {
            let len = lengths.map(|r| r[i]).and_then(|v| u32::try_from(v).ok());
            let str = if let Some(len) = len {
                // Safety: caller ensures string points to an initialized slice of bytes with length len
                core::str::from_utf8(unsafe {
                    core::slice::from_raw_parts(string.cast(), len as usize)
                })
                .expect("Shader source contained non-UTF8 character!")
            } else {
                // Safety: caller ensures string points to a valid and null-terminated C string
                let cstr = unsafe { std::ffi::CStr::from_ptr(string.cast()) };
                cstr.to_str()
                    .expect("Shader source contained non-UTF8 character!")
            };
            strings.push(str);
        }
        let shader = self.get_shader_raw_mut(shader);
        let ShaderInternal::Glsl(GlslShaderInternal { source: s, .. }) = &mut shader.internal
        else {
            panic!("UB: tried to write text source to a non-GLSL shader object")
        };
        s.clear();
        for str in strings {
            s.push_str(str);
        }
        gl_trace!(src: ShaderCompiler, "set source of {:?} to:\n{}", shader.name, s);
    }
    /// ### Parameters
    /// `shader`
    ///
    /// > Specifies the shader object to be compiled.
    ///
    /// ### Description
    /// [**glCompileShader**](crate::context::Context::oxidegl_compile_shader)
    /// compiles the source code strings that have been stored in the shader object
    /// specified by `shader`.
    ///
    /// The compilation status will be stored as part of the shader object's state.
    /// This value will be set to [`GL_TRUE`](crate::enums::GL_TRUE) if the shader
    /// was compiled without errors and is ready for use, and [`GL_FALSE`](crate::enums::GL_FALSE)
    /// otherwise. It can be queried by calling [**glGetShader**](crate::context::Context::oxidegl_get_shader)
    /// with arguments `shader` and [`GL_COMPILE_STATUS`](crate::enums::GL_COMPILE_STATUS).
    ///
    /// Compilation of a shader can fail for a number of reasons as specified by
    /// the OpenGL Shading Language Specification. Whether or not the compilation
    /// was successful, information about the compilation can be obtained from
    /// the shader object's information log by calling [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log).
    ///
    /// ### Associated Gets
    /// [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log)
    /// with argument `shader`
    ///
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with arguments
    /// `shader` and [`GL_COMPILE_STATUS`](crate::enums::GL_COMPILE_STATUS)
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)

    pub fn oxidegl_compile_shader(&mut self, shader: GLuint) {
        self.get_shader_raw_mut(shader).compile();
    }
    /// ### Parameters
    /// `shader`
    ///
    /// > Specifies the shader object to be queried.
    ///
    /// `pname`
    ///
    /// > Specifies the object parameter. Accepted symbolic names are [`GL_SHADER_TYPE`](crate::enums::GL_SHADER_TYPE),
    /// > [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS), [`GL_COMPILE_STATUS`](crate::enums::GL_COMPILE_STATUS),
    /// > [`GL_INFO_LOG_LENGTH`](crate::enums::GL_INFO_LOG_LENGTH), [`GL_SHADER_SOURCE_LENGTH`](crate::enums::GL_SHADER_SOURCE_LENGTH).
    ///
    /// `params`
    ///
    /// > Returns the requested object parameter.
    ///
    /// ### Description
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) returns
    /// in `params` the value of a parameter for a specific shader object. The
    /// following parameters are defined:
    ///
    /// [`GL_SHADER_TYPE`](crate::enums::GL_SHADER_TYPE)
    ///
    /// > `params` returns [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER) if
    /// > `shader` is a vertex shader object, [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER)
    /// > if `shader` is a geometry shader object, and [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER)
    /// > if `shader` is a fragment shader object.
    ///
    /// [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS)
    ///
    /// > `params` returns [`GL_TRUE`](crate::enums::GL_TRUE) if `shader` is currently
    /// > flagged for deletion, and [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
    ///
    /// [`GL_COMPILE_STATUS`](crate::enums::GL_COMPILE_STATUS)
    ///
    /// > `params` returns [`GL_TRUE`](crate::enums::GL_TRUE) if the last compile
    /// > operation on `shader` was successful, and [`GL_FALSE`](crate::enums::GL_FALSE)
    /// > otherwise.
    ///
    /// [`GL_INFO_LOG_LENGTH`](crate::enums::GL_INFO_LOG_LENGTH)
    ///
    /// > `params` returns the number of characters in the information log for `shader`
    /// > including the null termination character (i.e., the size of the character
    /// > buffer required to store the information log). If `shader` has no information
    /// > log, a value of 0 is returned.
    ///
    /// [`GL_SHADER_SOURCE_LENGTH`](crate::enums::GL_SHADER_SOURCE_LENGTH)
    ///
    /// > `params` returns the length of the concatenation of the source strings
    /// > that make up the shader source for the `shader`, including the null termination
    /// > character. (i.e., the size of the character buffer required to store the
    /// > shader source). If no source code exists, 0 is returned.
    ///
    /// ### Notes
    /// If an error is generated, no change is made to the contents of `params`.
    ///
    /// ### Associated Gets
    /// [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log)
    /// with argument `shader`
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    /// with argument `shader`
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)

    pub unsafe fn oxidegl_get_shaderiv(
        &mut self,
        shader: GLuint,
        pname: ShaderParameterName,
        params: *mut GLint,
    ) {
        // TODO come up with a way to handle shader refcounting/delete status
        let shader = self.get_shader_raw_mut(shader);
        // if someone is trying to compile a >4gb shader we have bigger problems
        #[allow(clippy::cast_possible_truncation)]
        let ret: u32 = match pname {
            ShaderParameterName::ShaderType => shader.stage.into(),
            ShaderParameterName::DeleteStatus => u32::from(false),
            ShaderParameterName::CompileStatus => u32::from(shader.internal.compile_status()),
            ShaderParameterName::InfoLogLength => shader.compiler_log.len() as u32,
            ShaderParameterName::ShaderSourceLength => shader.internal.source_len(),
        };
        // Safety: caller ensures params pointer is correct
        unsafe { *params.cast() = ret };
    }
    /// ### Parameters
    /// `shader`
    ///
    /// > Specifies the shader object to be deleted.
    ///
    /// ### Description
    /// [**glDeleteShader**](crate::context::Context::oxidegl_delete_shader) frees
    /// the memory and invalidates the name associated with the shader object specified
    /// by `shader`. This command effectively undoes the effects of a call to [**glCreateShader**](crate::context::Context::oxidegl_create_shader).
    ///
    /// If a shader object to be deleted is attached to a program object, it will
    /// be flagged for deletion, but it will not be deleted until it is no longer
    /// attached to any program object, for any rendering context (i.e., it must
    /// be detached from wherever it was attached before it will be deleted). A
    /// value of 0 for `shader` will be silently ignored.
    ///
    /// To determine whether an object has been flagged for deletion, call [**glGetShader**](crate::context::Context::oxidegl_get_shader)
    /// with arguments `shader` and [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS).
    ///
    /// ### Associated Gets
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with the program object to be queried
    ///
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with arguments
    /// `shader` and [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS)
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)

    pub fn oxidegl_delete_shader(&mut self, shader: GLuint) {
        let name = ObjectName::from_raw(shader);
        gl_debug!(src: ShaderCompiler, "marking {:?} for deletion", name);
        self.gl_state.shaders_to_delete.insert(name);
    }
}

impl Context {
    //TODO: replace with gl_state.shader_list.get_raw_mut()
    pub(crate) fn get_shader_raw_mut(&mut self, shader: GLuint) -> &mut Shader {
        self.gl_state
            .shader_list
            .get_opt_mut(ObjectName::from_raw(shader))
            .expect("tried to compile a nonexistent shader")
    }
}
