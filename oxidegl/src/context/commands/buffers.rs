use std::{fmt::Debug, num::NonZeroU32, os::raw::c_void, ptr::NonNull};

use log::debug;
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLBuffer;

use crate::{
    context::Context,
    debug_unreachable,
    dispatch::{
        conversions::{GLenumExt, GlDstType, SrcType, UnsafeFromGLenum},
        gl_types::{GLboolean, GLsizei, GLuint},
    },
    enums::{BufferAccess, BufferStorageMask, BufferTarget, BufferUsage, MapBufferAccessMask},
    OptionResultExt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BufferName(NonZeroU32);
impl BufferName {
    pub fn new(val: u32) -> Option<Self> {
        Some(Self(NonZeroU32::new(val)?))
    }
}

impl UnsafeFromGLenum for BufferName {
    #[inline]
    unsafe fn unsafe_from_gl_enum(val: GLuint) -> Self {
        #[cfg(debug_assertions)]
        return Self(NonZeroU32::try_from(val).expect("0 is not a valid buffer name!"));

        #[cfg(not(debug_assertions))]
        return unsafe { core::mem::transmute(val) };
    }
}
impl<Dst: GlDstType> SrcType<Dst> for Option<BufferName> {
    fn cast(self) -> Dst {
        Dst::from_uint(self.map_or(0, |s| s.0.get()))
    }
}
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
        debug_assert!(n >= 0, "UB: cannot create a negative amount of buffers");
        debug_assert!(!buffers.is_null(), "UB: buffer array pointer was null");
        debug_assert!(
            buffers.is_aligned(),
            "UB: buffer array pointer was not sufficiently aligned"
        );
        debug!("writing {n} new buffer names to {buffers:?}");

        for _ in 0..n {
            let name = self.gl_state.buffer_list.new_buffer_name();
        }
        panic!("command oxidegl_gen_buffers not yet implemented");
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
        debug_assert!(n >= 0, "UB: cannot create a negative amount of buffers");
        debug_assert!(!buffers.is_null(), "UB: buffer array pointer was null");
        debug_assert!(
            buffers.is_aligned(),
            "UB: buffer array pointer was not sufficiently aligned"
        );

        panic!("command oxidegl_create_buffers not yet implemented");
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
        panic!("command oxidegl_bind_buffer not yet implemented");
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
        let Some(pname) = BufferName::new(buffer) else {
            return false;
        };
        self.gl_state.buffer_list.is_buffer(pname)
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
        debug_assert!(n >= 0, "UB: cannot delete a negative amount of buffers");
        debug_assert!(!buffers.is_null(), "UB: buffer array pointer was null");
        debug_assert!(
            buffers.is_aligned(),
            "UB: buffer array pointer was not sufficiently aligned"
        );
        panic!("command oxidegl_delete_buffers not yet implemented");
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
/// Note that binding a buffer does not immediately allocate it. Buffers are bound via glBindBuffers, or created by glCreateBuffers
/// * Allocated: in this state the buffer has been fully initialized and is ready for use by the GL. Reached by glBufferStorage
pub(crate) struct Buffer {
    pub(crate) name: BufferName,
    pub(crate) current_binding: Option<BindingInfo>,
    pub(crate) size: usize,
    pub(crate) usage: BufferUsage,
    pub(crate) access: BufferAccess,
    pub(crate) access_flags: MapBufferAccessMask,
    pub(crate) immutable_storage: bool,
    pub(crate) storage_flags: BufferStorageMask,
    pub(crate) allocation: Option<RealizedBufferInternal>,
}
#[derive(Debug)]
/// Repre
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
    pub(crate) fn new_default(name: BufferName) -> Self {
        Self {
            name,
            current_binding: None,
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
#[derive(Debug)]
pub struct BindingInfo {
    /// The binding target this buffer is currently bound to
    pub(crate) target: BufferTarget,
    /// The index of the binding of this buffer if the target is an indexed target, 0 if not
    pub(crate) index: GLuint,
}

#[derive(Debug)]
/// Tracks the state of a given buffer *name* throughout the GL server lifetime
pub enum BufferNameState {
    /// No buffer is present; the buffer previously resident in this name has been deleted
    Empty,
    /// Indeterminate state that lies between glGenBuffers and glBindBuffers for non-DSA access
    Named,
    /// This buffer name is *bound* to a buffer object with the given state
    Bound(Buffer),
}
#[derive(Debug)]
pub struct BufferList {
    buffers: Vec<BufferNameState>,
}
impl BufferList {
    pub(crate) fn new() -> Self {
        Self {
            buffers: Vec::with_capacity(32),
        }
    }
    pub(crate) fn get_buffer(&self, name: BufferName) -> Option<&Buffer> {
        self.buffers
            .get((name.0.get() - 1) as usize)
            .and_then(|name_state| match name_state {
                BufferNameState::Bound(ref b) => Some(b),
                _ => None,
            })
    }
    pub(crate) unsafe fn get_buffer_unchecked(&self, name: BufferName) -> &Buffer {
        // Safety: Caller ensures that the buffer at name exists in the buffer list
        unsafe {
            match self
                .buffers
                .get((name.0.get() - 1) as usize)
                .debug_expect("UB: Tried to use a buffer name that didn't exist in the list!")
            {
                BufferNameState::Bound(ref b) => b,
                _ => {
                    debug_unreachable!(unsafe "UB: Tried to get a buffer with a name that has not yet been initialized")
                }
            }
        }
    }
    pub(crate) fn get_buffer_mut(&mut self, name: BufferName) -> Option<&mut Buffer> {
        self.buffers
            .get_mut((name.0.get() - 1) as usize)
            .and_then(|name_state| match name_state {
                BufferNameState::Bound(b) => Some(b),
                _ => None,
            })
    }
    pub(crate) unsafe fn get_buffer_unchecked_mut(&mut self, name: BufferName) -> &mut Buffer {
        // Safety: Caller ensures that the buffer at name exists in the buffer list
        unsafe {
            match self
                .buffers
                .get_mut((name.0.get() - 1) as usize)
                .debug_expect("UB: Tried to use a buffer name that didn't exist in the list!")
            {
                BufferNameState::Bound(b) => b,
                _ => {
                    debug_unreachable!(unsafe "UB: Tried to get a buffer with a name that has not yet been initialized")
                }
            }
        }
    }
    // Overflow is checked
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn new_buffer_name(&mut self) -> BufferName {
        debug_assert!(
            self.buffers.len() < (u32::MAX - 1) as usize,
            "UB: OxideGL does not allow generation of more than u32::MAX buffer names"
        );
        // Safety: 1 is added to self.buffers.len(), so it will not be 0. If someone manages to create u32::MAX buffer names
        // with a release build of OxideGL they will have bigger problems to deal with than an integer overflow
        let name = unsafe {
            BufferName(
                NonZeroU32::new((self.buffers.len() + 1) as u32)
                    .debug_expect("Overflow in buffer name creation"),
            )
        };
        self.buffers.push(BufferNameState::Named);
        name
    }
    // Overflow is checked
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn new_buffer(&mut self, buf: Buffer) -> BufferName {
        debug_assert!(
            self.buffers.len() < (u32::MAX - 1) as usize,
            "UB: OxideGL does not allow generation of more than u32::MAX buffer names"
        );
        // Safety: 1 is added to self.buffers.len(), so it will not be 0. If someone manages to create u32::MAX buffer names
        // with a release build of OxideGL they will have bigger problems to deal with than an integer overflow
        let name = unsafe {
            BufferName(
                NonZeroU32::new((self.buffers.len() + 1) as u32)
                    .debug_expect("Overflow in buffer name creation"),
            )
        };
        self.buffers.push(BufferNameState::Bound(buf));
        name
    }
    pub(crate) fn is_buffer(&self, name: BufferName) -> bool {
        self.get_buffer(name).is_some()
    }
}
