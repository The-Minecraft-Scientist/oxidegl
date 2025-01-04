use crate::{
    context::{
        commands::{buffer::Buffer, vao::Vao},
        debug::{gl_warn, with_debug_state, with_debug_state_mut},
        framebuffer::Framebuffer,
        program::Program,
        shader::Shader,
        state::ObjectName,
        Context,
    },
    dispatch::gl_types::{GLboolean, GLchar, GLsizei, GLuint, GLvoid, GLDEBUGPROC},
    enums::{DebugSeverity, DebugSource, DebugType, GetPointervPName, ObjectIdentifier},
};
use core::slice;
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    ptr,
};

impl Context {
    /// ### Parameters
    /// `identifier`
    ///
    /// > The namespace from which the name of the object is allocated.
    ///
    /// `name`
    ///
    /// > The name of the object to label.
    ///
    /// `length`
    ///
    /// > The length of the label to be used for the object.
    ///
    /// `label`
    ///
    /// > The address of a string containing the label to assign to the object.
    ///
    /// ### Description
    /// [**glObjectLabel**](crate::context::Context::oxidegl_object_label) labels
    /// the object identified by `name` within the namespace given by `identifier`.
    /// `identifier` must be one of [`GL_BUFFER`](crate::enums::GL_BUFFER), [`GL_SHADER`](crate::enums::GL_SHADER),
    /// [`GL_PROGRAM`](crate::enums::GL_PROGRAM), [`GL_VERTEX_ARRAY`](crate::enums::GL_VERTEX_ARRAY),
    /// [`GL_QUERY`](crate::enums::GL_QUERY), [`GL_PROGRAM_PIPELINE`](crate::enums::GL_PROGRAM_PIPELINE),
    /// [`GL_TRANSFORM_FEEDBACK`](crate::enums::GL_TRANSFORM_FEEDBACK), [`GL_SAMPLER`](crate::enums::GL_SAMPLER),
    /// [`GL_TEXTURE`](crate::enums::GL_TEXTURE), [`GL_RENDERBUFFER`](crate::enums::GL_RENDERBUFFER),
    /// [`GL_FRAMEBUFFER`](crate::enums::GL_FRAMEBUFFER), to indicate the namespace
    /// containing the names of buffers, shaders, programs, vertex array objects,
    /// query objects, program pipelines, transform feedback objects, samplers,
    /// textures, renderbuffers and frame buffers, respectively.
    ///
    /// `label` is the address of a string that will be used to label an object.
    /// `length` contains the number of characters in `label`. If `length` is negative,
    /// it is implied that `label` contains a null-terminated string. If `label`
    /// is NULL, any debug label is effectively removed from the object.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH).
    pub unsafe fn oxidegl_object_label(
        &mut self,
        identifier: ObjectIdentifier,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) {
        #[expect(
            clippy::cast_possible_wrap,
            reason = "bitcast is the desired behavior here"
        )]
        let lengthi = length as i32;
        let label = match (label.is_null(), lengthi) {
            (false, 0..) => {
                // safety: caller ensures that if length is positive, label points to a byte array of length
                let mut buf =
                    unsafe { slice::from_raw_parts(label.cast::<u8>(), length as usize).to_vec() };
                if buf.last().is_some_and(|v| *v != 0) || buf.last().is_none() {
                    // may not be null terminated
                    buf.push(0);
                }
                Some(
                    CString::from_vec_with_nul(buf)
                        .expect("invalid c string!")
                        .into_boxed_c_str(),
                )
            }
            // Safety: caller ensures that if length is negative label points to a nul-terminated c string
            (false, ..0) => Some(unsafe { CStr::from_ptr(label) }.into()),
            (true, _) => None,
        };
        let name = ObjectName::<()>::from_raw(name);
        with_debug_state_mut(|mut state| match identifier {
            ObjectIdentifier::Framebuffer => state.set_label(name.cast::<Framebuffer>(), label),
            ObjectIdentifier::Buffer => state.set_label(name.cast::<Buffer>(), label),
            ObjectIdentifier::Shader => state.set_label(name.cast::<Shader>(), label),
            ObjectIdentifier::Program => state.set_label(name.cast::<Program>(), label),
            ObjectIdentifier::VertexArray => state.set_label(name.cast::<Vao>(), label),
            ObjectIdentifier::ProgramPipeline => todo!(),
            ObjectIdentifier::Texture => todo!(),
            ObjectIdentifier::Renderbuffer => todo!(),
            ObjectIdentifier::TransformFeedback => todo!(),
            ObjectIdentifier::Query => todo!(),
            ObjectIdentifier::Sampler => todo!(),
        });
    }
    /// ### Parameters
    /// `ptr`
    ///
    /// > A pointer identifying a sync object.
    ///
    /// `length`
    ///
    /// > The length of the label to be used for the object.
    ///
    /// `label`
    ///
    /// > The address of a string containing the label to assign to the object.
    ///
    /// ### Description
    /// [**glObjectPtrLabel**](crate::context::Context::oxidegl_object_ptr_label)
    /// labels the sync object identified by `ptr`.
    ///
    /// `label` is the address of a string that will be used to label the object.
    /// `length` contains the number of characters in `label`. If `length` is negative,
    /// it is implied that `label` contains a null-terminated string. If `label`
    /// is NULL, any debug label is effectively removed from the object.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH).
    pub unsafe fn oxidegl_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        length: GLsizei,
        label: *const GLchar,
    ) {
        panic!("command oxidegl_object_ptr_label not yet implemented");
    }

    /// ### Parameters
    /// `identifier`
    ///
    /// > The namespace from which the name of the object is allocated.
    ///
    /// `name`
    ///
    /// > The name of the object whose label to retrieve.
    ///
    /// `bufSize`
    ///
    /// > The length of the buffer whose address is in `label`.
    ///
    /// `length`
    ///
    /// > The address of a variable to receive the length of the object label.
    ///
    /// `label`
    ///
    /// > The address of a string that will receive the object label.
    ///
    /// ### Description
    /// [**glGetObjectLabel**](crate::context::Context::oxidegl_get_object_label)
    /// retrieves the label of the object identified by `name` within the namespace
    /// given by `identifier`. `identifier` must be one of [`GL_BUFFER`](crate::enums::GL_BUFFER),
    /// [`GL_SHADER`](crate::enums::GL_SHADER), [`GL_PROGRAM`](crate::enums::GL_PROGRAM),
    /// [`GL_VERTEX_ARRAY`](crate::enums::GL_VERTEX_ARRAY), [`GL_QUERY`](crate::enums::GL_QUERY),
    /// [`GL_PROGRAM_PIPELINE`](crate::enums::GL_PROGRAM_PIPELINE), [`GL_TRANSFORM_FEEDBACK`](crate::enums::GL_TRANSFORM_FEEDBACK),
    /// [`GL_SAMPLER`](crate::enums::GL_SAMPLER), [`GL_TEXTURE`](crate::enums::GL_TEXTURE),
    /// [`GL_RENDERBUFFER`](crate::enums::GL_RENDERBUFFER), [`GL_FRAMEBUFFER`](crate::enums::GL_FRAMEBUFFER),
    /// to indicate the namespace containing the names of buffers, shaders, programs,
    /// vertex array objects, query objects, program pipelines, transform feedback
    /// objects, samplers, textures, renderbuffers and frame buffers, respectively.
    ///
    /// `label` is the address of a string that will be used to store the object
    /// label. `bufSize` specifies the number of characters in the array identified
    /// by `label`. `length` contains the address of a variable which will receive
    /// the number of characters in the object label. If `length` is NULL, then
    /// it is ignored and no data is written. Likewise, if `label` is NULL, or
    /// if `bufSize` is zero then no data is written to `label`.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH).
    pub unsafe fn oxidegl_get_object_label(
        &mut self,
        identifier: ObjectIdentifier,
        name: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        const EMPTY: &CStr = c"";
        let name = ObjectName::<()>::from_raw(name);
        let to_write = with_debug_state(|state| match identifier {
            ObjectIdentifier::Framebuffer => state.get_label(name.cast::<Framebuffer>()),
            ObjectIdentifier::Buffer => state.get_label(name.cast::<Buffer>()),
            ObjectIdentifier::Shader => state.get_label(name.cast::<Shader>()),
            ObjectIdentifier::Program => state.get_label(name.cast::<Program>()),
            ObjectIdentifier::VertexArray => state.get_label(name.cast::<Vao>()),
            ObjectIdentifier::ProgramPipeline => todo!(),
            ObjectIdentifier::Texture => todo!(),
            ObjectIdentifier::Renderbuffer => todo!(),
            ObjectIdentifier::TransformFeedback => todo!(),
            ObjectIdentifier::Query => todo!(),
            ObjectIdentifier::Sampler => todo!(),
        })
        .flatten();

        let to_write_ref = to_write.as_deref().unwrap_or(EMPTY);

        #[expect(clippy::cast_possible_truncation, reason = "blame the spec bestie")]
        let label_len_with_nul = to_write_ref.count_bytes() as u32 + 1; // + 1 for nul byte

        // Figure out which one of Khronos' glorious function overloads we're in
        match (label.is_null(), length.is_null(), buf_size) {
            // valid buffer for writing, continue;
            (false, _, 1..) => {}

            // invalid buffer for writing
            (false, t, 0) => {
                gl_warn!("Tried to read an object label into a zero-length buffer");
                if !t {
                    // Safety: checked for null, otherwise caller ensures that length is a valid pointer to a 32 bit integer
                    unsafe {
                        *length = 0;
                    }
                }
                return;
            }
            // overload to write out the total string length, not the number of bytes written
            (true, false, _) => {
                // Safety: checked for null, otherwise caller ensures that length is valid for writes
                unsafe {
                    *length = label_len_with_nul - 1;
                }
                return;
            }
            (true, true, _) => {
                gl_warn!(src: Api, ty: Portability, "Unspecified behavior: both <length> and <label> for glGetObjectLabel are null");
                return;
            }
        }

        if label_len_with_nul > buf_size {
            gl_warn!(src: Api, ty: Portability, "Unspecified behavior: buf size for glGetObjectLabel is 
                smaller than the length of the label. This implementation truncates the label to fit, but 
                this is not guaranteed by the spec");
        }
        // see assert above, len and buf_size are both guaranteed to be >= 1 so this cannot underflow
        // subtract 1 since length needs to be the number of character bytes written, not the total number of bytes written
        let bytes_to_copy = buf_size.min(label_len_with_nul) - 1;

        if !length.is_null() {
            // Safety: if length is non-null, caller ensures that it is valid for stores of u32
            unsafe {
                *length = bytes_to_copy;
            };
        }
        let bytes = to_write_ref.to_bytes();
        // make array of uninit for slice copy
        let label_bytes =
            // Safety: valid to create an &[MaybeUninit<u8>] from &[u8]
            unsafe { slice::from_raw_parts(bytes.as_ptr().cast::<MaybeUninit<u8>>(), bytes.len()) };
        // Safety: if label is non-null caller ensures it points to a (possibly uninitialized) byte buffer of size buf_size and is valid for writes
        let buf = unsafe {
            slice::from_raw_parts_mut(label.cast::<MaybeUninit<u8>>(), buf_size as usize)
        };
        buf[0..(bytes_to_copy as usize)].copy_from_slice(&label_bytes[0..(bytes_to_copy as usize)]);
        // write nul terminator
        buf[bytes_to_copy as usize + 1] = MaybeUninit::new(0);
    }

    /// ### Parameters
    /// `ptr`
    ///
    /// > The name of the sync object whose label to retrieve.
    ///
    /// `bufSize`
    ///
    /// > The length of the buffer whose address is in `label`.
    ///
    /// `length`
    ///
    /// > The address of a variable to receive the length of the object label.
    ///
    /// `label`
    ///
    /// > The address of a string that will receive the object label.
    ///
    /// ### Description
    /// [**glGetObjectPtrLabel**](crate::context::Context::oxidegl_get_object_ptr_label)
    /// retrieves the label of the sync object identified by `ptr`.
    ///
    /// `label` is the address of a string that will be used to store the object
    /// label. `bufSize` specifies the number of characters in the array identified
    /// by `label`. `length` contains the address of a variable which will receive
    /// the number of characters in the object label. If `length` is NULL, then
    /// it is ignored and no data is written. Likewise, if `label` is NULL, or
    /// if `bufSize` is zero then no data is written to `label`.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_LABEL_LENGTH`](crate::enums::GL_MAX_LABEL_LENGTH).
    pub unsafe fn oxidegl_get_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        buf_size: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        panic!("command oxidegl_get_object_ptr_label not yet implemented");
    }
    /// ### Parameters
    /// `pname`
    ///
    /// > Specifies the pointer to be returned. Must be one of [`GL_DEBUG_CALLBACK_FUNCTION`](crate::enums::GL_DEBUG_CALLBACK_FUNCTION)
    /// > or [`GL_DEBUG_CALLBACK_USER_PARAM`](crate::enums::GL_DEBUG_CALLBACK_USER_PARAM).
    ///
    /// `params`
    ///
    /// > Returns the pointer value specified by `pname`.
    ///
    /// ### Description
    /// [**glGetPointerv**](crate::context::Context::oxidegl_get_pointerv) returns
    /// pointer information. `pname` indicates the pointer to be returned, and
    /// `params` is a pointer to a location in which to place the returned data.
    /// The parameters that may be queried include:
    ///
    /// [`GL_DEBUG_CALLBACK_FUNCTION`](crate::enums::GL_DEBUG_CALLBACK_FUNCTION)
    ///
    /// > Returns the current callback function set with the `callback` argument
    /// > of [**glDebugMessageCallback**](crate::context::Context::oxidegl_debug_message_callback).
    ///
    /// [`GL_DEBUG_CALLBACK_USER_PARAM`](crate::enums::GL_DEBUG_CALLBACK_USER_PARAM)
    ///
    /// > Returns the user parameter to the current callback function set with the
    /// > `userParam` argument of [**glDebugMessageCallback**](crate::context::Context::oxidegl_debug_message_callback).
    ///
    /// ### Notes
    /// [**glGetPointerv**](crate::context::Context::oxidegl_get_pointerv) is available
    /// in the OpenGL core profile only if the GL version is 4.3 or later. It is
    /// available in the compatibility profile for all GL versions, and accepts
    /// additional queries. However, these reference pages document only the core
    /// profile.
    pub unsafe fn oxidegl_get_pointerv(
        &mut self,
        pname: GetPointervPName,
        params: *mut *mut GLvoid,
    ) {
        let ptr = with_debug_state(|state| match pname {
            GetPointervPName::DebugCallbackFunction => state.callback.map_or(ptr::null(), |v| {
                (v as *const unsafe fn(u32, u32, u32, u32, u32, *const i8, *const GLvoid))
                    .cast::<GLvoid>()
            }),
            GetPointervPName::DebugCallbackUserParam => state.user_param_ptr,
        })
        .expect("no debug context!")
        .cast_mut();
        // Safety: caller ensures params is valid for writes
        unsafe {
            *params = ptr;
        }
    }
    /// ### Parameters
    /// `callback`
    ///
    /// > The address of a callback function that will be called when a debug message
    /// > is generated.
    ///
    /// `userParam`
    ///
    /// > A user supplied pointer that will be passed on each invocation of `callback`.
    ///
    /// ### Description
    /// [**glDebugMessageCallback**](crate::context::Context::oxidegl_debug_message_callback)
    /// sets the current debug output callback function to the function whose address
    /// is given in `callback`. The callback function should have the following
    /// prototype (in C), or be otherwise compatible with such a prototype:
    ///
    /// This function is defined to have the same calling convention as the GL
    /// API functions. In most cases this is defined as
    ///
    /// Each time a debug message is generated the debug callback function will
    /// be invoked with `source`, `type`, `id`, and `severity` associated with
    /// the message, and `length` set to the length of debug message whose character
    /// string is in the array pointed to by `message`. `userParam` will be set
    /// to the value passed in the `userParam` parameter to the most recent call
    /// to [**glDebugMessageCallback**](crate::context::Context::oxidegl_debug_message_callback).
    ///
    /// ### Notes
    /// When the GL is in use remotely, the server may not be able to call functions
    /// in the client's address space. In such cases, the callback function may
    /// not be invoked and the user should retrieve debug messages from the context's
    /// debug message log by calling [**glGetDebugMessageLog**](crate::context::Context::oxidegl_get_debug_message_log).
    pub unsafe fn oxidegl_debug_message_callback(
        &mut self,
        callback: GLDEBUGPROC,
        user_param: *const GLvoid,
    ) {
        with_debug_state_mut(|mut state| {
            state.callback = callback;
            state.user_param_ptr = user_param.cast_mut();
        });
    }
    /// ### Parameters
    /// `source`
    ///
    /// > The source of debug messages to enable or disable.
    ///
    /// `type`
    ///
    /// > The type of debug messages to enable or disable.
    ///
    /// `severity`
    ///
    /// > The severity of debug messages to enable or disable.
    ///
    /// `count`
    ///
    /// > The length of the array `ids`.
    ///
    /// `ids`
    ///
    /// > The address of an array of unsigned integers contianing the ids of the
    /// > messages to enable or disable.
    ///
    /// `enabled`
    ///
    /// > A Boolean flag determining whether the selected messages should be enabled
    /// > or disabled.
    ///
    /// ### Description
    /// [**glDebugMessageControl**](crate::context::Context::oxidegl_debug_message_control)
    /// controls the reporting of debug messages generated by a debug context.
    /// The parameters `source`, `type` and `severity` form a filter to select
    /// messages from the pool of potential messages generated by the GL.
    ///
    /// `source` may be [`GL_DEBUG_SOURCE_API`](crate::enums::GL_DEBUG_SOURCE_API),
    /// [`GL_DEBUG_SOURCE_WINDOW_SYSTEM_`](crate::enums::GL_DEBUG_SOURCE_WINDOW_SYSTEM_),
    /// [`GL_DEBUG_SOURCE_SHADER_COMPILER`](crate::enums::GL_DEBUG_SOURCE_SHADER_COMPILER),
    /// [`GL_DEBUG_SOURCE_THIRD_PARTY`](crate::enums::GL_DEBUG_SOURCE_THIRD_PARTY),
    /// [`GL_DEBUG_SOURCE_APPLICATION`](crate::enums::GL_DEBUG_SOURCE_APPLICATION),
    /// [`GL_DEBUG_SOURCE_OTHER`](crate::enums::GL_DEBUG_SOURCE_OTHER) to select
    /// messages generated by usage of the GL API, the window system, the shader
    /// compiler, third party tools or libraries, explicitly by the application
    /// or by some other source, respectively. It may also take the value [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
    /// If `source` is not [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE) then only
    /// messages whose source matches `source` will be referenced.
    ///
    /// `type` may be one of [`GL_DEBUG_TYPE_ERROR`](crate::enums::GL_DEBUG_TYPE_ERROR),
    /// [`GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR`](crate::enums::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR),
    /// [`GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR`](crate::enums::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR),
    /// [`GL_DEBUG_TYPE_PORTABILITY`](crate::enums::GL_DEBUG_TYPE_PORTABILITY),
    /// [`GL_DEBUG_TYPE_PERFORMANCE`](crate::enums::GL_DEBUG_TYPE_PERFORMANCE),
    /// [`GL_DEBUG_TYPE_MARKER`](crate::enums::GL_DEBUG_TYPE_MARKER), [`GL_DEBUG_TYPE_PUSH_GROUP`](crate::enums::GL_DEBUG_TYPE_PUSH_GROUP),
    /// [`GL_DEBUG_TYPE_POP_GROUP`](crate::enums::GL_DEBUG_TYPE_POP_GROUP), or
    /// [`GL_DEBUG_TYPE_OTHER`](crate::enums::GL_DEBUG_TYPE_OTHER) to indicate
    /// the type of messages describing GL errors, attempted use of deprecated
    /// features, triggering of undefined behavior, portability issues, performance
    /// notifications, markers, group push and pop events, and other types of messages,
    /// respectively. It may also take the value [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
    /// If `type` is not [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE) then only
    /// messages whose type matches `type` will be referenced.
    ///
    /// `severity` may be one of [`GL_DEBUG_SEVERITY_LOW`](crate::enums::GL_DEBUG_SEVERITY_LOW),
    /// [`GL_DEBUG_SEVERITY_MEDIUM`](crate::enums::GL_DEBUG_SEVERITY_MEDIUM), or
    /// [`GL_DEBUG_SEVERITY_HIGH`](crate::enums::GL_DEBUG_SEVERITY_HIGH) to select
    /// messages of low, medium or high severity messages or to [`GL_DEBUG_SEVERITY_NOTIFICATION`](crate::enums::GL_DEBUG_SEVERITY_NOTIFICATION)
    /// for notifications. It may also take the value [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
    /// If `severity` is not [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE) then
    /// only messages whose severity matches `severity` will be referenced.
    ///
    /// `ids` contains a list of `count` message identifiers to select specific
    /// messages from the pool of available messages. If `count` is zero then the
    /// value of `ids` is ignored. Otherwise, only messages appearing in this list
    /// are selected. In this case, `source` and `type` may not be [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE)
    /// and `severity` must be [`GL_DONT_CARE`](crate::enums::GL_DONT_CARE).
    ///
    /// If `enabled` is [`GL_TRUE`](crate::enums::GL_TRUE) then messages that match
    /// the filter formed by `source`, `type`, `severity` and `ids` are enabled.
    /// Otherwise, those messages are disabled.
    ///
    /// ### Notes
    /// Although debug messages may be enabled in a non-debug context, the quantity
    /// and detail of such messages may be substantially inferior to those in a
    /// debug context. In particular, a valid implementation of the debug message
    /// queue in a non-debug context may produce no messages at all.
    ///
    /// [`GL_DEBUG_TYPE_MARKER`](crate::enums::GL_DEBUG_TYPE_MARKER), [`GL_DEBUG_TYPE_PUSH_GROUP`](crate::enums::GL_DEBUG_TYPE_PUSH_GROUP),
    /// [`GL_DEBUG_TYPE_POP_GROUP`](crate::enums::GL_DEBUG_TYPE_POP_GROUP), and
    /// [`GL_DEBUG_SEVERITY_NOTIFICATION`](crate::enums::GL_DEBUG_SEVERITY_NOTIFICATION)
    /// are available only if the GL version is 4.3 or higher.
    pub unsafe fn oxidegl_debug_message_control(
        &mut self,
        source: DebugSource,
        r#type: DebugType,
        severity: DebugSeverity,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) {
        // Safety: caller ensures invariants discussed in message_control are upheld
        with_debug_state_mut(|mut state| unsafe {
            state.message_control(source, r#type, severity, count, ids, enabled);
        });
    }
    /// ### Parameters
    /// `source`
    ///
    /// > The source of the debug message to insert.
    ///
    /// `type`
    ///
    /// > The type of the debug message insert.
    ///
    /// `id`
    ///
    /// > The user-supplied identifier of the message to insert.
    ///
    /// `severity`
    ///
    /// > The severity of the debug messages to insert.
    ///
    /// `length`
    ///
    /// > The length string contained in the character array whose address is given
    /// > by `message`.
    ///
    /// `message`
    ///
    /// > The address of a character array containing the message to insert.
    ///
    /// ### Description
    /// [**glDebugMessageInsert**](crate::context::Context::oxidegl_debug_message_insert)
    /// inserts a user-supplied message into the debug output queue. `source` specifies
    /// the source that will be used to classify the message and must be [`GL_DEBUG_SOURCE_APPLICATION`](crate::enums::GL_DEBUG_SOURCE_APPLICATION)
    /// or [`GL_DEBUG_SOURCE_THIRD_PARTY`](crate::enums::GL_DEBUG_SOURCE_THIRD_PARTY).
    /// All other sources are reserved for use by the GL implementation. `type`
    /// indicates the type of the message to be inserted and may be one of [`GL_DEBUG_TYPE_ERROR`](crate::enums::GL_DEBUG_TYPE_ERROR),
    /// [`GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR`](crate::enums::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR),
    /// [`GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR`](crate::enums::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR),
    /// [`GL_DEBUG_TYPE_PORTABILITY`](crate::enums::GL_DEBUG_TYPE_PORTABILITY),
    /// [`GL_DEBUG_TYPE_PERFORMANCE`](crate::enums::GL_DEBUG_TYPE_PERFORMANCE),
    /// [`GL_DEBUG_TYPE_MARKER`](crate::enums::GL_DEBUG_TYPE_MARKER), [`GL_DEBUG_TYPE_PUSH_GROUP`](crate::enums::GL_DEBUG_TYPE_PUSH_GROUP),
    /// [`GL_DEBUG_TYPE_POP_GROUP`](crate::enums::GL_DEBUG_TYPE_POP_GROUP), or
    /// [`GL_DEBUG_TYPE_OTHER`](crate::enums::GL_DEBUG_TYPE_OTHER). `severity`
    /// indicates the severity of the message and may be [`GL_DEBUG_SEVERITY_LOW`](crate::enums::GL_DEBUG_SEVERITY_LOW),
    /// [`GL_DEBUG_SEVERITY_MEDIUM`](crate::enums::GL_DEBUG_SEVERITY_MEDIUM), [`GL_DEBUG_SEVERITY_HIGH`](crate::enums::GL_DEBUG_SEVERITY_HIGH)
    /// or [`GL_DEBUG_SEVERITY_NOTIFICATION`](crate::enums::GL_DEBUG_SEVERITY_NOTIFICATION).
    /// `id` is available for application defined use and may be any value. This
    /// value will be recorded and used to identify the message.
    ///
    /// `length` contains a count of the characters in the character array whose
    /// address is given in `message`. If `length` is negative then `message` is
    /// treated as a null-terminated string. The length of the message, whether
    /// specified explicitly or implicitly, must be less than or equal to the implementation
    /// defined constant [`GL_MAX_DEBUG_MESSAGE_LENGTH`](crate::enums::GL_MAX_DEBUG_MESSAGE_LENGTH).
    ///
    /// ### Notes
    /// [`GL_DEBUG_TYPE_MARKER`](crate::enums::GL_DEBUG_TYPE_MARKER), [`GL_DEBUG_TYPE_PUSH_GROUP`](crate::enums::GL_DEBUG_TYPE_PUSH_GROUP),
    /// [`GL_DEBUG_TYPE_POP_GROUP`](crate::enums::GL_DEBUG_TYPE_POP_GROUP), and
    /// [`GL_DEBUG_SEVERITY_NOTIFICATION`](crate::enums::GL_DEBUG_SEVERITY_NOTIFICATION)
    /// are available only if the GL version is 4.3 or higher.
    pub unsafe fn oxidegl_debug_message_insert(
        &mut self,
        source: DebugSource,
        r#type: DebugType,
        id: GLuint,
        severity: DebugSeverity,
        length: GLsizei,
        buf: *const GLchar,
    ) {
        panic!("command oxidegl_debug_message_insert not yet implemented");
    }
}
