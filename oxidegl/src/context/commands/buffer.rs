use core::{ffi::c_void, fmt::Debug, ptr::NonNull};

use log::debug;
use objc2_metal::{MTLBuffer, MTLDevice, MTLResourceOptions};

use crate::{
    context::{
        state::{NamedObject, ObjectName},
        Context,
    },
    debug_unreachable,
    dispatch::{
        conversions::{IndexType, NoIndex},
        gl_types::{GLboolean, GLsizei, GLsizeiptr, GLuint, GLvoid},
    },
    enums::{
        BufferAccess, BufferStorageMask, BufferStorageTarget, BufferTarget, BufferUsage,
        MapBufferAccessMask,
    },
    ProtoObjRef,
};

impl Context {
    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of buffer object names to be generated.
    ///
    /// `buffers`
    ///
    /// > Specifies an array in which the generated buffer object names are stored.
    ///
    /// ### Description
    /// [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers) returns
    /// `n` buffer object names in `buffers`. There is no guarantee that the names
    /// form a contiguous set of integers; however, it is guaranteed that none
    /// of the returned names was in use immediately before the call to [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers).
    ///
    /// Buffer object names returned by a call to [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers)
    /// are not returned by subsequent calls, unless they are first deleted with
    /// [**glDeleteBuffers**](crate::context::Context::oxidegl_delete_buffers).
    ///
    /// No buffer objects are associated with the returned buffer object names
    /// until they are first bound by calling [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer).
    ///
    /// ### Associated Gets
    /// [**glIsBuffer**](crate::context::Context::oxidegl_is_buffer)

