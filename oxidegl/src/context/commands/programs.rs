use crate::{
    context::{program::Program, Context},
    dispatch::gl_types::GLuint,
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
            .shader_program_list
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
        let program = self.gl_state.shader_program_list.get_raw_mut(program);
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
        let program = self.gl_state.shader_program_list.get_raw_mut(program);
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
        let program = self.gl_state.shader_program_list.get_raw_mut(program);
        let shader = self.gl_state.shader_list.get_raw_mut(shader);
        let name = shader.name;
        if program.detach_shader(shader) && self.gl_state.shaders_to_delete.contains(&name) {
            self.gl_state.shader_list.delete(name);
            self.gl_state.shaders_to_delete.remove(&name);
        }
    }
}
