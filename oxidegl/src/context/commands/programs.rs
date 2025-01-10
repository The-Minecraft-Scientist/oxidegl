use crate::{
    context::{debug::gl_debug, gl_object::ObjectName, program::Program, Context},
    dispatch::gl_types::{GLint, GLuint},
    enums::ProgramProperty,
    run_if_changed,
};

impl Context {
    /// ### Description
    /// [**glCreateProgram**](crate::context::Context::oxidegl_create_program)
    /// creates an empty program object and returns a non-zero value by which it
    /// can be referenced. A program object is an object to which shader objects
    /// can be attached. This provides a mechanism to specify the shader objects
    /// that will be linked to create a program. It also provides a means for checking
    /// the compatibility of the shaders that will be used to create a program
    /// (for instance, checking the compatibility between a vertex shader and a
    /// fragment shader). When no longer needed as part of a program object, shader
    /// objects can be detached.
    ///
    /// One or more executables are created in a program object by successfully
    /// attaching shader objects to it with [**glAttachShader**](crate::context::Context::oxidegl_attach_shader),
    /// successfully compiling the shader objects with [**glCompileShader**](crate::context::Context::oxidegl_compile_shader),
    /// and successfully linking the program object with [**glLinkProgram**](crate::context::Context::oxidegl_link_program).
    /// These executables are made part of current state when [**glUseProgram**](crate::context::Context::oxidegl_use_program)
    /// is called. Program objects can be deleted by calling [**glDeleteProgram**](crate::context::Context::oxidegl_delete_program).
    /// The memory associated with the program object will be deleted when it
    /// is no longer part of current rendering state for any context.
    ///
    /// ### Notes
    /// Like buffer and texture objects, the name space for program objects may
    /// be shared across a set of contexts, as long as the server sides of the
    /// contexts share the same address space. If the name space is shared across
    /// contexts, any attached objects and the data associated with those attached
    /// objects are shared as well.
    ///
    /// Applications are responsible for providing the synchronization across API
    /// calls when objects are accessed from different execution threads.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with the argument [`GL_CURRENT_PROGRAM`](crate::enums::GL_CURRENT_PROGRAM)
    ///
    /// [**glGetActiveAttrib**](crate::context::Context::oxidegl_get_active_attrib)
    /// with a valid program object and the index of an active attribute variable
    ///
    /// [**glGetActiveUniform**](crate::context::Context::oxidegl_get_active_uniform)
    /// with a valid program object and the index of an active uniform variable
    ///
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with a valid program object
    ///
    /// [**glGetAttribLocation**](crate::context::Context::oxidegl_get_attrib_location)
    /// with a valid program object and the name of an attribute variable
    ///
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) with a
    /// valid program object and the parameter to be queried
    ///
    /// [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log)
    /// with a valid program object
    ///
    /// [**glGetUniform**](crate::context::Context::oxidegl_get_uniform) with a
    /// valid program object and the location of a uniform variable
    ///
    /// [**glGetUniformLocation**](crate::context::Context::oxidegl_get_uniform_location)
    /// with a valid program object and the name of a uniform variable
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    pub fn oxidegl_create_program(&mut self) -> GLuint {
        self.gl_state
            .program_list
            .new_obj(Program::new_named)
            .to_raw()
    }
    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the handle of the program object to be linked.
    ///
    /// ### Description
    /// [**glLinkProgram**](crate::context::Context::oxidegl_link_program) links
    /// the program object specified by `program`. If any shader objects of type
    /// [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER) are attached to `program`,
    /// they will be used to create an executable that will run on the programmable
    /// vertex processor. If any shader objects of type [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER)
    /// are attached to `program`, they will be used to create an executable that
    /// will run on the programmable geometry processor. If any shader objects
    /// of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER) are attached
    /// to `program`, they will be used to create an executable that will run on
    /// the programmable fragment processor.
    ///
    /// The status of the link operation will be stored as part of the program
    /// object's state. This value will be set to [`GL_TRUE`](crate::enums::GL_TRUE)
    /// if the program object was linked without errors and is ready for use, and
    /// [`GL_FALSE`](crate::enums::GL_FALSE) otherwise. It can be queried by calling
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) with arguments
    /// `program` and [`GL_LINK_STATUS`](crate::enums::GL_LINK_STATUS).
    ///
    /// As a result of a successful link operation, all active user-defined uniform
    /// variables belonging to `program` will be initialized to 0, and each of
    /// the program object's active uniform variables will be assigned a location
    /// that can be queried by calling [**glGetUniformLocation**](crate::context::Context::oxidegl_get_uniform_location).
    /// Also, any active user-defined attribute variables that have not been bound
    /// to a generic vertex attribute index will be bound to one at this time.
    ///
    /// Linking of a program object can fail for a number of reasons as specified
    /// in the *OpenGL Shading Language Specification*. The following lists some
    /// of the conditions that will cause a link error.
    ///
    /// > The number of active attribute variables supported by the implementation
    /// > has been exceeded.
    ///
    /// > The storage limit for uniform variables has been exceeded.
    ///
    /// > The number of active uniform variables supported by the implementation
    /// > has been exceeded.
    ///
    /// > The [**main**](crate::context::Context::oxidemain) function is missing
    /// > for the vertex, geometry or fragment shader.
    ///
    /// > A varying variable actually used in the fragment shader is not declared
    /// > in the same way (or is not declared at all) in the vertex shader, or geometry
    /// > shader if present.
    ///
    /// > A reference to a function or variable name is unresolved.
    ///
    /// > A shared global is declared with two different types or two different initial
    /// > values.
    ///
    /// > One or more of the attached shader objects has not been successfully compiled.
    ///
    /// > Binding a generic attribute matrix caused some rows of the matrix to fall
    /// > outside the allowed maximum of [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS).
    ///
    /// > Not enough contiguous vertex attribute slots could be found to bind attribute
    /// > matrices.
    ///
    /// > The program object contains objects to form a fragment shader but does
    /// > not contain objects to form a vertex shader.
    ///
    /// > The program object contains objects to form a geometry shader but does
    /// > not contain objects to form a vertex shader.
    ///
    /// > The program object contains objects to form a geometry shader and the input
    /// > primitive type, output primitive type, or maximum output vertex count is
    /// > not specified in any compiled geometry shader object.
    ///
    /// > The program object contains objects to form a geometry shader and the input
    /// > primitive type, output primitive type, or maximum output vertex count is
    /// > specified differently in multiple geometry shader objects.
    ///
    /// > The number of active outputs in the fragment shader is greater than the
    /// > value of [`GL_MAX_DRAW_BUFFERS`](crate::enums::GL_MAX_DRAW_BUFFERS).
    ///
    /// > The program has an active output assigned to a location greater than or
    /// > equal to the value of [`GL_MAX_DUAL_SOURCE_DRAW_BUFFERS`](crate::enums::GL_MAX_DUAL_SOURCE_DRAW_BUFFERS)
    /// > and has an active output assigned an index greater than or equal to one.
    ///
    /// > More than one varying out variable is bound to the same number and index.
    ///
    /// > The explicit binding assigments do not leave enough space for the linker
    /// > to automatically assign a location for a varying out array, which requires
    /// > multiple contiguous locations.
    ///
    /// > The `count` specified by [**glTransformFeedbackVaryings**](crate::context::Context::oxidegl_transform_feedback_varyings)
    /// > is non-zero, but the program object has no vertex or geometry shader.
    ///
    /// > Any variable name specified to [**glTransformFeedbackVaryings**](crate::context::Context::oxidegl_transform_feedback_varyings)
    /// > in the `varyings` array is not declared as an output in the vertex shader
    /// > (or the geometry shader, if active).
    ///
    /// > Any two entries in the `varyings` array given [**glTransformFeedbackVaryings**](crate::context::Context::oxidegl_transform_feedback_varyings)
    /// > specify the same varying variable.
    ///
    /// > The total number of components to capture in any transform feedback varying
    /// > variable is greater than the constant [`GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS`](crate::enums::GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS)
    /// > and the buffer mode is [`GL_SEPARATE_ATTRIBS`](crate::enums::GL_SEPARATE_ATTRIBS).
    ///
    /// When a program object has been successfully linked, the program object
    /// can be made part of current state by calling [**glUseProgram**](crate::context::Context::oxidegl_use_program).
    /// Whether or not the link operation was successful, the program object's
    /// information log will be overwritten. The information log can be retrieved
    /// by calling [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log).
    ///
    /// [**glLinkProgram**](crate::context::Context::oxidegl_link_program) will
    /// also install the generated executables as part of the current rendering
    /// state if the link operation was successful and the specified program object
    /// is already currently in use as a result of a previous call to [**glUseProgram**](crate::context::Context::oxidegl_use_program).
    /// If the program object currently in use is relinked unsuccessfully, its
    /// link status will be set to [`GL_FALSE`](crate::enums::GL_FALSE), but the
    /// executables and associated state will remain part of the current state
    /// until a subsequent call to [**glUseProgram**](crate::context::Context::oxidegl_use_program)
    /// removes it from use. After it is removed from use, it cannot be made part
    /// of current state until it has been successfully relinked.
    ///
    /// If `program` contains shader objects of type [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER),
    /// and optionally of type [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER),
    /// but does not contain shader objects of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER),
    /// the vertex shader executable will be installed on the programmable vertex
    /// processor, the geometry shader executable, if present, will be installed
    /// on the programmable geometry processor, but no executable will be installed
    /// on the fragment processor. The results of rasterizing primitives with such
    /// a program will be undefined.
    ///
    /// The program object's information log is updated and the program is generated
    /// at the time of the link operation. After the link operation, applications
    /// are free to modify attached shader objects, compile attached shader objects,
    /// detach shader objects, delete shader objects, and attach additional shader
    /// objects. None of these operations affects the information log or the program
    /// that is part of the program object.
    ///
    /// ### Notes
    /// If the link operation is unsuccessful, any information about a previous
    /// link operation on `program` is lost (i.e., a failed link does not restore
    /// the old state of `program` ). Certain information can still be retrieved
    /// from `program` even after an unsuccessful link operation. See for instance
    /// [**glGetActiveAttrib**](crate::context::Context::oxidegl_get_active_attrib)
    /// and [**glGetActiveUniform**](crate::context::Context::oxidegl_get_active_uniform).
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with the argument [`GL_CURRENT_PROGRAM`](crate::enums::GL_CURRENT_PROGRAM)
    ///
    /// [**glGetActiveAttrib**](crate::context::Context::oxidegl_get_active_attrib)
    /// with argument `program` and the index of an active attribute variable
    ///
    /// [**glGetActiveUniform**](crate::context::Context::oxidegl_get_active_uniform)
    /// with argument `program` and the index of an active uniform variable
    ///
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with argument `program`
    ///
    /// [**glGetAttribLocation**](crate::context::Context::oxidegl_get_attrib_location)
    /// with argument `program` and an attribute variable name
    ///
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) with arguments
    /// `program` and [`GL_LINK_STATUS`](crate::enums::GL_LINK_STATUS)
    ///
    /// [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log)
    /// with argument `program`
    ///
    /// [**glGetUniform**](crate::context::Context::oxidegl_get_uniform) with argument
    /// `program` and a uniform variable location
    ///
    /// [**glGetUniformLocation**](crate::context::Context::oxidegl_get_uniform_location)
    /// with argument `program` and a uniform variable name
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    pub fn oxidegl_link_program(&mut self, program: GLuint) {
        // regenerate program-related state if this program is the currently bound one
        // TODO program pipelines will complicate this
        if self
            .gl_state
            .program_binding
            .is_some_and(|n| n.to_raw() == program)
        {
            self.new_pipeline();
            self.new_encoder();
            self.remap_buffers();
        }

        let program = self.gl_state.program_list.get_raw_mut(program);
        program.link(&mut self.gl_state.shader_list, &self.platform_state.device);
    }
    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the program object to which a shader object will be attached.
    ///
    /// `shader`
    ///
    /// > Specifies the shader object that is to be attached.
    ///
    /// ### Description
    /// In order to create a complete shader program, there must be a way to specify
    /// the list of things that will be linked together. Program objects provide
    /// this mechanism. Shaders that are to be linked together in a program object
    /// must first be attached to that program object. [**glAttachShader**](crate::context::Context::oxidegl_attach_shader)
    /// attaches the shader object specified by `shader` to the program object
    /// specified by `program`. This indicates that `shader` will be included in
    /// link operations that will be performed on `program`.
    ///
    /// All operations that can be performed on a shader object are valid whether
    /// or not the shader object is attached to a program object. It is permissible
    /// to attach a shader object to a program object before source code has been
    /// loaded into the shader object or before the shader object has been compiled.
    /// It is permissible to attach multiple shader objects of the same type because
    /// each may contain a portion of the complete shader. It is also permissible
    /// to attach a shader object to more than one program object. If a shader
    /// object is deleted while it is attached to a program object, it will be
    /// flagged for deletion, and deletion will not occur until [**glDetachShader**](crate::context::Context::oxidegl_detach_shader)
    /// is called to detach it from all program objects to which it is attached.
    ///
    /// ### Associated Gets
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with the handle of a valid program object
    ///
    /// [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log)
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)
    pub fn oxidegl_attach_shader(&mut self, program: GLuint, shader: GLuint) {
        let program = self.gl_state.program_list.get_raw_mut(program);
        program.attach_shader(self.gl_state.shader_list.get_raw_mut(shader));
    }
    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the program object from which to detach the shader object.
    ///
    /// `shader`
    ///
    /// > Specifies the shader object to be detached.
    ///
    /// ### Description
    /// [**glDetachShader**](crate::context::Context::oxidegl_detach_shader) detaches
    /// the shader object specified by `shader` from the program object specified
    /// by `program`. This command can be used to undo the effect of the command
    /// [**glAttachShader**](crate::context::Context::oxidegl_attach_shader).
    ///
    /// If `shader` has already been flagged for deletion by a call to [**glDeleteShader**](crate::context::Context::oxidegl_delete_shader)
    /// and it is not attached to any other program object, it will be deleted
    /// after it has been detached.
    ///
    /// ### Associated Gets
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with the handle of a valid program object
    ///
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with arguments
    /// `shader` and [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS)
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)
    pub fn oxidegl_detach_shader(&mut self, program: GLuint, shader: GLuint) {
        let program = self.gl_state.program_list.get_raw_mut(program);
        let shader = self.gl_state.shader_list.get_raw_mut(shader);
        let name = shader.name;
        if program.detach_shader(shader) && self.gl_state.shader_deletion_queue.contains(&name) {
            self.gl_state.shader_list.delete(name);
            self.gl_state.shader_deletion_queue.remove(&name);
        }
    }
    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the program object to be queried.
    ///
    /// `pname`
    ///
    /// > Specifies the object parameter. Accepted symbolic names are [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS),
    /// > [`GL_LINK_STATUS`](crate::enums::GL_LINK_STATUS), [`GL_VALIDATE_STATUS`](crate::enums::GL_VALIDATE_STATUS),
    /// > [`GL_INFO_LOG_LENGTH`](crate::enums::GL_INFO_LOG_LENGTH), [`GL_ATTACHED_SHADERS`](crate::enums::GL_ATTACHED_SHADERS),
    /// > [`GL_ACTIVE_ATOMIC_COUNTER_BUFFERS`](crate::enums::GL_ACTIVE_ATOMIC_COUNTER_BUFFERS),
    /// > [`GL_ACTIVE_ATTRIBUTES`](crate::enums::GL_ACTIVE_ATTRIBUTES), [`GL_ACTIVE_ATTRIBUTE_MAX_LENGTH`](crate::enums::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH),
    /// > [`GL_ACTIVE_UNIFORMS`](crate::enums::GL_ACTIVE_UNIFORMS), [`GL_ACTIVE_UNIFORM_BLOCKS`](crate::enums::GL_ACTIVE_UNIFORM_BLOCKS),
    /// > [`GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH`](crate::enums::GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH),
    /// > [`GL_ACTIVE_UNIFORM_MAX_LENGTH`](crate::enums::GL_ACTIVE_UNIFORM_MAX_LENGTH),
    /// > [`GL_COMPUTE_WORK_GROUP_SIZE`](crate::enums::GL_COMPUTE_WORK_GROUP_SIZE),
    /// > [`GL_PROGRAM_BINARY_LENGTH`](crate::enums::GL_PROGRAM_BINARY_LENGTH), [`GL_TRANSFORM_FEEDBACK_BUFFER_MODE`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_MODE),
    /// > [`GL_TRANSFORM_FEEDBACK_VARYINGS`](crate::enums::GL_TRANSFORM_FEEDBACK_VARYINGS),
    /// > [`GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH`](crate::enums::GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH),
    /// > [`GL_GEOMETRY_VERTICES_OUT`](crate::enums::GL_GEOMETRY_VERTICES_OUT), [`GL_GEOMETRY_INPUT_TYPE`](crate::enums::GL_GEOMETRY_INPUT_TYPE),
    /// > and [`GL_GEOMETRY_OUTPUT_TYPE`](crate::enums::GL_GEOMETRY_OUTPUT_TYPE).
    ///
    /// `params`
    ///
    /// > Returns the requested object parameter.
    ///
    /// ### Description
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) returns
    /// in `params` the value of a parameter for a specific program object. The
    /// following parameters are defined:
    ///
    /// [`GL_DELETE_STATUS`](crate::enums::GL_DELETE_STATUS)
    ///
    ///
    /// > `params` returns [`GL_TRUE`](crate::enums::GL_TRUE) if `program` is currently
    /// > flagged for deletion, and [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
    ///
    /// [`GL_LINK_STATUS`](crate::enums::GL_LINK_STATUS)
    ///
    ///
    /// > `params` returns [`GL_TRUE`](crate::enums::GL_TRUE) if the last link operation
    /// > on `program` was successful, and [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
    ///
    /// [`GL_VALIDATE_STATUS`](crate::enums::GL_VALIDATE_STATUS)
    ///
    ///
    /// > `params` returns [`GL_TRUE`](crate::enums::GL_TRUE) or if the last validation
    /// > operation on `program` was successful, and [`GL_FALSE`](crate::enums::GL_FALSE)
    /// > otherwise.
    ///
    /// [`GL_INFO_LOG_LENGTH`](crate::enums::GL_INFO_LOG_LENGTH)
    ///
    ///
    /// > `params` returns the number of characters in the information log for `program`
    /// > including the null termination character (i.e., the size of the character
    /// > buffer required to store the information log). If `program` has no information
    /// > log, a value of 0 is returned.
    ///
    /// [`GL_ATTACHED_SHADERS`](crate::enums::GL_ATTACHED_SHADERS)
    ///
    ///
    /// > `params` returns the number of shader objects attached to `program`.
    ///
    /// [`GL_ACTIVE_ATOMIC_COUNTER_BUFFERS`](crate::enums::GL_ACTIVE_ATOMIC_COUNTER_BUFFERS)
    ///
    ///
    /// > `params` returns the number of active attribute atomic counter buffers
    /// > used by `program`.
    ///
    /// [`GL_ACTIVE_ATTRIBUTES`](crate::enums::GL_ACTIVE_ATTRIBUTES)
    ///
    ///
    /// > `params` returns the number of active attribute variables for `program`.
    ///
    /// [`GL_ACTIVE_ATTRIBUTE_MAX_LENGTH`](crate::enums::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH)
    ///
    ///
    /// > `params` returns the length of the longest active attribute name for `program`,
    /// > including the null termination character (i.e., the size of the character
    /// > buffer required to store the longest attribute name). If no active attributes
    /// > exist, 0 is returned.
    ///
    /// [`GL_ACTIVE_UNIFORMS`](crate::enums::GL_ACTIVE_UNIFORMS)
    ///
    ///
    /// > `params` returns the number of active uniform variables for `program`.
    ///
    /// [`GL_ACTIVE_UNIFORM_MAX_LENGTH`](crate::enums::GL_ACTIVE_UNIFORM_MAX_LENGTH)
    ///
    ///
    /// > `params` returns the length of the longest active uniform variable name
    /// > for `program`, including the null termination character (i.e., the size
    /// > of the character buffer required to store the longest uniform variable
    /// > name). If no active uniform variables exist, 0 is returned.
    ///
    /// [`GL_PROGRAM_BINARY_LENGTH`](crate::enums::GL_PROGRAM_BINARY_LENGTH)
    ///
    ///
    /// > `params` returns the length of the program binary, in bytes that will be
    /// > returned by a call to [**glGetProgramBinary**](crate::context::Context::oxidegl_get_program_binary).
    /// > When a progam's [`GL_LINK_STATUS`](crate::enums::GL_LINK_STATUS) is [`GL_FALSE`](crate::enums::GL_FALSE),
    /// > its program binary length is zero.
    ///
    /// [`GL_COMPUTE_WORK_GROUP_SIZE`](crate::enums::GL_COMPUTE_WORK_GROUP_SIZE)
    ///
    ///
    /// > `params` returns an array of three integers containing the local work group
    /// > size of the compute program as specified by its input layout qualifier(s).
    /// > `program` must be the name of a program object that has been previously
    /// > linked successfully and contains a binary for the compute shader stage.
    ///
    /// [`GL_TRANSFORM_FEEDBACK_BUFFER_MODE`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_MODE)
    ///
    ///
    /// > `params` returns a symbolic constant indicating the buffer mode used when
    /// > transform feedback is active. This may be [`GL_SEPARATE_ATTRIBS`](crate::enums::GL_SEPARATE_ATTRIBS)
    /// > or [`GL_INTERLEAVED_ATTRIBS`](crate::enums::GL_INTERLEAVED_ATTRIBS).
    ///
    /// [`GL_TRANSFORM_FEEDBACK_VARYINGS`](crate::enums::GL_TRANSFORM_FEEDBACK_VARYINGS)
    ///
    ///
    /// > `params` returns the number of varying variables to capture in transform
    /// > feedback mode for the program.
    ///
    /// [`GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH`](crate::enums::GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH)
    ///
    ///
    /// > `params` returns the length of the longest variable name to be used for
    /// > transform feedback, including the null-terminator.
    ///
    /// [`GL_GEOMETRY_VERTICES_OUT`](crate::enums::GL_GEOMETRY_VERTICES_OUT)
    ///
    ///
    /// > `params` returns the maximum number of vertices that the geometry shader
    /// > in `program` will output.
    ///
    /// [`GL_GEOMETRY_INPUT_TYPE`](crate::enums::GL_GEOMETRY_INPUT_TYPE)
    ///
    ///
    /// > `params` returns a symbolic constant indicating the primitive type accepted
    /// > as input to the geometry shader contained in `program`.
    ///
    /// [`GL_GEOMETRY_OUTPUT_TYPE`](crate::enums::GL_GEOMETRY_OUTPUT_TYPE)
    ///
    ///
    /// > `params` returns a symbolic constant indicating the primitive type that
    /// > will be output by the geometry shader contained in `program`.
    ///
    /// ### Notes
    /// [`GL_ACTIVE_UNIFORM_BLOCKS`](crate::enums::GL_ACTIVE_UNIFORM_BLOCKS) and
    /// [`GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH`](crate::enums::GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH)
    /// are available only if the GL version 3.1 or greater.
    ///
    /// [`GL_GEOMETRY_VERTICES_OUT`](crate::enums::GL_GEOMETRY_VERTICES_OUT), [`GL_GEOMETRY_INPUT_TYPE`](crate::enums::GL_GEOMETRY_INPUT_TYPE)
    /// and [`GL_GEOMETRY_OUTPUT_TYPE`](crate::enums::GL_GEOMETRY_OUTPUT_TYPE)
    /// are accepted only if the GL version is 3.2 or greater.
    ///
    /// [`GL_COMPUTE_WORK_GROUP_SIZE`](crate::enums::GL_COMPUTE_WORK_GROUP_SIZE)
    /// is accepted only if the GL version is 4.3 or greater.
    ///
    /// If an error is generated, no change is made to the contents of `params`.
    ///
    /// ### Associated Gets
    /// [**glGetActiveAttrib**](crate::context::Context::oxidegl_get_active_attrib)
    /// with argument `program`
    ///
    /// [**glGetActiveUniform**](crate::context::Context::oxidegl_get_active_uniform)
    /// with argument `program`
    ///
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with argument `program`
    ///
    /// [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log)
    /// with argument `program`
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    pub unsafe fn oxidegl_get_programiv(
        &mut self,
        program: GLuint,
        pname: ProgramProperty,
        params: *mut GLint,
    ) {
        let program = self.gl_state.program_list.get_raw(program);
        gl_debug!("getting program property {pname:?} of {:?}", program.name);
        //2gb shader is not real :3
        #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let ret = match pname {
            ProgramProperty::DeleteStatus => {
                i32::from(self.gl_state.program_deletion_queue.contains(&program.name))
            }
            ProgramProperty::LinkStatus | ProgramProperty::ValidateStatus => {
                i32::from(program.latest_linkage.is_some())
            }

            ProgramProperty::InfoLogLength => program.info_log.len() as i32,
            ProgramProperty::AttachedShaders => program.attached_shader_count() as i32,
            ProgramProperty::ActiveUniforms => todo!(),
            ProgramProperty::ActiveUniformMaxLength => todo!(),
            ProgramProperty::ActiveAttributes => todo!(),
            ProgramProperty::ActiveAttributeMaxLength => todo!(),
            ProgramProperty::TransformFeedbackVaryingMaxLength => todo!(),
            ProgramProperty::TransformFeedbackBufferMode => todo!(),
            ProgramProperty::TransformFeedbackVaryings => todo!(),
            ProgramProperty::ActiveUniformBlockMaxNameLength => todo!(),
            ProgramProperty::ActiveUniformBlocks => todo!(),
            ProgramProperty::GeometryVerticesOut => todo!(),
            ProgramProperty::GeometryInputType => todo!(),
            ProgramProperty::GeometryOutputType => todo!(),
            ProgramProperty::ProgramBinaryLength => todo!(),
            ProgramProperty::ActiveAtomicCounterBuffers => todo!(),
            ProgramProperty::ComputeWorkGroupSize => todo!(),
        };
        // Safety: caller ensures params points to a valid storage and is aligned correctly for `i32`
        unsafe { core::ptr::write(params, ret) };
    }

    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the handle of the program object to be validated.
    ///
    /// ### Description
    /// [**glValidateProgram**](crate::context::Context::oxidegl_validate_program)
    /// checks to see whether the executables contained in `program` can execute
    /// given the current OpenGL state. The information generated by the validation
    /// process will be stored in `program` 's information log. The validation
    /// information may consist of an empty string, or it may be a string containing
    /// information about how the current program object interacts with the rest
    /// of current OpenGL state. This provides a way for OpenGL implementers to
    /// convey more information about why the current program is inefficient, suboptimal,
    /// failing to execute, and so on.
    ///
    /// The status of the validation operation will be stored as part of the program
    /// object's state. This value will be set to [`GL_TRUE`](crate::enums::GL_TRUE)
    /// if the validation succeeded, and [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
    /// It can be queried by calling [**glGetProgram**](crate::context::Context::oxidegl_get_program)
    /// with arguments `program` and [`GL_VALIDATE_STATUS`](crate::enums::GL_VALIDATE_STATUS).
    /// If validation is successful, `program` is guaranteed to execute given
    /// the current state. Otherwise, `program` is guaranteed to not execute.
    ///
    /// This function is typically useful only during application development.
    /// The informational string stored in the information log is completely implementation
    /// dependent; therefore, an application should not expect different OpenGL
    /// implementations to produce identical information strings.
    ///
    /// ### Notes
    /// This function mimics the validation operation that OpenGL implementations
    /// must perform when rendering commands are issued while programmable shaders
    /// are part of current state. The error [`GL_INVALID_OPERATION`](crate::enums::GL_INVALID_OPERATION)
    /// will be generated by any command that triggers the rendering of geometry
    /// if:
    ///
    /// > any two active samplers in the current program object are of different
    /// > types, but refer to the same texture image unit,
    ///
    /// > the number of active samplers in the program exceeds the maximum number
    /// > of texture image units allowed.
    ///
    /// It may be difficult or cause a performance degradation for applications
    /// to catch these errors when rendering commands are issued. Therefore, applications
    /// are advised to make calls to [**glValidateProgram**](crate::context::Context::oxidegl_validate_program)
    /// to detect these issues during application development.
    ///
    /// ### Associated Gets
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) with arguments
    /// `program` and [`GL_VALIDATE_STATUS`](crate::enums::GL_VALIDATE_STATUS)
    ///
    /// [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log)
    /// with argument `program`
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    //HACK: validation is a no-op because it's Complicated™
    pub fn oxidegl_validate_program(&mut self, program: GLuint) {}

    /// ### Parameters
    /// `program`
    ///
    /// > Specifies the handle of the program object whose executables are to be
    /// > used as part of current rendering state.
    ///
    /// ### Description
    /// [**glUseProgram**](crate::context::Context::oxidegl_use_program) installs
    /// the program object specified by `program` as part of current rendering
    /// state. One or more executables are created in a program object by successfully
    /// attaching shader objects to it with [**glAttachShader**](crate::context::Context::oxidegl_attach_shader),
    /// successfully compiling the shader objects with [**glCompileShader**](crate::context::Context::oxidegl_compile_shader),
    /// and successfully linking the program object with [**glLinkProgram**](crate::context::Context::oxidegl_link_program).
    ///
    /// A program object will contain an executable that will run on the vertex
    /// processor if it contains one or more shader objects of type [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER)
    /// that have been successfully compiled and linked. A program object will
    /// contain an executable that will run on the geometry processor if it contains
    /// one or more shader objects of type [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER)
    /// that have been successfully compiled and linked. Similarly, a program object
    /// will contain an executable that will run on the fragment processor if it
    /// contains one or more shader objects of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER)
    /// that have been successfully compiled and linked.
    ///
    /// While a program object is in use, applications are free to modify attached
    /// shader objects, compile attached shader objects, attach additional shader
    /// objects, and detach or delete shader objects. None of these operations
    /// will affect the executables that are part of the current state. However,
    /// relinking the program object that is currently in use will install the
    /// program object as part of the current rendering state if the link operation
    /// was successful (see [**glLinkProgram**](crate::context::Context::oxidegl_link_program)
    /// ). If the program object currently in use is relinked unsuccessfully, its
    /// link status will be set to [`GL_FALSE`](crate::enums::GL_FALSE), but the
    /// executables and associated state will remain part of the current state
    /// until a subsequent call to [**glUseProgram**](crate::context::Context::oxidegl_use_program)
    /// removes it from use. After it is removed from use, it cannot be made part
    /// of current state until it has been successfully relinked.
    ///
    /// If `program` is zero, then the current rendering state refers to an *invalid*
    /// program object and the results of shader execution are undefined. However,
    /// this is not an error.
    ///
    /// If `program` does not contain shader objects of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER),
    /// an executable will be installed on the vertex, and possibly geometry processors,
    /// but the results of fragment shader execution will be undefined.
    ///
    /// ### Notes
    /// Like buffer and texture objects, the name space for program objects may
    /// be shared across a set of contexts, as long as the server sides of the
    /// contexts share the same address space. If the name space is shared across
    /// contexts, any attached objects and the data associated with those attached
    /// objects are shared as well.
    ///
    /// Applications are responsible for providing the synchronization across API
    /// calls when objects are accessed from different execution threads.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with the argument [`GL_CURRENT_PROGRAM`](crate::enums::GL_CURRENT_PROGRAM)
    ///
    /// [**glGetActiveAttrib**](crate::context::Context::oxidegl_get_active_attrib)
    /// with a valid program object and the index of an active attribute variable
    ///
    /// [**glGetActiveUniform**](crate::context::Context::oxidegl_get_active_uniform)
    /// with a valid program object and the index of an active uniform variable
    ///
    /// [**glGetAttachedShaders**](crate::context::Context::oxidegl_get_attached_shaders)
    /// with a valid program object
    ///
    /// [**glGetAttribLocation**](crate::context::Context::oxidegl_get_attrib_location)
    /// with a valid program object and the name of an attribute variable
    ///
    /// [**glGetProgram**](crate::context::Context::oxidegl_get_program) with a
    /// valid program object and the parameter to be queried
    ///
    /// [**glGetProgramInfoLog**](crate::context::Context::oxidegl_get_program_info_log)
    /// with a valid program object
    ///
    /// [**glGetUniform**](crate::context::Context::oxidegl_get_uniform) with a
    /// valid program object and the location of a uniform variable
    ///
    /// [**glGetUniformLocation**](crate::context::Context::oxidegl_get_uniform_location)
    /// with a valid program object and the name of a uniform variable
    ///
    /// [**glIsProgram**](crate::context::Context::oxidegl_is_program)
    pub fn oxidegl_use_program(&mut self, program: GLuint) {
        let name = ObjectName::try_from_raw(program);
        debug_assert!(
            name.map(|name| self.gl_state.program_list.is(name))
                .is_some_and(|b| b),
            "UB: tried to bind an invalid shader program!"
        );
        run_if_changed!(self.gl_state.program_binding;= name => {
                self.new_pipeline();
                self.new_encoder();
                self.remap_buffers();
            }
        );
        gl_debug!("bound {name:?} as current shader program");
    }
}
