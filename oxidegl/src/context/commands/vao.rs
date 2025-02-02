use core::slice;
use std::{array, iter, num::NonZeroU32};

use log::{debug, trace};
use objc2_metal::MTLVertexFormat;

use crate::{
    context::{
        commands::buffer::Buffer,
        debug::gl_debug,
        error::{gl_assert, GlError, GlFallible},
        gl_object::{LateInit, NamedObject, ObjectName},
        Context,
    },
    dispatch::{
        conversions::{sizei, CurrentBinding, MaybeObjectName},
        gl_types::{GLboolean, GLenum, GLint, GLintptr, GLsizei, GLuint, GLvoid},
    },
    enums::{VertexAttribIType, VertexAttribPointerType, VertexAttribType},
    run_if_changed,
};

//TODO glGetVertexAttribiv
//TODO move logical components out of this file, should be ffi only

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object for [**glVertexArrayAttrib{I,
/// > L}Format**](crate::context::Context::oxidegl_vertex_array_attrib{_i, _l}_format)
/// > functions.
///
/// `attribindex`
///
/// > The generic vertex attribute array being described.
///
/// `size`
///
/// > The number of values per vertex that are stored in the array.
///
/// `type`
///
/// > The type of the data stored in the array.
///
/// `normalized`
///
/// > Specifies whether fixed-point data values should be normalized( [`GL_TRUE`](crate::enums::GL_TRUE))
/// > or converted directly as fixed-point values( [`GL_FALSE`](crate::enums::GL_FALSE))
/// > when they are accessed. This parameter is ignored if `type` is [`GL_FIXED`](crate::enums::GL_FIXED).
///
/// `relativeoffset`
///
/// > The distance between elements within the buffer.
///
/// ### Description
/// [**glVertexAttribFormat**](crate::context::Context::oxidegl_vertex_attrib_format),
/// [**glVertexAttribIFormat**](crate::context::Context::oxidegl_vertex_attrib_i_format)
/// and [**glVertexAttribLFormat**](crate::context::Context::oxidegl_vertex_attrib_l_format),
/// as well as [**glVertexArrayAttribFormat**](crate::context::Context::oxidegl_vertex_array_attrib_format),
/// [**glVertexArrayAttribIFormat**](crate::context::Context::oxidegl_vertex_array_attrib_i_format)
/// and [**glVertexArrayAttribLFormat**](crate::context::Context::oxidegl_vertex_array_attrib_l_format)
/// specify the organization of data in vertex arrays. The first three calls
/// operate on the bound vertex array object, whereas the last three ones modify
/// the state of a vertex array object with ID `vaobj`. `attribindex` specifies
/// the index of the generic vertex attribute array whose data layout is being
/// described, and must be less than the value of [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS).
///
/// `size` determines the number of components per vertex are allocated to
/// the specified attribute and must be 1, 2, 3, 4, or `type` indicates the
/// type of the data. If `type` is one of [`GL_BYTE`](crate::enums::GL_BYTE),
/// [`GL_SHORT`](crate::enums::GL_SHORT), [`GL_INT`](crate::enums::GL_INT),
/// [`GL_FIXED`](crate::enums::GL_FIXED), [`GL_FLOAT`](crate::enums::GL_FLOAT),
/// [`GL_HALF_FLOAT`](crate::enums::GL_HALF_FLOAT), and [`GL_DOUBLE`](crate::enums::GL_DOUBLE)
/// indicate types [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE), [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT),
/// and [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT) indicate types
/// [`GL_INT_2_10_10_10_REV`](crate::enums::GL_INT_2_10_10_10_REV) and [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::enums::GL_UNSIGNED_INT_2_10_10_10_REV)
/// indicating respectively four signed or unsigned elements packed into a
/// single [`GL_UNSIGNED_INT_10F_11F_11F_REV`](crate::enums::GL_UNSIGNED_INT_10F_11F_11F_REV)
/// indicating three floating point values packed into a single
///
/// [**glVertexAttribLFormat**](crate::context::Context::oxidegl_vertex_attrib_l_format)
/// and [**glVertexArrayAttribLFormat**](crate::context::Context::oxidegl_vertex_array_attrib_l_format)
/// is used to specify layout for data associated with a generic attribute
/// variable declared as 64-bit double precision components. For [**glVertexAttribLFormat**](crate::context::Context::oxidegl_vertex_attrib_l_format)
/// and [**glVertexArrayAttribLFormat**](crate::context::Context::oxidegl_vertex_array_attrib_l_format),
/// `type` must be [`GL_DOUBLE`](crate::enums::GL_DOUBLE). In contrast to [**glVertexAttribFormat**](crate::context::Context::oxidegl_vertex_attrib_format)
/// or [**glVertexArrayAttribFormat**](crate::context::Context::oxidegl_vertex_array_attrib_format),
/// which will cause data declared as [`GL_DOUBLE`](crate::enums::GL_DOUBLE)
/// to be converted to 32-bit representation, [**glVertexAttribLFormat**](crate::context::Context::oxidegl_vertex_attrib_l_format)
/// and [**glVertexArrayAttribLFormat**](crate::context::Context::oxidegl_vertex_array_attrib_l_format)
/// cause such data to be left in its natural, 64-bit representation.
///
/// For [**glVertexAttribFormat**](crate::context::Context::oxidegl_vertex_attrib_format)
/// and [**glVertexArrayAttribFormat**](crate::context::Context::oxidegl_vertex_array_attrib_format),
/// if `normalized` is [`GL_TRUE`](crate::enums::GL_TRUE), then integer data
/// is normalized to the range \[-1, 1\] or \[0, 1\] if it is signed or unsigned,
/// respectively. If `normalized` is [`GL_FALSE`](crate::enums::GL_FALSE) then
/// integer data is directly converted to floating point.
///
/// `relativeoffset` is the offset, measured in basic machine units of the
/// first element relative to the start of the vertex buffer binding this attribute
/// fetches from.
///
/// [**glVertexAttribFormat**](crate::context::Context::oxidegl_vertex_attrib_format)
/// and [**glVertexArrayAttribFormat**](crate::context::Context::oxidegl_vertex_array_attrib_format)
/// should be used to describe vertex attribute layout for floating-point vertex
/// attributes, [**glVertexAttribIFormat**](crate::context::Context::oxidegl_vertex_attrib_i_format)
/// and [**glVertexArrayAttribIFormat**](crate::context::Context::oxidegl_vertex_array_attrib_i_format)
/// should be used to describe vertex attribute layout for integer vertex attribute,
/// and [**glVertexAttribLFormat**](crate::context::Context::oxidegl_vertex_attrib_l_format)
/// and [**glVertexArrayAttribLFormat**](crate::context::Context::oxidegl_vertex_array_attrib_l_format)
/// should be used to describe the layout for 64-bit vertex attributes. Data
/// for an array specified by [**glVertexAttribIFormat**](crate::context::Context::oxidegl_vertex_attrib_i_format)
/// and [**glVertexArrayAttribIFormat**](crate::context::Context::oxidegl_vertex_array_attrib_i_format)
/// will always be left as integer values; such data are referred to as pure
/// integers.
///
/// ### Notes
/// [`GL_UNSIGNED_INT_10F_11F_11F_REV`](crate::enums::GL_UNSIGNED_INT_10F_11F_11F_REV)
/// is accepted for `type` only if the GL version is 4.4 or higher.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with arguments [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS),
/// or [`GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET`](crate::enums::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET).
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with argument [`GL_VERTEX_ATTRIB_RELATIVE_OFFSET`](crate::enums::GL_VERTEX_ATTRIB_RELATIVE_OFFSET).
impl Context {
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn oxidegl_vertex_attrib_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribType,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) -> GlFallible {
        self.oxidegl_vertex_attrib_format_internal(
            attribindex,
            size as u32,
            r#type,
            relativeoffset,
            IntegralCastBehavior::Cast,
            CurrentBinding,
        )
    }
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn oxidegl_vertex_attrib_i_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribIType,
        relativeoffset: GLuint,
    ) -> GlFallible {
        self.oxidegl_vertex_attrib_format_internal(
            attribindex,
            size as u32,
            // Safety: transmute is sound because the allowed values of VertexAttribIType are
            // a strict subset of the allowed values of VertexAttribType, and the two are otherwise valid to transmute between
            unsafe { core::mem::transmute::<VertexAttribIType, VertexAttribType>(r#type) },
            relativeoffset,
            IntegralCastBehavior::Native,
            CurrentBinding,
        )
    }
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn oxidegl_vertex_array_attrib_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribType,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) -> GlFallible {
        self.oxidegl_vertex_attrib_format_internal(
            attribindex,
            size as u32,
            r#type,
            relativeoffset,
            IntegralCastBehavior::Cast,
            vaobj,
        )
    }
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn oxidegl_vertex_array_attrib_i_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribIType,
        relativeoffset: GLuint,
    ) -> GlFallible {
        self.oxidegl_vertex_attrib_format_internal(
            attribindex,
            size as u32,
            // Safety: transmute is sound because the allowed values of VertexAttribIType are
            // a strict subset of the allowed values of VertexAttribType, and the two are otherwise valid to transmute between
            unsafe { core::mem::transmute::<VertexAttribIType, VertexAttribType>(r#type) },
            relativeoffset,
            IntegralCastBehavior::Native,
            vaobj,
        )
    }
    pub(crate) fn oxidegl_vertex_array_attrib_l_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("OxideGL does not currently support vertex attributes of double type");
    }
    pub(crate) fn oxidegl_vertex_attrib_l_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("OxideGL does not currently support vertex attributes of double type");
    }
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn oxidegl_vertex_attrib_format_internal(
        &mut self,
        attrib_index: u32,
        num_components: u32,
        r#type: VertexAttribType,
        relative_offset: GLuint,
        integer_behavior: IntegralCastBehavior,
        maybe_name: impl MaybeObjectName<Vao>,
    ) -> GlFallible {
        let vao = self.get_vao(maybe_name)?;
        let attrib = vao.get_attrib_mut(attrib_index)?;
        gl_assert!(
            relative_offset <= u32::from(MAX_VERTEX_ATTRIBUTE_STRIDE),
            InvalidValue
        );
        // Caller ensures num_components is in-bounds
        attrib.components = num_components as u8;
        attrib.component_type = r#type;

        attrib.relative_offset = relative_offset as u16;
        attrib.validate()?;
        gl_debug!("updated vertex attribute format at index {attrib_index} of {:#?} to {:?}*{num_components} with relative offset {relative_offset} and integer behavior {integer_behavior:?}", vao.name, r#type);
        Ok(())
    }
    #[inline]
    pub(crate) fn get_vao(
        &mut self,
        maybe_name: impl MaybeObjectName<Vao>,
    ) -> GlFallible<&mut Vao> {
        if let Some(name) = maybe_name.get() {
            self.gl_state
                .vao_list
                .get_opt_mut(name)
                .ok_or(GlError::InvalidOperation.e())
        } else {
            self.gl_state
                .vao_list
                .get_opt_mut(
                    self.gl_state
                        .vao_binding
                        .ok_or(GlError::InvalidOperation.e())?,
                )
                .ok_or(GlError::InvalidOperation.e())
        }
    }
}

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object to be used by [**glVertexArrayVertexBuffer**](crate::context::Context::oxidegl_vertex_array_vertex_buffer)
/// > function.
///
/// `bindingindex`
///
/// > The index of the vertex buffer binding point to which to bind the buffer.
///
/// `buffer`
///
/// > The name of a buffer to bind to the vertex buffer binding point.
///
/// `offset`
///
/// > The offset of the first element of the buffer.
///
/// `stride`
///
/// > The distance between elements within the buffer.
///
/// ### Description
/// [**glBindVertexBuffer**](crate::context::Context::oxidegl_bind_vertex_buffer)
/// and [**glVertexArrayVertexBuffer**](crate::context::Context::oxidegl_vertex_array_vertex_buffer)
/// bind the buffer named `buffer` to the vertex buffer binding point whose
/// index is given by `bindingindex`. [**glBindVertexBuffer**](crate::context::Context::oxidegl_bind_vertex_buffer)
/// modifies the binding of the currently bound vertex array object, whereas
/// [**glVertexArrayVertexBuffer**](crate::context::Context::oxidegl_vertex_array_vertex_buffer)
/// allows the caller to specify ID of the vertex array object with an argument
/// named `vaobj`, for which the binding should be modified. `offset` and `stride`
/// specify the offset of the first element within the buffer and the distance
/// between elements within the buffer, respectively, and are both measured
/// in basic machine units. `bindingindex` must be less than the value of [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS).
/// `offset` and `stride` must be greater than or equal to zero. If `buffer`
/// is zero, then any buffer currently bound to the specified binding point
/// is unbound.
///
/// If `buffer` is not the name of an existing buffer object, the GL first
/// creates a new state vector, initialized with a zero-sized memory buffer
/// and comprising all the state and with the same initial values as in case
/// of [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer). `buffer`
/// is then attached to the specified `bindingindex` of the vertex array object.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS).
impl Context {
    pub(crate) fn oxidegl_bind_vertex_buffer(
        &mut self,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) -> GlFallible {
        self.vertex_array_vertex_buffers_internal(
            CurrentBinding,
            bindingindex,
            iter::once(ObjectName::try_from_raw(buffer).ok()),
            iter::once(offset),
            iter::once(stride),
        )
    }
    pub(crate) fn oxidegl_vertex_array_vertex_buffer(
        &mut self,
        vaobj: GLuint,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) -> GlFallible {
        self.vertex_array_vertex_buffers_internal(
            vaobj,
            bindingindex,
            iter::once(ObjectName::try_from_raw(buffer).ok()),
            iter::once(offset),
            iter::once(stride),
        )
    }
}
impl Context {
    #[inline]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless
    )]
    pub(crate) fn vertex_array_vertex_buffers_internal(
        &mut self,
        vao: impl MaybeObjectName<Vao>,
        idx: u32,
        buffers: impl ExactSizeIterator<Item = Option<ObjectName<Buffer>>> + Clone,
        offsets: impl ExactSizeIterator<Item = GLintptr>,
        strides: impl ExactSizeIterator<Item = GLsizei>,
    ) -> GlFallible {
        buffers.clone().try_for_each(|v| -> GlFallible {
            gl_assert!(
                v.is_none_or(|n| self.gl_state.buffer_list.is(n)),
                InvalidOperation,
                "VAO buffer binding buffer name must be zero or a valid buffer name"
            );
            Ok(())
        })?;
        let vao = self.get_vao(vao)?;
        let vao_name = vao.name;

        let mut bindingindex = idx;
        for (name, (offset, stride)) in buffers.zip(offsets.zip(strides)) {
            sizei!(stride);
            gl_assert!(offset >= 0, InvalidValue);
            gl_assert!(
                stride <= MAX_VERTEX_ATTRIBUTE_STRIDE as u32,
                InvalidValue,
                "Vertex attribute binding stride larger than MAX_VERTEX_ATTRIBUTE_STRIDE"
            );
            let r = vao.get_binding_mut(bindingindex)?;
            debug!("bound {name:?} to {:?} at binding index {bindingindex} with offset {offset} and stride {stride}", vao_name);
            r.buf = name;
            r.offset = offset as usize;
            r.stride = stride as u16;

            bindingindex += 1;
        }
        // mark dirty state if this vao is current
        if self.gl_state.vao_binding == Some(vao_name) {
            self.remap_buffers();
            self.update_encoder();
        }
        Ok(())
    }
}

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object for [**glVertexArrayVertexBuffers**](crate::context::Context::oxidegl_vertex_array_vertex_buffers).
///
/// `first`
///
/// > Specifies the first vertex buffer binding point to which a buffer object
/// > is to be bound.
///
/// `count`
///
/// > Specifies the number of buffers to bind.
///
/// `buffers`
///
/// > Specifies the address of an array of names of existing buffer objects.
///
/// `offsets`
///
/// > Specifies the address of an array of offsets to associate with the binding
/// > points.
///
/// `strides`
///
/// > Specifies the address of an array of strides to associate with the binding
/// > points.
///
/// ### Description
/// [**glBindVertexBuffers**](crate::context::Context::oxidegl_bind_vertex_buffers)
/// and [**glVertexArrayVertexBuffers**](crate::context::Context::oxidegl_vertex_array_vertex_buffers)
/// bind storage from an array of existing buffer objects to a specified number
/// of consecutive vertex buffer binding points units in a vertex array object.
/// For [**glBindVertexBuffers**](crate::context::Context::oxidegl_bind_vertex_buffers),
/// the vertex array object is the currently bound vertex array object. For
/// [**glVertexArrayVertexBuffers**](crate::context::Context::oxidegl_vertex_array_vertex_buffers),
/// `vaobj` is the name of the vertex array object.
///
/// `count` existing buffer objects are bound to vertex buffer binding points
/// numbered $first$ through $first+ count- 1$. If `buffers` is not NULL, it
/// specifies an array of `count` values, each of which must be zero or the
/// name of an existing buffer object. `offsets` and `strides` specify arrays
/// of `count` values indicating the offset of the first element and stride
/// between elements in each buffer, respectively. If `buffers` is NULL, each
/// affected vertex buffer binding point from $first$ through $first+ count
///- 1$ will be reset to have no bound buffer object. In this case, the offsets
///  and strides associated with the binding points are set to default values,
///  ignoring `offsets` and `strides`.
///
/// [**glBindVertexBuffers**](crate::context::Context::oxidegl_bind_vertex_buffers)
/// is equivalent (assuming no errors are generated) to:
///
/// [**glVertexArrayVertexBuffers**](crate::context::Context::oxidegl_vertex_array_vertex_buffers)
/// is equivalent to the pseudocode above, but replacing [**glBindVertexBuffers**](crate::context::Context::oxidegl_bind_vertex_buffers)
/// (args) with [**glVertexArrayVertexBuffers**](crate::context::Context::oxidegl_vertex_array_vertex_buffers)
/// (vaobj, args).
///
/// The values specified in `buffers`, `offsets`, and `strides` will be checked
/// separately for each vertex buffer binding point. When a value for a specific
/// vertex buffer binding point is invalid, the state for that binding point
/// will be unchanged and an error will be generated. However, state for other
/// vertex buffer binding points will still be changed if their corresponding
/// values are valid.
impl Context {
    pub(crate) unsafe fn oxidegl_bind_vertex_buffers(
        &mut self,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) -> GlFallible {
        sizei!(count);
        self.vertex_array_vertex_buffers_internal(
            CurrentBinding,
            first,
            // Safety: slices invariants are upheld by the caller
            unsafe {
                slice::from_raw_parts(buffers.cast::<Option<ObjectName<Buffer>>>(), count as usize)
            }
            .iter()
            .copied(),
            // Safety: see above ^
            unsafe { slice::from_raw_parts(offsets, count as usize) }
                .iter()
                .copied(),
            // Safety: see above ^
            unsafe { slice::from_raw_parts(strides, count as usize) }
                .iter()
                .copied(),
        )
    }
    pub(crate) unsafe fn oxidegl_vertex_array_vertex_buffers(
        &mut self,
        vaobj: GLuint,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) -> GlFallible {
        sizei!(count);
        self.vertex_array_vertex_buffers_internal(
            vaobj,
            first,
            // Safety: slices invariants are upheld by the caller
            unsafe {
                slice::from_raw_parts(buffers.cast::<Option<ObjectName<Buffer>>>(), count as usize)
            }
            .iter()
            .copied(),
            // Safety: see above ^
            unsafe { slice::from_raw_parts(offsets, count as usize) }
                .iter()
                .copied(),
            // Safety: see above ^
            unsafe { slice::from_raw_parts(strides, count as usize) }
                .iter()
                .copied(),
        )
    }
}

