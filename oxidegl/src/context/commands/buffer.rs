use core::{ffi::c_void, fmt::Debug, ptr::NonNull};

use log::debug;
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLBuffer;

use crate::{
    context::{
        state::{NamedObject, ObjectName},
        Context,
    },
    debug_unreachable,
    dispatch::{
        conversions::{IndexType, NoIndex},
        gl_types::{GLboolean, GLsizei, GLuint},
    },
    enums::{BufferAccess, BufferStorageMask, BufferTarget, BufferUsage, MapBufferAccessMask},
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
        self.oxidegl_bind_buffer_internal(ObjectName::from_raw(buffer), target, NoIndex);
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

impl Context {
    fn oxidegl_bind_buffer_internal<I: IndexType>(
        &mut self,
        to_bind: Option<ObjectName<Buffer>>,
        target: BufferTarget,
        idx: I,
    ) {
        self.dirty_buffers();
        if let Some(maybe_named) = to_bind {
            self.gl_state
                .buffer_list
                .ensure_init(maybe_named, Buffer::new_default);
            debug_assert!(
                self.gl_state.buffer_list.is(maybe_named),
                "UB: Tried to bind an uninitialized buffer name to a VAO"
            );
        }

        let bindings = &mut self.gl_state.buffer_bindings;
        let r = match target {
            // Safety: Caller ensures idx is in-bounds for indexed targets
            BufferTarget::UniformBuffer => unsafe {
                bindings.uniform.get_unchecked_mut(idx.get_numeric())
            },
            //Safety: see above
            BufferTarget::AtomicCounterBuffer => unsafe {
                bindings.atomic_counter.get_unchecked_mut(idx.get_numeric())
            },
            //Safety: see above
            BufferTarget::ShaderStorageBuffer => unsafe {
                bindings.shader_storage.get_unchecked_mut(idx.get_numeric())
            },
            //Safety: see above
            BufferTarget::TransformFeedbackBuffer => unsafe {
                bindings
                    .transform_feedback
                    .get_unchecked_mut(idx.get_numeric())
            },
            t => {
                debug_assert!(
                    idx.get().is_none(),
                    "UB: Tried to bind at an index of a non-indexed binding target"
                );
                &mut match t {
                    BufferTarget::ArrayBuffer => bindings.array,
                    BufferTarget::ElementArrayBuffer => bindings.element_array,
                    BufferTarget::PixelPackBuffer => bindings.pixel_pack,
                    BufferTarget::PixelUnpackBuffer => bindings.pixel_unpack,
                    BufferTarget::TextureBuffer => bindings.texture,
                    BufferTarget::CopyReadBuffer => bindings.copy_read,
                    BufferTarget::CopyWriteBuffer => bindings.copy_write,
                    BufferTarget::DrawIndirectBuffer => bindings.draw_indirect,
                    BufferTarget::DispatchIndirectBuffer => bindings.dispatch_indirect,
                    BufferTarget::QueryBuffer => bindings.query,
                    BufferTarget::ParameterBuffer => bindings.parameter,
                    #[allow(unused_unsafe)]
                    // Safety: all other variants are covered
                    _ => unsafe { debug_unreachable!() },
                }
            }
        };
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
/// As the reference page and spec note, the existence of a *name* does not imply the existence of a
/// buffer *object*. Buffer names are created by [glGenBuffers](Context::oxidegl_gen_buffers). This intermediate "named" state
/// can only be reached without DSA (glCreateBuffers initializes the buffers "as if \[they] had been bound to an unspecified target")
/// * Bound: in this state the "state vector" of the given buffer name is initialized and it is now a buffer object.
/// Note that binding a buffer does not immediately allocate it. Buffers are bound via [glBindBuffer](Context::oxidegl_bind_buffer), or created by glCreateBuffers
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
    pub(crate) mtl: Retained<ProtocolObject<dyn MTLBuffer>>,
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
