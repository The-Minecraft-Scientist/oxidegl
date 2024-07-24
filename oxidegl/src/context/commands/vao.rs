use std::num::NonZeroU32;

use log::debug;
use objc2_metal::MTLAttributeFormat;

use crate::{
    context::{
        commands::buffer::Buffer,
        state::{NamedObject, ObjectName},
        Context,
    },
    dispatch::gl_types::{GLboolean, GLenum, GLint, GLsizei, GLuint},
    enums::{VertexAttribIType, VertexAttribType},
};

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
    pub fn oxidegl_vertex_attrib_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribType,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_format not yet implemented");
    }
    pub fn oxidegl_vertex_attrib_i_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribIType,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_i_format not yet implemented");
    }
    pub fn oxidegl_vertex_array_attrib_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribType,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_format not yet implemented");
    }
    pub fn oxidegl_vertex_array_attrib_i_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: VertexAttribIType,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_i_format not yet implemented");
    }
    pub fn oxidegl_vertex_array_attrib_l_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("OxideGL does not support vertex attributes of double type");
    }
    pub fn oxidegl_vertex_attrib_l_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("OxideGL does not support vertex attributes of double type");
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
    pub unsafe fn oxidegl_gen_vertex_arrays(&mut self, n: GLsizei, arrays: *mut GLuint) {
        panic!("command oxidegl_gen_vertex_arrays not yet implemented");
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
    pub unsafe fn oxidegl_create_vertex_arrays(&mut self, n: GLsizei, mut arrays: *mut GLuint) {
        panic!("command oxidegl_create_vertex_arrays not yet implemented");
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
    pub fn oxidegl_bind_vertex_array(&mut self, array: GLuint) {
        panic!("command oxidegl_bind_vertex_array not yet implemented");
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
    pub fn oxidegl_is_vertex_array(&mut self, array: GLuint) -> GLboolean {
        panic!("command oxidegl_is_vertex_array not yet implemented");
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
    pub unsafe fn oxidegl_delete_vertex_arrays(&mut self, n: GLsizei, arrays: *const GLuint) {
        panic!("command oxidegl_delete_vertex_arrays not yet implemented");
    }
}

pub const MAX_VERTEX_ATTRIBUTES: usize = 32;
pub const MAX_VERTEX_ATTRIB_BUFFER_BINDINGS: usize = 16;

#[derive(Debug)]
pub struct Vao {
    name: ObjectName<Self>,
    attribs: [Option<GLVertexAttrib>; MAX_VERTEX_ATTRIBUTES],
    buffer_bindings: [Option<AttributeBufferBinding>; MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
}
impl Vao {
    fn new_default(name: ObjectName<Self>) -> Self {
        Self {
            name,
            attribs: [None; MAX_VERTEX_ATTRIBUTES],
            buffer_bindings: [None; MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
        }
    }
}
impl NamedObject for Vao {}

#[derive(Debug, Clone, Copy)]
pub struct GLVertexAttrib {
    components: u8,
    buffer_idx: u8,
    relative_offset: u16,
    normalize: bool,
    component_type: VertexAttribType,
    divisor: Option<NonZeroU32>,
}

impl GLVertexAttrib {
    pub fn new(
        components: u8,
        buffer_idx: u8,
        relative_offset: u16,
        normalize: bool,
        component_type: VertexAttribType,
        divisor: Option<NonZeroU32>,
    ) -> Self {
        debug_assert!(
            !((components == 0 || components > 4)
                && !matches!(
                    component_type,
                    VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
                )),
            "UB: attribute size BGRA is not supported for non-BGR10A2 formats!"
        );
        debug_assert!(
            !(matches!(
                component_type,
                VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
            ) && components < 4
                || components > 4),
            "UB: attribute size {components} is not supported with BGR10A2 formats!"
        );
        debug_assert!(
            !(component_type == VertexAttribType::UnsignedInt10F11F11FRev && components != 3),
            "UB: attribute size for attribute with format RG11B10 must be 3, got {components}"
        );
        debug_assert!(
            !((components == 0 || components > 4) && !normalize),
            "UB: size is GL_BGRA but normalize is false"
        );
        Self {
            components,
            buffer_idx,
            relative_offset,
            normalize,
            component_type,
            divisor,
        }
    }
    fn set_divisor(&mut self, divisor: u32) {}
    #[inline]
    pub fn component_byte_size(&self) -> usize {
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
    pub fn stride(&self) -> usize {
        let component_size = self.component_byte_size();
        // Packed values always have layout of uint
        if component_size == 0 {
            return 4;
        }
        let contiguous_size = component_size * self.components as usize;
        let align = contiguous_size.next_power_of_two().max(16);
        contiguous_size.div_ceil(align) * align
    }
    #[inline]
    pub fn get_mtl_layout(&self) -> AttributeFormatWithConversion {
        unsafe { gl_attribute_to_mtl(self.component_type, self.components, self.normalize) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttributeBufferBinding {
    buf: ObjectName<Buffer>,
    offset: u64,
    stride: u16,
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
                    1 => concat_idents::concat_idents!(name = $base, $($suffix)* { MTLAttributeFormat::name }),
                    2 => concat_idents::concat_idents!(name = $base, 2, $($suffix)* { MTLAttributeFormat::name }),
                    3 => concat_idents::concat_idents!(name = $base, 3, $($suffix)* { MTLAttributeFormat::name }),
                    4 => concat_idents::concat_idents!(name = $base, 4, $($suffix)* { MTLAttributeFormat::name }),
                    #[allow(unused_unsafe)]
                    //Safety: Caller ensures the number of components is in bounds
                    _ => unsafe { crate::debug_unreachable!("UB: invalid vertex attribute size!") }
                }.0 as u32
            },
            normalization_const: $bitlen,
            conversion: $convtype,
        }

    };
}

#[inline]
unsafe fn gl_attribute_to_mtl(
    ty: VertexAttribType,
    num_components: u8,
    normalize: bool,
) -> AttributeFormatWithConversion {
    // All MTLAttributeFormat values are in-bounds for u32
    #[allow(clippy::cast_possible_truncation)]
    match ty {
        VertexAttribType::Byte => generate_attr_match_branch!(
            Char,
            normalize,
            num_components,
            7,
            (norm: VertexAttrPreProcess::Native, unnorm: VertexAttrPreProcess::Cast)
        ),
        VertexAttribType::UnsignedByte => generate_attr_match_branch!(
            UChar,
            normalize,
            num_components,
            8,
            (norm: VertexAttrPreProcess::Native, unnorm: VertexAttrPreProcess::Cast)
        ),

        VertexAttribType::Short => generate_attr_match_branch!(
            Short,
            normalize,
            num_components,
            15,
            (norm: VertexAttrPreProcess::Native, unnorm: VertexAttrPreProcess::Cast)
        ),
        VertexAttribType::UnsignedShort => generate_attr_match_branch!(
            UShort,
            normalize,
            num_components,
            16,
            (norm: VertexAttrPreProcess::Native, unnorm: VertexAttrPreProcess::Cast)
        ),

        VertexAttribType::Int => generate_attr_match_branch!(
            Int!,
            normalize,
            num_components,
            31,
            (norm: VertexAttrPreProcess::Normalize, unnorm: VertexAttrPreProcess::Cast)
        ),
        VertexAttribType::UnsignedInt => generate_attr_match_branch!(
            UInt!,
            normalize,
            num_components,
            32,
            (norm: VertexAttrPreProcess::Normalize, unnorm: VertexAttrPreProcess::Cast)
        ),

        VertexAttribType::HalfFloat => generate_attr_match_branch!(;
            Half,
            num_components,
            0,
            VertexAttrPreProcess::Native,
        ),
        VertexAttribType::Float => generate_attr_match_branch!(;
            Float,
            num_components,
            0,
            VertexAttrPreProcess::Native,
        ),
        VertexAttribType::UnsignedInt2101010Rev => AttributeFormatWithConversion {
            mtl_format: MTLAttributeFormat::UInt1010102Normalized.0 as u32,
            normalization_const: 0,
            conversion: VertexAttrPreProcess::Native,
        },
        VertexAttribType::Int2101010Rev => AttributeFormatWithConversion {
            mtl_format: MTLAttributeFormat::Int1010102Normalized.0 as u32,
            normalization_const: 0,
            conversion: VertexAttrPreProcess::Native,
        },

        VertexAttribType::UnsignedInt10F11F11FRev => AttributeFormatWithConversion {
            mtl_format: MTLAttributeFormat::FloatRG11B10.0 as u32,
            normalization_const: 0,
            conversion: VertexAttrPreProcess::Native,
        },

        // Unsupported types
        VertexAttribType::Double => {
            panic!("OxideGL does not support vertex attributes of Double type")
        }
        VertexAttribType::Fixed => panic!("OxideGL does not support fixed-point parameter types"),
    }
}
/// Describes conversion code that must be run on this vertex attribute before usage in the vertex shader
#[derive(Debug, Clone, Copy)]
pub enum VertexAttrPreProcess {
    /// This vertex attribute's in-memory representation is integral but GL client requests normalization to float.
    Normalize,
    /// This vertex attribute's in-memory representation is integral but GL client requests casting to float
    Cast,
    /// This vertex attribute's in-memory representation is equivalent to the representation requested by the GL client
    Native,
}
#[derive(Debug, Clone, Copy)]
pub struct AttributeFormatWithConversion {
    /// [`MTLAttributeFormat`] truncated to 32 bits
    pub mtl_format: u32,
    /// Values should be divided or multiplied by ``2^normalization_const`` when normalizing
    pub normalization_const: u8,
    /// Type of conversion code that must be added to the vertex shader
    pub conversion: VertexAttrPreProcess,
}
impl AttributeFormatWithConversion {
    fn get_mtl_format(self) -> MTLAttributeFormat {
        MTLAttributeFormat(self.mtl_format as usize)
    }
}