// Generic VAO object creation and destruction
impl Context {
    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of vertex array object names to generate.
    ///
    /// `arrays`
    ///
    /// > Specifies an array in which the generated vertex array object names are
    /// > stored.
    ///
    /// ### Description
    /// [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays)
    /// returns `n` vertex array object names in `arrays`. There is no guarantee
    /// that the names form a contiguous set of integers; however, it is guaranteed
    /// that none of the returned names was in use immediately before the call
    /// to [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays).
    ///
    /// Vertex array object names returned by a call to [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays)
    /// are not returned by subsequent calls, unless they are first deleted with
    /// [**glDeleteVertexArrays**](crate::context::Context::oxidegl_delete_vertex_arrays).
    ///
    /// The names returned in `arrays` are marked as used, for the purposes of
    /// [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays)
    /// only, but they acquire state and type only when they are first bound.
    pub(crate) unsafe fn oxidegl_gen_vertex_arrays(&mut self, n: GLsizei, arrays: *mut GLuint) {
        // Safety: Caller guarantees invariants are upheld
        unsafe {
            self.gl_state.vao_list.gen_obj(n, arrays);
        }
    }

    /// ### Parameters
    /// `n`
    ///
    /// > Number of vertex array objects to create.
    ///
    /// `arrays`
    ///
    /// > Specifies an array in which names of the new vertex array objects are stored.
    ///
    /// ### Description
    /// [**glCreateVertexArrays**](crate::context::Context::oxidegl_create_vertex_arrays)
    /// returns `n` previously unused vertex array object names in `arrays`, each
    /// representing a new vertex array object initialized to the default state.
    pub(crate) unsafe fn oxidegl_create_vertex_arrays(&mut self, n: GLsizei, arrays: *mut GLuint) {
        // Safety: Caller guarantees invariants are upheld
        unsafe {
            self.gl_state
                .vao_list
                .create_obj(Vao::new_default, n, arrays);
        }
    }