    pub unsafe fn oxidegl_gen_buffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        // Safety: Caller ensures validity
        unsafe { self.gl_state.buffer_list.gen_obj(n, buffers) }
    }

    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of buffer objects to create.
    ///
    /// `buffers`
    ///
    /// > Specifies an array in which names of the new buffer objects are stored.
    ///
    /// ### Description
    /// [**glCreateBuffers**](crate::context::Context::oxidegl_create_buffers)
    /// returns `n` previously unused buffer names in `buffers`, each representing
    /// a new buffer object initialized as if it had been bound to an unspecified
    /// target.

    pub unsafe fn oxidegl_create_buffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        // Safety: Caller ensures validity
        unsafe {
            self.gl_state
                .buffer_list
                .create_obj(Buffer::new_default, n, buffers);
        }
    }

    /// ### Parameters
    /// `target`
    ///
    /// > Specifies the target to which the buffer object is bound, which must be
    /// > one of the buffer binding targets in the following table:
    ///
    /// > | *Buffer Binding Target*                               | *Purpose*      |
    /// > |-------------------------------------------------------|----------------|
    /// > | [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER)    | Vertex attributes |
    /// > | [`GL_ATOMIC_COUNTER_BUFFER`](crate::enums::GL_ATOMIC_COUNTER_BUFFER) | Atomic counter storage |
    /// > | [`GL_COPY_READ_BUFFER`](crate::enums::GL_COPY_READ_BUFFER) | Buffer copy source |
    /// > | [`GL_COPY_WRITE_BUFFER`](crate::enums::GL_COPY_WRITE_BUFFER) | Buffer copy destination |
    /// > | [`GL_DISPATCH_INDIRECT_BUFFER`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER) | Indirect compute dispatch commands |
    /// > | [`GL_DRAW_INDIRECT_BUFFER`](crate::enums::GL_DRAW_INDIRECT_BUFFER) | Indirect command arguments |
    /// > | [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER) | Vertex array indices |
    /// > | [`GL_PIXEL_PACK_BUFFER`](crate::enums::GL_PIXEL_PACK_BUFFER) | Pixel read target |
    /// > | [`GL_PIXEL_UNPACK_BUFFER`](crate::enums::GL_PIXEL_UNPACK_BUFFER) | Texture data source |
    /// > | [`GL_QUERY_BUFFER`](crate::enums::GL_QUERY_BUFFER)    | Query result buffer |
    /// > | [`GL_SHADER_STORAGE_BUFFER`](crate::enums::GL_SHADER_STORAGE_BUFFER) | Read-write storage for shaders |
    /// > | [`GL_TEXTURE_BUFFER`](crate::enums::GL_TEXTURE_BUFFER) | Texture data buffer |
    /// > | [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER) | Transform feedback buffer |
    /// > | [`GL_UNIFORM_BUFFER`](crate::enums::GL_UNIFORM_BUFFER) | Uniform block storage |
    ///
    /// `buffer`
    ///
    /// > Specifies the name of a buffer object.
    ///
    /// ### Description
    /// [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer) binds
    /// a buffer object to the specified buffer binding point. Calling [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer)
    /// with `target` set to one of the accepted symbolic constants and `buffer`
    /// set to the name of a buffer object binds that buffer object name to the
    /// target. If no buffer object with name `buffer` exists, one is created with
    /// that name. When a buffer object is bound to a target, the previous binding
    /// for that target is automatically broken.
    ///
    /// Buffer object names are unsigned integers. The value zero is reserved,
    /// but there is no default buffer object for each buffer object target. Instead,
    /// `buffer` set to zero effectively unbinds any buffer object previously bound,
    /// and restores client memory usage for that buffer object target (if supported
    /// for that target). Buffer object names and the corresponding buffer object
    /// contents are local to the shared object space of the current GL rendering
    /// context; two rendering contexts share buffer object names only if they
    /// explicitly enable sharing between contexts through the appropriate GL windows
    /// interfaces functions.
    ///
    /// [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers) must be
    /// used to generate a set of unused buffer object names.
    ///
    /// The state of a buffer object immediately after it is first bound is an
    /// unmapped zero-sized memory buffer with [`GL_READ_WRITE`](crate::enums::GL_READ_WRITE)
    /// access and [`GL_STATIC_DRAW`](crate::enums::GL_STATIC_DRAW) usage.
    ///
    /// While a non-zero buffer object name is bound, GL operations on the target
    /// to which it is bound affect the bound buffer object, and queries of the
    /// target to which it is bound return state from the bound buffer object.
    /// While buffer object name zero is bound, as in the initial state, attempts
    /// to modify or query state on the target to which it is bound generates an
    /// [`GL_INVALID_OPERATION`](crate::enums::GL_INVALID_OPERATION) error.
    ///
    /// When a non-zero buffer object is bound to the [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER)
    /// target, the vertex array pointer parameter is interpreted as an offset
    /// within the buffer object measured in basic machine units.
    ///
    /// When a non-zero buffer object is bound to the [`GL_DRAW_INDIRECT_BUFFER`](crate::enums::GL_DRAW_INDIRECT_BUFFER)
    /// target, parameters for draws issued through [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect)
    /// and [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// are sourced from the specified offset in that buffer object's data store.
    ///
    /// When a non-zero buffer object is bound to the [`GL_DISPATCH_INDIRECT_BUFFER`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER)
    /// target, the parameters for compute dispatches issued through [**glDispatchComputeIndirect**](crate::context::Context::oxidegl_dispatch_compute_indirect)
    /// are sourced from the specified offset in that buffer object's data store.
    ///
    /// While a non-zero buffer object is bound to the [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER)
    /// target, the indices parameter of [**glDrawElements**](crate::context::Context::oxidegl_draw_elements),
    /// [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced),
    /// [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex),
    /// [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements),
    /// [**glDrawRangeElementsBaseVertex**](crate::context::Context::oxidegl_draw_range_elements_base_vertex),
    /// [**glMultiDrawElements**](crate::context::Context::oxidegl_multi_draw_elements),
    /// or [**glMultiDrawElementsBaseVertex**](crate::context::Context::oxidegl_multi_draw_elements_base_vertex)
    /// is interpreted as an offset within the buffer object measured in basic
    /// machine units.
    ///
    /// While a non-zero buffer object is bound to the [`GL_PIXEL_PACK_BUFFER`](crate::enums::GL_PIXEL_PACK_BUFFER)
    /// target, the following commands are affected: [**glGetCompressedTexImage**](crate::context::Context::oxidegl_get_compressed_tex_image),
    /// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image), and
    /// [**glReadPixels**](crate::context::Context::oxidegl_read_pixels). The pointer
    /// parameter is interpreted as an offset within the buffer object measured
    /// in basic machine units.
    ///
    /// While a non-zero buffer object is bound to the [`GL_PIXEL_UNPACK_BUFFER`](crate::enums::GL_PIXEL_UNPACK_BUFFER)
    /// target, the following commands are affected: [**glCompressedTexImage1D**](crate::context::Context::oxidegl_compressed_tex_image1_d),
    /// [**glCompressedTexImage2D**](crate::context::Context::oxidegl_compressed_tex_image2_d),
    /// [**glCompressedTexImage3D**](crate::context::Context::oxidegl_compressed_tex_image3_d),
    /// [**glCompressedTexSubImage1D**](crate::context::Context::oxidegl_compressed_tex_sub_image1_d),
    /// [**glCompressedTexSubImage2D**](crate::context::Context::oxidegl_compressed_tex_sub_image2_d),
    /// [**glCompressedTexSubImage3D**](crate::context::Context::oxidegl_compressed_tex_sub_image3_d),
    /// [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d), [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d),
    /// [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d), [**glTexSubImage1D**](crate::context::Context::oxidegl_tex_sub_image1_d),
    /// [**glTexSubImage2D**](crate::context::Context::oxidegl_tex_sub_image2_d),
    /// and [**glTexSubImage3D**](crate::context::Context::oxidegl_tex_sub_image3_d).
    /// The pointer parameter is interpreted as an offset within the buffer object
    /// measured in basic machine units.
    ///
    /// The buffer targets [`GL_COPY_READ_BUFFER`](crate::enums::GL_COPY_READ_BUFFER)
    /// and [`GL_COPY_WRITE_BUFFER`](crate::enums::GL_COPY_WRITE_BUFFER) are provided
    /// to allow [**glCopyBufferSubData**](crate::context::Context::oxidegl_copy_buffer_sub_data)
    /// to be used without disturbing the state of other bindings. However, [**glCopyBufferSubData**](crate::context::Context::oxidegl_copy_buffer_sub_data)
    /// may be used with any pair of buffer binding points.
    ///
    /// The [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER)
    /// buffer binding point may be passed to [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer),
    /// but will not directly affect transform feedback state. Instead, the indexed
    /// [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER)
    /// bindings must be used through a call to [**glBindBufferBase**](crate::context::Context::oxidegl_bind_buffer_base)
    /// or [**glBindBufferRange**](crate::context::Context::oxidegl_bind_buffer_range).
    /// This will affect the generic [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER)
    /// binding.
    ///
    /// Likewise, the [`GL_UNIFORM_BUFFER`](crate::enums::GL_UNIFORM_BUFFER), [`GL_ATOMIC_COUNTER_BUFFER`](crate::enums::GL_ATOMIC_COUNTER_BUFFER)
    /// and [`GL_SHADER_STORAGE_BUFFER`](crate::enums::GL_SHADER_STORAGE_BUFFER)
    /// buffer binding points may be used, but do not directly affect uniform buffer,
    /// atomic counter buffer or shader storage buffer state, respectively. [**glBindBufferBase**](crate::context::Context::oxidegl_bind_buffer_base)
    /// or [**glBindBufferRange**](crate::context::Context::oxidegl_bind_buffer_range)
    /// must be used to bind a buffer to an indexed uniform buffer, atomic counter
    /// buffer or shader storage buffer binding point.
    ///
    /// The [`GL_QUERY_BUFFER`](crate::enums::GL_QUERY_BUFFER) binding point is
    /// used to specify a buffer object that is to receive the results of query
    /// objects through calls to the [**glGetQueryObject**](crate::context::Context::oxidegl_get_query_object)
    /// family of commands.
    ///
    /// A buffer object binding created with [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer)
    /// remains active until a different buffer object name is bound to the same
    /// target, or until the bound buffer object is deleted with [**glDeleteBuffers**](crate::context::Context::oxidegl_delete_buffers).
    ///
    /// Once created, a named buffer object may be re-bound to any target as often
    /// as needed. However, the GL implementation may make choices about how to
    /// optimize the storage of a buffer object based on its initial binding target.
    ///
    /// ### Notes
    /// The [`GL_COPY_READ_BUFFER`](crate::enums::GL_COPY_READ_BUFFER), [`GL_UNIFORM_BUFFER`](crate::enums::GL_UNIFORM_BUFFER)
    /// and [`GL_TEXTURE_BUFFER`](crate::enums::GL_TEXTURE_BUFFER) targets are
    /// available only if the GL version is 3.1 or greater.
    ///
    /// The [`GL_ATOMIC_COUNTER_BUFFER`](crate::enums::GL_ATOMIC_COUNTER_BUFFER)
    /// target is available only if the GL version is 4.2 or greater.
    ///
    /// The [`GL_DISPATCH_INDIRECT_BUFFER`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER)
    /// and [`GL_SHADER_STORAGE_BUFFER`](crate::enums::GL_SHADER_STORAGE_BUFFER)
    /// targets are available only if the GL version is 4.3 or greater.
    ///
    /// The [`GL_QUERY_BUFFER`](crate::enums::GL_QUERY_BUFFER) target is available
    /// only if the GL version is 4.4 or greater.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_ARRAY_BUFFER_BINDING`](crate::enums::GL_ARRAY_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_ATOMIC_COUNTER_BUFFER_BINDING`](crate::enums::GL_ATOMIC_COUNTER_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_COPY_READ_BUFFER_BINDING`](crate::enums::GL_COPY_READ_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_COPY_WRITE_BUFFER_BINDING`](crate::enums::GL_COPY_WRITE_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_DRAW_INDIRECT_BUFFER_BINDING`](crate::enums::GL_DRAW_INDIRECT_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_DISPATCH_INDIRECT_BUFFER_BINDING`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_ELEMENT_ARRAY_BUFFER_BINDING`](crate::enums::GL_ELEMENT_ARRAY_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_PACK_BUFFER_BINDING`](crate::enums::GL_PIXEL_PACK_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_UNPACK_BUFFER_BINDING`](crate::enums::GL_PIXEL_UNPACK_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_SHADER_STORAGE_BUFFER_BINDING`](crate::enums::GL_SHADER_STORAGE_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TRANSFORM_FEEDBACK_BUFFER_BINDING`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER_BINDING)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_UNIFORM_BUFFER_BINDING`](crate::enums::GL_UNIFORM_BUFFER_BINDING)

    pub fn oxidegl_bind_buffer(&mut self, target: BufferTarget, buffer: GLuint) {
        self.bind_buffer_internal(ObjectName::try_from_raw(buffer), target, NoIndex);
    }
    /// ### Parameters
    /// `buffer`
    ///
    /// > Specifies a value that may be the name of a buffer object.
    ///
    /// ### Description
    /// [**glIsBuffer**](crate::context::Context::oxidegl_is_buffer) returns [`GL_TRUE`](crate::enums::GL_TRUE)
    /// if `buffer` is currently the name of a buffer object. If `buffer` is zero,
    /// or is a non-zero value that is not currently the name of a buffer object,
    /// or if an error occurs, [**glIsBuffer**](crate::context::Context::oxidegl_is_buffer)
    /// returns [`GL_FALSE`](crate::enums::GL_FALSE).
    ///
    /// A name returned by [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers),
    /// but not yet associated with a buffer object by calling [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer),
    /// is not the name of a buffer object.

    pub fn oxidegl_is_buffer(&mut self, buffer: GLuint) -> GLboolean {
        self.gl_state.buffer_list.is_obj(buffer)
    }
    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of buffer objects to be deleted.
    ///
    /// `buffers`
    ///
    /// > Specifies an array of buffer objects to be deleted.
    ///
    /// ### Description
    /// [**glDeleteBuffers**](crate::context::Context::oxidegl_delete_buffers)
    /// deletes `n` buffer objects named by the elements of the array `buffers`.
    /// After a buffer object is deleted, it has no contents, and its name is
    /// free for reuse (for example by [**glGenBuffers**](crate::context::Context::oxidegl_gen_buffers)
    /// ). If a buffer object that is currently bound is deleted, the binding reverts
    /// to 0 (the absence of any buffer object).
    ///
    /// [**glDeleteBuffers**](crate::context::Context::oxidegl_delete_buffers)
    /// silently ignores 0's and names that do not correspond to existing buffer
    /// objects.
    ///
    /// ### Associated Gets
    /// [**glIsBuffer**](crate::context::Context::oxidegl_is_buffer)
    pub unsafe fn oxidegl_delete_buffers(&mut self, n: GLsizei, buffers: *const GLuint) {
        // Safety: Caller ensures validity
        unsafe {
            self.gl_state.buffer_list.delete_objects(n, buffers);
        }
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the buffer object is bound for [**glBufferStorage**](crate::context::Context::oxidegl_buffer_storage),
/// > which must be one of the buffer binding targets in the following table:
///
/// > | *Buffer Binding Target*                               | *Purpose*      |
/// > |-------------------------------------------------------|----------------|
/// > | [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER)    | Vertex attributes |
/// > | [`GL_ATOMIC_COUNTER_BUFFER`](crate::enums::GL_ATOMIC_COUNTER_BUFFER) | Atomic counter storage |
/// > | [`GL_COPY_READ_BUFFER`](crate::enums::GL_COPY_READ_BUFFER) | Buffer copy source |
/// > | [`GL_COPY_WRITE_BUFFER`](crate::enums::GL_COPY_WRITE_BUFFER) | Buffer copy destination |
/// > | [`GL_DISPATCH_INDIRECT_BUFFER`](crate::enums::GL_DISPATCH_INDIRECT_BUFFER) | Indirect compute dispatch commands |
/// > | [`GL_DRAW_INDIRECT_BUFFER`](crate::enums::GL_DRAW_INDIRECT_BUFFER) | Indirect command arguments |
/// > | [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER) | Vertex array indices |
/// > | [`GL_PIXEL_PACK_BUFFER`](crate::enums::GL_PIXEL_PACK_BUFFER) | Pixel read target |
/// > | [`GL_PIXEL_UNPACK_BUFFER`](crate::enums::GL_PIXEL_UNPACK_BUFFER) | Texture data source |
/// > | [`GL_QUERY_BUFFER`](crate::enums::GL_QUERY_BUFFER)    | Query result buffer |
/// > | [`GL_SHADER_STORAGE_BUFFER`](crate::enums::GL_SHADER_STORAGE_BUFFER) | Read-write storage for shaders |
/// > | [`GL_TEXTURE_BUFFER`](crate::enums::GL_TEXTURE_BUFFER) | Texture data buffer |
/// > | [`GL_TRANSFORM_FEEDBACK_BUFFER`](crate::enums::GL_TRANSFORM_FEEDBACK_BUFFER) | Transform feedback buffer |
/// > | [`GL_UNIFORM_BUFFER`](crate::enums::GL_UNIFORM_BUFFER) | Uniform block storage |
///
/// `buffer`
///
/// > Specifies the name of the buffer object for [**glNamedBufferStorage**](crate::context::Context::oxidegl_named_buffer_storage)
/// > function.
///
/// `size`
///
/// > Specifies the size in bytes of the buffer object's new data store.
///
/// `data`
///
/// > Specifies a pointer to data that will be copied into the data store for
/// > initialization, or [`NULL`](crate::enums::NULL) if no data is to be copied.
///
/// `flags`
///
/// > Specifies the intended usage of the buffer's data store. Must be a bitwise
/// > combination of the following flags. [`GL_DYNAMIC_STORAGE_BIT`](crate::enums::GL_DYNAMIC_STORAGE_BIT),
/// > [`GL_MAP_READ_BIT`](crate::enums::GL_MAP_READ_BIT) [`GL_MAP_WRITE_BIT`](crate::enums::GL_MAP_WRITE_BIT),
/// > [`GL_MAP_PERSISTENT_BIT`](crate::enums::GL_MAP_PERSISTENT_BIT), [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT),
/// > and [`GL_CLIENT_STORAGE_BIT`](crate::enums::GL_CLIENT_STORAGE_BIT).
///
/// ### Description
/// [**glBufferStorage**](crate::context::Context::oxidegl_buffer_storage)
/// and [**glNamedBufferStorage**](crate::context::Context::oxidegl_named_buffer_storage)
/// create a new immutable data store. For [**glBufferStorage**](crate::context::Context::oxidegl_buffer_storage),
/// the buffer object currently bound to `target` will be initialized. For
/// [**glNamedBufferStorage**](crate::context::Context::oxidegl_named_buffer_storage),
/// `buffer` is the name of the buffer object that will be configured. The
/// size of the data store is specified by `size`. If an initial data is available,
/// its address may be supplied in `data`. Otherwise, to create an uninitialized
/// data store, `data` should be [`NULL`](crate::enums::NULL).
///
/// The `flags` parameters specifies the intended usage of the buffer's data
/// store. It must be a bitwise combination of a subset of the following flags:
/// [`GL_DYNAMIC_STORAGE_BIT`](crate::enums::GL_DYNAMIC_STORAGE_BIT)
///
/// > The contents of the data store may be updated after creation through calls
/// > to [**glBufferSubData**](crate::context::Context::oxidegl_buffer_sub_data).
/// > If this bit is not set, the buffer content may not be directly updated
/// > by the client. The data argument may be used to specify the initial content
/// > of the buffer's data store regardless of the presence of the [`GL_DYNAMIC_STORAGE_BIT`](crate::enums::GL_DYNAMIC_STORAGE_BIT).
/// > Regardless of the presence of this bit, buffers may always be updated
/// > with server-side calls such as [**glCopyBufferSubData**](crate::context::Context::oxidegl_copy_buffer_sub_data)
/// > and [**glClearBufferSubData**](crate::context::Context::oxidegl_clear_buffer_sub_data).
///
/// [`GL_MAP_READ_BIT`](crate::enums::GL_MAP_READ_BIT)
///
/// > The data store may be mapped by the client for read access and a pointer
/// > in the client's address space obtained that may be read from.
///
/// [`GL_MAP_WRITE_BIT`](crate::enums::GL_MAP_WRITE_BIT)
///
/// > The data store may be mapped by the client for write access and a pointer
/// > in the client's address space obtained that may be written through.
///
/// [`GL_MAP_PERSISTENT_BIT`](crate::enums::GL_MAP_PERSISTENT_BIT)
///
/// > The client may request that the server read from or write to the buffer
/// > while it is mapped. The client's pointer to the data store remains valid
/// > so long as the data store is mapped, even during execution of drawing or
/// > dispatch commands.
///
/// [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT)
///
/// > Shared access to buffers that are simultaneously mapped for client access
/// > and are used by the server will be coherent, so long as that mapping is
/// > performed using [**glMapBufferRange**](crate::context::Context::oxidegl_map_buffer_range).
/// > That is, data written to the store by either the client or server will
/// > be immediately visible to the other with no further action taken by the
/// > application. In particular,
///
/// >> If [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT) is not set
/// >> and the client performs a write followed by a call to the [**glMemoryBarrier**](crate::context::Context::oxidegl_memory_barrier)
/// >> command with the [`GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT`](crate::enums::GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT)
/// >> set, then in subsequent commands the server will see the writes.
///
/// >> If [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT) is set and
/// >> the client performs a write, then in subsequent commands the server will
/// >> see the writes.
///
/// >> If [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT) is not set
/// >> and the server performs a write, the application must call [**glMemoryBarrier**](crate::context::Context::oxidegl_memory_barrier)
/// >> with the [`GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT`](crate::enums::GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT)
/// >> set and then call [**glFenceSync**](crate::context::Context::oxidegl_fence_sync)
/// >> with [`GL_SYNC_GPU_COMMANDS_COMPLETE`](crate::enums::GL_SYNC_GPU_COMMANDS_COMPLETE)
/// >> (or [**glFinish**](crate::context::Context::oxidegl_finish) ). Then the
/// >> CPU will see the writes after the sync is complete.
///
/// >> If [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT) is set and
/// >> the server does a write, the app must call [**glFenceSync**](crate::context::Context::oxidegl_fence_sync)
/// >> with [`GL_SYNC_GPU_COMMANDS_COMPLETE`](crate::enums::GL_SYNC_GPU_COMMANDS_COMPLETE)
/// >> (or [**glFinish**](crate::context::Context::oxidegl_finish) ). Then the
/// >> CPU will see the writes after the sync is complete.
///
/// [`GL_CLIENT_STORAGE_BIT`](crate::enums::GL_CLIENT_STORAGE_BIT)
///
/// > When all other criteria for the buffer storage allocation are met, this
/// > bit may be used by an implementation to determine whether to use storage
/// > that is local to the server or to the client to serve as the backing store
/// > for the buffer.
///
///
/// The allowed combinations of flags are subject to certain restrictions.
/// They are as follows: If `flags` contains [`GL_MAP_PERSISTENT_BIT`](crate::enums::GL_MAP_PERSISTENT_BIT),
/// > it must also contain at least one of [`GL_MAP_READ_BIT`](crate::enums::GL_MAP_READ_BIT)
/// > or [`GL_MAP_WRITE_BIT`](crate::enums::GL_MAP_WRITE_BIT).
///
/// > If `flags` contains [`GL_MAP_COHERENT_BIT`](crate::enums::GL_MAP_COHERENT_BIT),
/// > it must also contain [`GL_MAP_PERSISTENT_BIT`](crate::enums::GL_MAP_PERSISTENT_BIT).
///
///
/// ### Notes
/// [**glBufferStorage**](crate::context::Context::oxidegl_buffer_storage)
/// is available only if the GL version is 4.4 or greater.
///
/// [**glNamedBufferStorage**](crate::context::Context::oxidegl_named_buffer_storage)
/// is available only if the GL version is 4.5 or greater.
///
/// If `data` is [`NULL`](crate::enums::NULL), a data store of the specified
/// size is still created, but its contents remain uninitialized and thus undefined.
///
/// ### Associated Gets
/// [**glGetBufferSubData**](crate::context::Context::oxidegl_get_buffer_sub_data)
///
/// [**glGetBufferParameter**](crate::context::Context::oxidegl_get_buffer_parameter)
/// with argument [`GL_BUFFER_SIZE`](crate::enums::GL_BUFFER_SIZE) or [`GL_BUFFER_USAGE`](crate::enums::GL_BUFFER_USAGE)
impl Context {
    pub unsafe fn oxidegl_buffer_storage(
        &mut self,
        target: BufferStorageTarget,
        size: GLsizeiptr,
        data: *const GLvoid,
        flags: BufferStorageMask,
    ) {
        let binding = self
            .get_buffer_binding_mut(
                // Safety: BufferStorageTarget has the same set of valid values as BufferTarget
                unsafe { core::mem::transmute::<BufferStorageTarget, BufferTarget>(target) },
                NoIndex,
            )
            .expect("UB: tried to allocate storage for an unbound buffer binding");
        // Safety: Caller ensures data pointer is correctly initialized
        unsafe { self.buffer_storage_internal(binding, size, data, flags) };
    }
    pub unsafe fn oxidegl_named_buffer_storage(
        &mut self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const GLvoid,
        flags: BufferStorageMask,
    ) {
        let name = ObjectName::try_from_raw(buffer)
            .expect("UB: Tried to allocate storage for buffer name 0");
        debug!("Allocated {size} byte storage for {name:?}, initialized with ptr {data:?}");
        // Safety: Caller ensures data pointer is correctly initialized
        unsafe {
            self.buffer_storage_internal(name, size, data, flags);
        }
    }
}
impl Context {
    unsafe fn buffer_storage_internal(
        &mut self,
        // Not the right place to use MaybeObjectName because we need to abstract out which binding point this buffer is bound to (if any)
        name: ObjectName<Buffer>,
        size: GLsizeiptr,
        data: *const GLvoid,
        flags: BufferStorageMask,
    ) {
        self.gl_state
            .buffer_list
            .get_opt_mut(name)
            .expect("UB: buffer name not present in buffer list!");

        assert!(size >= 0, "UB: Tried to create buffer with negative length");
        #[allow(clippy::cast_sign_loss)]
        let size = size as usize;
        // TODO: lower-coherence storage modes (StorageModeManaged or single-upload StorageModePrivate).
        // Shared backing buffers are going to annihilate perf with larger buffers
        let options = MTLResourceOptions::MTLResourceStorageModeShared;

        let maybe_ptr = NonNull::new(data.cast_mut());

        let buffer;
        if let Some(ptr) = maybe_ptr {
            // Safety: caller ensures pointer validity, and that the slice implicitly formed by (data, size) is correctly initialized
            buffer = unsafe {
                self.platform_state
                    .device
                    .newBufferWithBytes_length_options(ptr, size, options)
            };
        } else {
            buffer = self
                .platform_state
                .device
                .newBufferWithLength_options(size, options);
        };
        let buffer = buffer.expect("INTERNAL ERROR: Metal Buffer allocation failed");
    }
}

impl Context {
    #[inline]
    pub(crate) fn get_buffer_binding_mut<I: IndexType>(
        &mut self,
        target: BufferTarget,
        idx: I,
    ) -> &mut Option<ObjectName<Buffer>> {
        match target {
            BufferTarget::UniformBuffer => {
                &mut self.gl_state.buffer_bindings.uniform[idx.get_numeric()]
            }
            BufferTarget::AtomicCounterBuffer => {
                &mut self.gl_state.buffer_bindings.atomic_counter[idx.get_numeric()]
            }
            BufferTarget::ShaderStorageBuffer => {
                &mut self.gl_state.buffer_bindings.shader_storage[idx.get_numeric()]
            }
            BufferTarget::TransformFeedbackBuffer => {
                &mut self.gl_state.buffer_bindings.transform_feedback[idx.get_numeric()]
            }
            t => {
                debug_assert!(
                    idx.get().is_none(),
                    "UB: Tried to bind at an index of a non-indexed binding target"
                );
                match t {
                    BufferTarget::ArrayBuffer => &mut self.gl_state.buffer_bindings.array,
                    BufferTarget::ElementArrayBuffer => {
                        &mut self.gl_state.buffer_bindings.element_array
                    }
                    BufferTarget::PixelPackBuffer => &mut self.gl_state.buffer_bindings.pixel_pack,
                    BufferTarget::PixelUnpackBuffer => {
                        &mut self.gl_state.buffer_bindings.pixel_unpack
                    }
                    BufferTarget::TextureBuffer => &mut self.gl_state.buffer_bindings.texture,
                    BufferTarget::CopyReadBuffer => &mut self.gl_state.buffer_bindings.copy_read,
                    BufferTarget::CopyWriteBuffer => &mut self.gl_state.buffer_bindings.copy_write,
                    BufferTarget::DrawIndirectBuffer => {
                        &mut self.gl_state.buffer_bindings.draw_indirect
                    }
                    BufferTarget::DispatchIndirectBuffer => {
                        &mut self.gl_state.buffer_bindings.dispatch_indirect
                    }
                    BufferTarget::QueryBuffer => &mut self.gl_state.buffer_bindings.query,
                    BufferTarget::ParameterBuffer => &mut self.gl_state.buffer_bindings.parameter,
                    #[allow(unused_unsafe)]
                    // Safety: all other variants are covered
                    _ => unsafe { debug_unreachable!() },
                }
            }
        }
    }
    pub(crate) fn bind_buffer_internal<I: IndexType>(
        &mut self,
        to_bind: Option<ObjectName<Buffer>>,
        target: BufferTarget,
        idx: I,
    ) {
        if let Some(maybe_named) = to_bind {
            self.gl_state
                .buffer_list
                .ensure_init(maybe_named, Buffer::new_default);
            debug_assert!(
                self.gl_state.buffer_list.is(maybe_named),
                "UB: Tried to bind an uninitialized buffer name to a VAO"
            );
        }
        let r = self.get_buffer_binding_mut(target, idx);

        *r = to_bind;
        debug!("bound buffer {to_bind:?} to target {target:?} at index {idx:?}");
    }
}

#[derive(Debug)]
/// Represents a GL buffer, tracking all of the state specified by the OpenGL spec, as well as a backing Metal buffer
///
/// ## Lifecycle
/// in OpenGL buffers have around three different states:
/// * Named: in this state there exists a u32 that uniquely identifies this slot in the buffer list.
///   As the reference page and spec note, the existence of a *name* does not imply the existence of a
///   buffer *object*. Buffer names are created by [glGenBuffers](Context::oxidegl_gen_buffers). This intermediate "named" state
///   can only be reached without DSA (glCreateBuffers initializes the buffers "as if \[they] had been bound to an unspecified target")
/// * Bound: in this state the "state vector" of the given buffer name is initialized and it is now a buffer object.
///   Note that binding a buffer does not immediately allocate it. Buffers are bound via [glBindBuffer](Context::oxidegl_bind_buffer), or created by glCreateBuffers
/// * Allocated: in this state the buffer has been fully initialized and is ready for use by the GL. Reached by [glBufferStorage](Context::oxidegl_buffer_storage)
pub(crate) struct Buffer {
    pub(crate) name: ObjectName<Self>,
    pub(crate) size: usize,
    pub(crate) usage: BufferUsage,
    pub(crate) access: BufferAccess,
    pub(crate) access_flags: MapBufferAccessMask,
    pub(crate) immutable_storage: bool,
    pub(crate) storage_flags: BufferStorageMask,
    pub(crate) allocation: Option<RealizedBufferInternal>,
}
#[derive(Debug)]
pub(crate) struct RealizedBufferInternal {
    pub(crate) mapping: Option<MappingInfo>,
    pub(crate) mtl: ProtoObjRef<dyn MTLBuffer>,
}
impl Buffer {
    // fn get_best_storage_mode_for_access_hint(access: BufferAccess, usage_hint: BufferUsage) -> MTLStorageMode {
    //     match usage_hint {
    //         // CPU Upload once, GPU read a few times
    //         BufferUsage::StreamDraw => todo!(),
    //         // Created from GPU data once, CPU read a few times
    //         BufferUsage::StreamRead => todo!(),
    //         // Created from GPU once, GPU read a few times
    //         BufferUsage::StreamCopy => todo!(),
    //         // CPU upload once, GPU read many times
    //         BufferUsage::StaticDraw => todo!(),
    //         // GPU create once, CPU read many times
    //         BufferUsage::StaticRead => todo!(),
    //         // GPU create once, GPU read many times
    //         BufferUsage::StaticCopy => todo!(),
    //         // CPU upload many times GPU read many times
    //         BufferUsage::DynamicDraw => todo!(),
    //         // GPU create many times, CPU read many times
    //         BufferUsage::DynamicRead => todo!(),
    //         // GPU create many times, GPU read many times
    //         BufferUsage::DynamicCopy => todo!(),
    //     }
    // }
    //
    pub(crate) fn new_default(name: ObjectName<Buffer>) -> Self {
        Self {
            name,
            size: 0,
            usage: BufferUsage::StaticDraw,
            access: BufferAccess::ReadWrite,
            access_flags: MapBufferAccessMask::empty(),
            immutable_storage: false,
            storage_flags: BufferStorageMask::empty(),
            allocation: None,
        }
    }
}

#[derive(Debug)]
/// Specifies the location of a memory-mapped buffer to the GL client
pub struct MappingInfo {
    /// Pointer to the mapped location in system memory
    pub(crate) ptr: NonNull<c_void>,
    /// Offset from the start of the buffer to the start of the mapped region
    pub(crate) ptr_offset: usize,
    /// Length of the region of the buffer which has been mapped
    pub(crate) len: usize,
}

impl NamedObject for Buffer {}