    /// ### Parameters
    /// `array`
    ///
    /// > Specifies the name of the vertex array to bind.
    ///
    /// ### Description
    /// [**glBindVertexArray**](crate::context::Context::oxidegl_bind_vertex_array)
    /// binds the vertex array object with name `array`. `array` is the name of
    /// a vertex array object previously returned from a call to [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays),
    /// or zero to break the existing vertex array object binding.
    ///
    /// If no vertex array object with name `array` exists, one is created when
    /// `array` is first bound. If the bind is successful no change is made to
    /// the state of the vertex array object, and any previous vertex array object
    /// binding is broken.
    pub(crate) fn oxidegl_bind_vertex_array(&mut self, array: GLuint) -> GlFallible {
        let name = ObjectName::try_from_raw(array).ok();
        if let Some(name) = name {
            self.gl_state.vao_list.ensure_init(name, Vao::new_default)?;
        }
        trace!("bound {name:?} as current VAO");
        run_if_changed!(self.gl_state.vao_binding;= name => {
                self.remap_buffers();
                self.update_encoder();
            }
        );
        Ok(())
    }

    /// ### Parameters
    /// `array`
    ///
    /// > Specifies a value that may be the name of a vertex array object.
    ///
    /// ### Description
    /// [**glIsVertexArray**](crate::context::Context::oxidegl_is_vertex_array)
    /// returns [`GL_TRUE`](crate::enums::GL_TRUE) if `array` is currently the
    /// name of a vertex array object. If `array` is zero, or if `array` is not
    /// the name of a vertex array object, or if an error occurs, [**glIsVertexArray**](crate::context::Context::oxidegl_is_vertex_array)
    /// returns [`GL_FALSE`](crate::enums::GL_FALSE). If `array` is a name returned
    /// by [**glGenVertexArrays**](crate::context::Context::oxidegl_gen_vertex_arrays),
    /// by that has not yet been bound through a call to [**glBindVertexArray**](crate::context::Context::oxidegl_bind_vertex_array),
    /// then the name is not a vertex array object and [**glIsVertexArray**](crate::context::Context::oxidegl_is_vertex_array)
    /// returns [`GL_FALSE`](crate::enums::GL_FALSE).
    pub(crate) fn oxidegl_is_vertex_array(&mut self, array: GLuint) -> GLboolean {
        self.gl_state.vao_list.is_obj(array)
    }

    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of vertex array objects to be deleted.
    ///
    /// `arrays`
    ///
    /// > Specifies the address of an array containing the `n` names of the objects
    /// > to be deleted.
    ///
    /// ### Description
    /// [**glDeleteVertexArrays**](crate::context::Context::oxidegl_delete_vertex_arrays)
    /// deletes `n` vertex array objects whose names are stored in the array addressed
    /// by `arrays`. Once a vertex array object is deleted it has no contents and
    /// its name is again unused. If a vertex array object that is currently bound
    /// is deleted, the binding for that object reverts to zero and the default
    /// vertex array becomes current. Unused names in `arrays` are silently ignored,
    /// as is the value zero.
    pub(crate) unsafe fn oxidegl_delete_vertex_arrays(
        &mut self,
        n: GLsizei,
        arrays: *const GLuint,
    ) {
        // Safety: Caller ensures invariants are upheld
        unsafe {
            self.gl_state.vao_list.delete_objects(n, arrays);
        }
    }
}

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object for [**glDisableVertexArrayAttrib**](crate::context::Context::oxidegl_disable_vertex_array_attrib)
/// > and [**glEnableVertexArrayAttrib**](crate::context::Context::oxidegl_enable_vertex_array_attrib)
/// > functions.
///
/// `index`
///
/// > Specifies the index of the generic vertex attribute to be enabled or disabled.
///
/// ### Description
/// [**glEnableVertexAttribArray**](crate::context::Context::oxidegl_enable_vertex_attrib_array)
/// and [**glEnableVertexArrayAttrib**](crate::context::Context::oxidegl_enable_vertex_array_attrib)
/// enable the generic vertex attribute array specified by `index`. [**glEnableVertexAttribArray**](crate::context::Context::oxidegl_enable_vertex_attrib_array)
/// uses currently bound vertex array object for the operation, whereas [**glEnableVertexArrayAttrib**](crate::context::Context::oxidegl_enable_vertex_array_attrib)
/// updates state of the vertex array object with ID `vaobj`.
///
/// [**glDisableVertexAttribArray**](crate::context::Context::oxidegl_disable_vertex_attrib_array)
/// and [**glDisableVertexArrayAttrib**](crate::context::Context::oxidegl_disable_vertex_array_attrib)
/// disable the generic vertex attribute array specified by `index`. [**glDisableVertexAttribArray**](crate::context::Context::oxidegl_disable_vertex_attrib_array)
/// uses currently bound vertex array object for the operation, whereas [**glDisableVertexArrayAttrib**](crate::context::Context::oxidegl_disable_vertex_array_attrib)
/// updates state of the vertex array object with ID `vaobj`.
///
/// By default, all client-side capabilities are disabled, including all generic
/// vertex attribute arrays. If enabled, the values in the generic vertex attribute
/// array will be accessed and used for rendering when calls are made to vertex
/// array commands such as [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays),
/// [**glDrawElements**](crate::context::Context::oxidegl_draw_elements), [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements),
/// [**glMultiDrawElements**](crate::context::Context::oxidegl_multi_draw_elements),
/// or [**glMultiDrawArrays**](crate::context::Context::oxidegl_multi_draw_arrays).
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_ENABLED`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_ENABLED)
///
/// [**glGetVertexAttribPointerv**](crate::context::Context::oxidegl_get_vertex_attrib_pointerv)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_POINTER`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_POINTER)
impl Context {
    pub(crate) fn oxidegl_disable_vertex_attrib_array(&mut self, index: GLuint) -> GlFallible {
        self.disable_vertex_array_attrib(CurrentBinding, index)
    }
    pub(crate) fn oxidegl_enable_vertex_attrib_array(&mut self, index: GLuint) -> GlFallible {
        self.enable_vertex_array_attrib(CurrentBinding, index)
    }
    pub(crate) fn oxidegl_disable_vertex_array_attrib(
        &mut self,
        vaobj: GLuint,
        index: GLuint,
    ) -> GlFallible {
        self.disable_vertex_array_attrib(vaobj, index)
    }
    pub(crate) fn oxidegl_enable_vertex_array_attrib(
        &mut self,
        vaobj: GLuint,
        index: GLuint,
    ) -> GlFallible {
        self.enable_vertex_array_attrib(vaobj, index)
    }
}

impl Context {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn enable_vertex_array_attrib(
        &mut self,
        vao: impl MaybeObjectName<Vao>,
        index: GLuint,
    ) -> GlFallible {
        let vao = self.get_vao(vao)?;
        gl_debug!(
            "enabling vertex attribute at index {index} of {:?}",
            vao.name
        );
        vao.get_attrib_mut(index)?.enabled = true;
        Ok(())
    }
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn disable_vertex_array_attrib(
        &mut self,
        vao: impl MaybeObjectName<Vao>,
        index: GLuint,
    ) -> GlFallible {
        let vao = self.get_vao(vao)?;
        gl_debug!(
            "disabling vertex attribute at index {index} of {:?}",
            vao.name
        );
        vao.get_attrib_mut(index)?.enabled = false;
        Ok(())
    }
}

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object for [**glVertexArrayAttribBinding**](crate::context::Context::oxidegl_vertex_array_attrib_binding).
///
/// `attribindex`
///
/// > The index of the attribute to associate with a vertex buffer binding.
///
/// `bindingindex`
///
/// > The index of the vertex buffer binding with which to associate the generic
/// > vertex attribute.
///
/// ### Description
/// [**glVertexAttribBinding**](crate::context::Context::oxidegl_vertex_attrib_binding)
/// and [**glVertexArrayAttribBinding**](crate::context::Context::oxidegl_vertex_array_attrib_binding)
/// establishes an association between the generic vertex attribute of a vertex
/// array object whose index is given by `attribindex`, and a vertex buffer
/// binding whose index is given by `bindingindex`. For [**glVertexAttribBinding**](crate::context::Context::oxidegl_vertex_attrib_binding),
/// the vertex array object affected is that currently bound. For [**glVertexArrayAttribBinding**](crate::context::Context::oxidegl_vertex_array_attrib_binding),
/// `vaobj` is the name of the vertex array object.
///
/// `attribindex` must be less than the value of [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS)
/// and `bindingindex` must be less than the value of [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS).
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with arguments [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS),
/// [`GL_VERTEX_BINDING_DIVISOR`](crate::enums::GL_VERTEX_BINDING_DIVISOR).
impl Context {
    pub(crate) fn oxidegl_vertex_attrib_binding(
        &mut self,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) -> GlFallible {
        self.vertex_attrib_binding_internal(CurrentBinding, attribindex, bindingindex)
    }
    pub(crate) fn oxidegl_vertex_array_attrib_binding(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) -> GlFallible {
        self.vertex_attrib_binding_internal(vaobj, attribindex, bindingindex)
    }
}
impl Context {
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn vertex_attrib_binding_internal(
        &mut self,
        vao: impl MaybeObjectName<Vao>,
        attrib_index: u32,
        binding_index: u32,
    ) -> GlFallible {
        gl_assert!(
            (binding_index as usize) < MAX_VERTEX_ATTRIB_BUFFER_BINDINGS,
            InvalidValue,
            "out of bounds attribute buffer binding index"
        );
        let vao = self.get_vao(vao)?;
        gl_debug!(
            "binding vertex attribute index {attrib_index} to {:?} buffer binding {binding_index}",
            vao.name
        );
        vao.get_attrib_mut(attrib_index)?.buffer_idx = binding_index as u8;
        Ok(())
    }
}

/// ### Parameters
/// `vaobj`
///
/// > Specifies the name of the vertex array object for [**glVertexArrayBindingDivisor**](crate::context::Context::oxidegl_vertex_array_binding_divisor)
/// > function.
///
/// `bindingindex`
///
/// > The index of the binding whose divisor to modify.
///
/// `divisor`
///
/// > The new value for the instance step rate to apply.
///
/// ### Description
/// [**glVertexBindingDivisor**](crate::context::Context::oxidegl_vertex_binding_divisor)
/// and [**glVertexArrayBindingDivisor**](crate::context::Context::oxidegl_vertex_array_binding_divisor)
/// modify the rate at which generic vertex attributes advance when rendering
/// multiple instances of primitives in a single draw command. If `divisor`
/// is zero, the attributes using the buffer bound to `bindingindex` advance
/// once per vertex. If `divisor` is non-zero, the attributes advance once
/// per `divisor` instances of the set(s) of vertices being rendered. An attribute
/// is referred to as *instanced* if the corresponding `divisor` value is non-zero.
///
/// [**glVertexBindingDivisor**](crate::context::Context::oxidegl_vertex_binding_divisor)
/// uses currently bound vertex array object, whereas [**glVertexArrayBindingDivisor**](crate::context::Context::oxidegl_vertex_array_binding_divisor)
/// updates state of the vertex array object with ID `vaobj`.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with arguments [`GL_MAX_VERTEX_ATTRIB_BINDINGS`](crate::enums::GL_MAX_VERTEX_ATTRIB_BINDINGS),
/// [`GL_VERTEX_BINDING_DIVISOR`](crate::enums::GL_VERTEX_BINDING_DIVISOR).
impl Context {
    pub(crate) fn oxidegl_vertex_binding_divisor(
        &mut self,
        bindingindex: GLuint,
        divisor: GLuint,
    ) -> GlFallible {
        self.vertex_binding_divisor_internal(CurrentBinding, bindingindex, divisor)
    }
    pub(crate) fn oxidegl_vertex_array_binding_divisor(
        &mut self,
        vaobj: GLuint,
        bindingindex: GLuint,
        divisor: GLuint,
    ) -> GlFallible {
        self.vertex_binding_divisor_internal(vaobj, bindingindex, divisor)
    }
}

impl Context {
    #[inline]
    pub(crate) fn vertex_binding_divisor_internal(
        &mut self,
        vao: impl MaybeObjectName<Vao>,
        binding_index: u32,
        divisor: u32,
    ) -> GlFallible {
        let vao = self.get_vao(vao)?;
        debug!("setting vertex attribute binding divisor at buffer binding index {binding_index} to {divisor}");
        vao.get_binding_mut(binding_index)?.divisor = NonZeroU32::new(divisor);
        Ok(())
    }
}

/// ### Parameters
/// `index`
///
/// > Specifies the index of the generic vertex attribute to be modified.
///
/// `size`
///
/// > Specifies the number of components per generic vertex attribute. Must be
/// > 1, 2, 3, 4. Additionally, the symbolic constant [`GL_BGRA`](crate::enums::GL_BGRA)
/// > is accepted by [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer).
/// > The initial value is 4.
///
/// `type`
///
/// > Specifies the data type of each component in the array. The symbolic constants
/// > [`GL_BYTE`](crate::enums::GL_BYTE), [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
/// > [`GL_SHORT`](crate::enums::GL_SHORT), [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT),
/// > [`GL_INT`](crate::enums::GL_INT), and [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT)
/// > are accepted by [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer)
/// > and [**glVertexAttribIPointer**](crate::context::Context::oxidegl_vertex_attrib_i_pointer).
/// > Additionally [`GL_HALF_FLOAT`](crate::enums::GL_HALF_FLOAT), [`GL_FLOAT`](crate::enums::GL_FLOAT),
/// > [`GL_DOUBLE`](crate::enums::GL_DOUBLE), [`GL_FIXED`](crate::enums::GL_FIXED),
/// > [`GL_INT_2_10_10_10_REV`](crate::enums::GL_INT_2_10_10_10_REV), [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::enums::GL_UNSIGNED_INT_2_10_10_10_REV)
/// > and [`GL_UNSIGNED_INT_10F_11F_11F_REV`](crate::enums::GL_UNSIGNED_INT_10F_11F_11F_REV)
/// > are accepted by [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer).
/// > [`GL_DOUBLE`](crate::enums::GL_DOUBLE) is also accepted by [**glVertexAttribLPointer**](crate::context::Context::oxidegl_vertex_attrib_l_pointer)
/// > and is the only token accepted by the `type` parameter for that function.
/// > The initial value is [`GL_FLOAT`](crate::enums::GL_FLOAT).
///
/// `normalized`
///
/// > For [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer),
/// > specifies whether fixed-point data values should be normalized( [`GL_TRUE`](crate::enums::GL_TRUE))
/// > or converted directly as fixed-point values( [`GL_FALSE`](crate::enums::GL_FALSE))
/// > when they are accessed.
///
/// `stride`
///
/// > Specifies the byte offset between consecutive generic vertex attributes.
/// > If `stride` is 0, the generic vertex attributes are understood to be tightly
/// > packed in the array. The initial value is 0.
///
/// `pointer`
///
/// > Specifies a offset of the first component of the first generic vertex attribute
/// > in the array in the data store of the buffer currently bound to the [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER)
/// > target. The initial value is 0.
///
/// ### Description
/// [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer),
/// [**glVertexAttribIPointer**](crate::context::Context::oxidegl_vertex_attrib_i_pointer)
/// and [**glVertexAttribLPointer**](crate::context::Context::oxidegl_vertex_attrib_l_pointer)
/// specify the location and data format of the array of generic vertex attributes
/// at index `index` to use when rendering. `size` specifies the number of
/// components per attribute and must be 1, 2, 3, 4, or [`GL_BGRA`](crate::enums::GL_BGRA).
/// `type` specifies the data type of each component, and `stride` specifies
/// the byte stride from one attribute to the next, allowing vertices and attributes
/// to be packed into a single array or stored in separate arrays.
///
/// For [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer),
/// if `normalized` is set to [`GL_TRUE`](crate::enums::GL_TRUE), it indicates
/// that values stored in an integer format are to be mapped to the range \[-1,1\]
/// (for signed values) or \[0,1\] (for unsigned values) when they are accessed
/// and converted to floating point. Otherwise, values will be converted to
/// floats directly without normalization.
///
/// For [**glVertexAttribIPointer**](crate::context::Context::oxidegl_vertex_attrib_i_pointer),
/// only the integer types [`GL_BYTE`](crate::enums::GL_BYTE), [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
/// [`GL_SHORT`](crate::enums::GL_SHORT), [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT),
/// [`GL_INT`](crate::enums::GL_INT), [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT)
/// are accepted. Values are always left as integer values.
///
/// [**glVertexAttribLPointer**](crate::context::Context::oxidegl_vertex_attrib_l_pointer)
/// specifies state for a generic vertex attribute array associated with a
/// shader attribute variable declared with 64-bit double precision components.
/// `type` must be [`GL_DOUBLE`](crate::enums::GL_DOUBLE). `index`, `size`,
/// and `stride` behave as described for [**glVertexAttribPointer**](crate::context::Context::oxidegl_vertex_attrib_pointer)
/// and [**glVertexAttribIPointer**](crate::context::Context::oxidegl_vertex_attrib_i_pointer).
///
/// If `pointer` is not [`GL_ARRAY_BUFFER`](crate::enums::GL_ARRAY_BUFFER)
/// target (see [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer)
/// ), otherwise an error is generated. `pointer` is treated as a byte offset
/// into the buffer object's data store. The buffer object binding( [`GL_ARRAY_BUFFER_BINDING`](crate::enums::GL_ARRAY_BUFFER_BINDING))
/// is saved as generic vertex attribute array state( [`GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING))
/// for index `index`.
///
/// When a generic vertex attribute array is specified, `size`, `type`, `normalized`,
/// `stride`, and `pointer` are saved as vertex array state, in addition to
/// the current vertex array buffer object binding.
///
/// To enable and disable a generic vertex attribute array, call [**glEnableVertexAttribArray**](crate::context::Context::oxidegl_enable_vertex_attrib_array)
/// and [**glDisableVertexAttribArray**](crate::context::Context::oxidegl_disable_vertex_attrib_array)
/// with `index`. If enabled, the generic vertex attribute array is used when
/// [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays), [**glMultiDrawArrays**](crate::context::Context::oxidegl_multi_draw_arrays),
/// [**glDrawElements**](crate::context::Context::oxidegl_draw_elements), [**glMultiDrawElements**](crate::context::Context::oxidegl_multi_draw_elements),
/// or [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements)
/// is called.
///
/// ### Notes
/// Each generic vertex attribute array is initially disabled and isn't accessed
/// when [**glDrawElements**](crate::context::Context::oxidegl_draw_elements),
/// [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements),
/// [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays), [**glMultiDrawArrays**](crate::context::Context::oxidegl_multi_draw_arrays),
/// or [**glMultiDrawElements**](crate::context::Context::oxidegl_multi_draw_elements)
/// is called.
///
/// [`GL_UNSIGNED_INT_10F_11F_11F_REV`](crate::enums::GL_UNSIGNED_INT_10F_11F_11F_REV)
/// is accepted for `type` only if the GL version is 4.4 or higher.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_VERTEX_ATTRIBS`](crate::enums::GL_MAX_VERTEX_ATTRIBS)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_ENABLED`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_ENABLED)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_SIZE`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_SIZE)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_TYPE`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_TYPE)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_NORMALIZED`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_STRIDE`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_STRIDE)
///
/// [**glGetVertexAttrib**](crate::context::Context::oxidegl_get_vertex_attrib)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_ARRAY_BUFFER_BINDING`](crate::enums::GL_ARRAY_BUFFER_BINDING)
///
/// [**glGetVertexAttribPointerv**](crate::context::Context::oxidegl_get_vertex_attrib_pointerv)
/// with arguments `index` and [`GL_VERTEX_ATTRIB_ARRAY_POINTER`](crate::enums::GL_VERTEX_ATTRIB_ARRAY_POINTER)
///
///
// thank god there's no DSA version of this shite
impl Context {
    #[allow(clippy::cast_sign_loss)]
    pub(crate) unsafe fn oxidegl_vertex_attrib_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: VertexAttribPointerType,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) -> GlFallible {
        self.vertex_attrib_pointer_internal(
            index,
            size as u32,
            // Safety: VertexAttribType has the same valid values as VertexAttribPointerType
            unsafe { core::mem::transmute::<VertexAttribPointerType, VertexAttribType>(r#type) },
            stride,
            pointer,
            if normalized {
                IntegralCastBehavior::Normalize
            } else {
                {
                    IntegralCastBehavior::Cast
                }
            },
        )
    }
    #[allow(clippy::cast_sign_loss)]
    pub(crate) unsafe fn oxidegl_vertex_attrib_i_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: VertexAttribIType,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) -> GlFallible {
        self.vertex_attrib_pointer_internal(
            index,
            size as u32,
            // Safety: VertexAttribType has a strict superset of the valid values of VertexAttribIType
            unsafe { core::mem::transmute::<VertexAttribIType, VertexAttribType>(r#type) },
            stride,
            pointer,
            IntegralCastBehavior::Native,
        )
    }
    pub(crate) unsafe fn oxidegl_vertex_attrib_l_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        panic!("OxideGL does not support vertex attributes of Double type");
    }
}

impl Context {
    #[inline]
    pub(crate) fn vertex_attrib_pointer_internal(
        &mut self,
        index: GLuint,
        size: GLuint,
        ty: VertexAttribType,
        stride: GLsizei,
        pointer: *const GLvoid,
        integer_behavior: IntegralCastBehavior,
    ) -> GlFallible {
        //FIXME highly suboptimal implementation, the logic of all of the commands should be moved onto methods on `Vao`
        sizei!(stride);
        let vao = self.get_vao(CurrentBinding)?;
        let saved_binding = *vao.get_binding_mut(index)?;
        let saved_attr = *vao.get_attrib_mut(index)?;
        let e = {
            'bail: {
                if let Err(e) = self.oxidegl_vertex_attrib_format_internal(
                    index,
                    size,
                    ty,
                    0,
                    integer_behavior,
                    CurrentBinding,
                ) {
                    break 'bail e;
                }
                if let Err(e) = self.vertex_attrib_binding_internal(CurrentBinding, index, index) {
                    break 'bail e;
                }
                if let Err(e) = {
                    let stride = if stride > 0 {
                        stride
                    } else {
                        let vao = self.get_vao(CurrentBinding)?;
                        let attrib = vao.get_attrib_mut(index)?;
                        u32::from(attrib.compute_stride())
                    };
                    let buf = self
                        .gl_state
                        .buffer_bindings
                        .array
                        .ok_or(GlError::InvalidOperation)?
                        .to_raw();
                    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
                    self.oxidegl_bind_vertex_buffer(
                        index,
                        buf,
                        pointer as usize as isize,
                        stride as i32,
                    )
                } {
                    break 'bail e;
                }
                // return if we didn't hit an error
                return Ok(());
            }
        };
        let vao = self.get_vao(CurrentBinding)?;
        // Need to revert any side effects if we error halfway through
        *vao.get_binding_mut(index)? = saved_binding;
        *vao.get_attrib_mut(index)? = saved_attr;
        Err(e)
    }
}
pub const MAX_VERTEX_ATTRIBUTES: usize = 32;
pub const MAX_VERTEX_ATTRIB_BUFFER_BINDINGS: usize = 16;
pub const MAX_VERTEX_ATTRIBUTE_STRIDE: u16 = 2048;

#[derive(Debug)]
pub struct Vao {
    pub(crate) name: ObjectName<Self>,
    pub(crate) attribs: [VertexAttrib; MAX_VERTEX_ATTRIBUTES],
    pub(crate) buffer_bindings: [AttributeBufferBinding; MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
}
impl Vao {
    fn new_default(name: ObjectName<Self>) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            name,
            attribs: array::from_fn(|i| VertexAttrib::new_default(i as u8)),
            buffer_bindings: [AttributeBufferBinding::new_default();
                MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
        }
    }
    fn get_binding_mut(&mut self, idx: u32) -> GlFallible<&mut AttributeBufferBinding> {
        self.buffer_bindings
            .get_mut(idx as usize)
            .ok_or(GlError::InvalidValue.e())
    }
    fn get_attrib_mut(&mut self, idx: u32) -> GlFallible<&mut VertexAttrib> {
        self.attribs
            .get_mut(idx as usize)
            .ok_or(GlError::InvalidValue.e())
    }
}
impl NamedObject for Vao {
    type LateInitType = LateInit<Self>;
    const LATE_INIT_FUNC: fn(ObjectName<Self>) -> Self = Self::new_default;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VertexAttrib {
    pub(crate) components: u8,
    pub(crate) buffer_idx: u8,
    pub(crate) relative_offset: u16,
    pub(crate) integral_cast: IntegralCastBehavior,
    pub(crate) component_type: VertexAttribType,
    pub(crate) enabled: bool,
}

impl VertexAttrib {
    #[inline]
    pub(crate) fn validate(&self) -> GlFallible {
        let components = self.components;
        let component_type = self.component_type;
        let normalize = self.integral_cast == IntegralCastBehavior::Normalize;
        gl_assert!(
            !((components == 0 || components > 4)
                && !matches!(
                    component_type,
                    VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
                )),
            InvalidValue,
            "UB: attribute size BGRA is not supported for non-BGR10A2 formats!"
        );
        gl_assert!(
            !(matches!(
                component_type,
                VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
            ) && components < 4
                || components > 4),
            InvalidValue,
            "UB: attribute size {components} is not supported with BGR10A2 formats!"
        );
        gl_assert!(
            !(component_type == VertexAttribType::UnsignedInt10F11F11FRev && components != 3),
            InvalidValue,
            "UB: attribute size for attribute with format RG11FB10F must be 3, got {components}"
        );
        gl_assert!(
            !((components == 0 || components > 4) && !normalize),
            InvalidValue,
            "UB: size is GL_BGRA but normalize is false"
        );
        gl_assert!(
            self.relative_offset < MAX_VERTEX_ATTRIBUTE_STRIDE,
            InvalidValue,
            "UB: relative offset greater than maximum stride"
        );
        assert!(!normalize);
        Ok(())
    }
    pub(crate) const fn new_default(idx: u8) -> Self {
        Self {
            components: 4,
            buffer_idx: idx,
            relative_offset: 0,
            integral_cast: IntegralCastBehavior::Native,
            component_type: VertexAttribType::Float,
            enabled: false,
        }
    }
    #[inline]
    pub(crate) fn component_byte_size(&self) -> usize {
        match self.component_type {
            VertexAttribType::Byte | VertexAttribType::UnsignedByte => 1,
            VertexAttribType::HalfFloat
            | VertexAttribType::Fixed
            | VertexAttribType::Short
            | VertexAttribType::UnsignedShort => 2,
            VertexAttribType::Float | VertexAttribType::Int | VertexAttribType::UnsignedInt => 4,
            VertexAttribType::Double => 8,
            // Return a placeholder for 32bit packed values
            VertexAttribType::UnsignedInt2101010Rev
            | VertexAttribType::Int2101010Rev
            | VertexAttribType::UnsignedInt10F11F11FRev => 0,
        }
    }
    // FIXME: directly enforce MSL layout rules here
    // See: Metal Shading Language Specification, tables 2.1, 2.2, 2.5
    #[inline]
    pub(crate) fn compute_stride(&self) -> u16 {
        let component_size = self.component_byte_size();
        // Packed values always have layout of uint
        if component_size == 0 {
            return 4;
        }
        let contiguous_size = component_size * self.components as usize;
        let align = contiguous_size.next_power_of_two().max(16);

        // Max allowable stride is 2048 (<u16::MAX)
        #[allow(clippy::cast_possible_truncation)]
        let ret = (contiguous_size.div_ceil(align) * align) as u16;
        ret
    }
    #[inline]
    pub(crate) fn get_mtl_layout(&self) -> AttributeFormatWithConversion {
        gl_attribute_to_mtl(self.component_type, self.components, self.integral_cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttributeBufferBinding {
    pub(crate) buf: Option<ObjectName<Buffer>>,
    pub(crate) offset: usize,
    pub(crate) stride: u16,
    pub(crate) divisor: Option<NonZeroU32>,
}
impl AttributeBufferBinding {
    pub(crate) const fn new_default() -> Self {
        Self {
            buf: None,
            offset: 0,
            stride: 16,
            divisor: None,
        }
    }
}

macro_rules! generate_attr_match_branch {
    ($base:ident, $normalize:expr, $size:expr, $bitlen:expr, (norm: $nconvtype:expr, unnorm: $unconvtype:expr)) => {
        if $normalize {
            generate_attr_match_branch!(;$base, $size, $bitlen, $nconvtype, Normalized)
        } else {
            generate_attr_match_branch!(;$base, $size, $bitlen, $unconvtype, )
        }
    };
    ($base:ident!, $normalize:expr, $size:expr, $bitlen:expr, (norm: $nconvtype:expr, unnorm: $unconvtype:expr)) => {
        if $normalize {
            generate_attr_match_branch!(;$base, $size, $bitlen, $nconvtype, )
        } else {
            generate_attr_match_branch!(;$base, $size, $bitlen, $unconvtype, )
        }
    };
    (;$base:ident, $size:expr, $bitlen:expr, $convtype:expr, $($suffix:ident),* ) => {
        AttributeFormatWithConversion {
            mtl_format: {
                    match $size {
                    1 => concat_idents::concat_idents!(name = $base, $($suffix)* { MTLVertexFormat::name }),
                    2 => concat_idents::concat_idents!(name = $base, 2, $($suffix)* { MTLVertexFormat::name }),
                    3 => concat_idents::concat_idents!(name = $base, 3, $($suffix)* { MTLVertexFormat::name }),
                    4 => concat_idents::concat_idents!(name = $base, 4, $($suffix)* { MTLVertexFormat::name }),
                    #[allow(unused_unsafe)]
                    // Safety: Caller ensures the number of components is in bounds
                    _ => unreachable!("UB: invalid vertex attribute size!")
                }.0 as u32
            },
            normalization_const: $bitlen,
            conversion: $convtype,
            bgra_shuffle: false,
        }

    };
}

#[inline]
fn gl_attribute_to_mtl(
    ty: VertexAttribType,
    num_components: u8,
    behavior: IntegralCastBehavior,
) -> AttributeFormatWithConversion {
    let normalize = behavior == IntegralCastBehavior::Normalize;
    let bgra_shuffle = !(1..=4).contains(&num_components);
    // All MTLVertexFormat values are in-bounds for u32
    #[allow(clippy::cast_possible_truncation)]
    match ty {
        VertexAttribType::Byte => generate_attr_match_branch!(
            Char,
            normalize,
            num_components,
            7,
            (norm: IntegralCastBehavior::Native, unnorm: behavior)
        ),
        VertexAttribType::UnsignedByte => generate_attr_match_branch!(
            UChar,
            normalize,
            num_components,
            8,
            (norm: IntegralCastBehavior::Native, unnorm: behavior)
        ),

        VertexAttribType::Short => generate_attr_match_branch!(
            Short,
            normalize,
            num_components,
            15,
            (norm: IntegralCastBehavior::Native, unnorm: behavior)
        ),
        VertexAttribType::UnsignedShort => generate_attr_match_branch!(
            UShort,
            normalize,
            num_components,
            16,
            (norm: IntegralCastBehavior::Native, unnorm: behavior)
        ),

        VertexAttribType::Int => generate_attr_match_branch!(
            Int!,
            normalize,
            num_components,
            31,
            (norm: IntegralCastBehavior::Normalize, unnorm: behavior)
        ),
        VertexAttribType::UnsignedInt => generate_attr_match_branch!(
            UInt!,
            normalize,
            num_components,
            32,
            (norm: IntegralCastBehavior::Normalize, unnorm: behavior)
        ),

        VertexAttribType::HalfFloat => generate_attr_match_branch!(;
            Half,
            num_components,
            0,
            IntegralCastBehavior::Native,
        ),
        VertexAttribType::Float => generate_attr_match_branch!(;
            Float,
            num_components,
            0,
            IntegralCastBehavior::Native,
        ),
        VertexAttribType::UnsignedInt2101010Rev => AttributeFormatWithConversion {
            mtl_format: MTLVertexFormat::UInt1010102Normalized.0 as u32,
            normalization_const: 0,
            conversion: IntegralCastBehavior::Native,
            bgra_shuffle,
        },
        VertexAttribType::Int2101010Rev => AttributeFormatWithConversion {
            mtl_format: MTLVertexFormat::Int1010102Normalized.0 as u32,
            normalization_const: 0,
            conversion: IntegralCastBehavior::Native,
            bgra_shuffle,
        },

        VertexAttribType::UnsignedInt10F11F11FRev => AttributeFormatWithConversion {
            mtl_format: MTLVertexFormat::FloatRG11B10.0 as u32,
            normalization_const: 0,
            conversion: IntegralCastBehavior::Native,
            bgra_shuffle: false,
        },

        // Unsupported types
        VertexAttribType::Double => {
            panic!("OxideGL does not support vertex attributes of Double type");
        }
        VertexAttribType::Fixed => panic!("OxideGL does not support fixed-point parameter types"),
    }
}
/// Describes conversion code that must be run on this vertex attribute before usage in the vertex shader
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegralCastBehavior {
    /// This vertex attribute's in-memory representation is integral but GL client requests normalization to float.
    Normalize,
    /// This vertex attribute's in-memory representation is integral but GL client requests casting to float
    Cast,
    /// This vertex attribute's in-memory representation is equivalent to the representation requested by the GL client
    Native,
}
#[derive(Debug, Clone, Copy)]
pub struct AttributeFormatWithConversion {
    /// [`MTLVertexFormat`] truncated to 32 bits
    pub(crate) mtl_format: u32,
    /// Values should be divided or multiplied by `2^normalization_const` when normalizing
    pub(crate) normalization_const: u8,
    /// Type of conversion code that must be added to the vertex shader
    pub(crate) conversion: IntegralCastBehavior,
    pub(crate) bgra_shuffle: bool,
}
impl AttributeFormatWithConversion {
    pub(crate) fn to_vertex_format(self) -> MTLVertexFormat {
        MTLVertexFormat(self.mtl_format as usize)
    }
    pub(crate) fn validate(self) {
        assert_eq!(self.conversion, IntegralCastBehavior::Native, "OxideGL does not yet support normalize or cast-to-float semantics for integer-typed vertex attributes");
    }
}
