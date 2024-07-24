use objc2_metal::MTLAttributeFormat;

use crate::{
    context::{
        commands::buffer::Buffer,
        state::{NamedObject, ObjectName},
        Context,
    },
    enums::VertexAttribType,
};

impl Context {}

pub const MAX_VERTEX_ATTRIBUTES: usize = 32;
pub const MAX_VERTEX_ATTRIB_BUFFER_BINDINGS: usize = 16;

#[derive(Debug)]
pub struct Vao {
    name: ObjectName<Self>,
    attribs: [Option<GLVertexAttrib>; MAX_VERTEX_ATTRIBUTES],
    buffer_bindings: [Option<AttributeBufferBinding>; MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
}

#[derive(Debug, Clone, Copy)]
pub struct GLVertexAttrib {
    components: u8,
    buffer_idx: u8,
    relative_offset: u16,
    normalize: bool,
    component_type: VertexAttribType,
    divisor: u32,
}
impl GLVertexAttrib {
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
    fn stride(&self) -> usize {
        let component_size = self.component_byte_size();
        // Packed values always have layout of uint
        if component_size == 0 {
            return 4;
        }
        let contiguous_size = component_size * self.components as usize;
        let align = contiguous_size.next_power_of_two().max(16);
        contiguous_size.div_ceil(align) * align
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
    debug_assert!(
        !((num_components == 0 || num_components > 4)
            && !matches!(
                ty,
                VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
            )),
        "UB: attribute size BGRA is not supported for non-BGR10A2 formats!"
    );
    debug_assert!(
        !(matches!(
            ty,
            VertexAttribType::UnsignedInt2101010Rev | VertexAttribType::Int2101010Rev
        ) && num_components < 4
            || num_components > 4),
        "UB: attribute size {num_components} is not supported with BGR10A2 formats!"
    );
    debug_assert!(
        !(ty == VertexAttribType::UnsignedInt10F11F11FRev && num_components != 3),
        "UB: attribute size for attribute with format RG11B10 must be 3, got {num_components}"
    );
    debug_assert!(
        !((num_components == 0 || num_components > 4) && !normalize),
        "UB: if size is GL_BGRA normalized must be true"
    );
    // All MTLAttributeFormat values are in-bounds for
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
struct AttributeFormatWithConversion {
    /// [`MTLAttributeFormat`] truncated to 32 bits
    pub mtl_format: u32,
    /// Values should be divided or multiplied by ``2^normalization_const`` when normalizing
    pub normalization_const: u8,
    /// Type of conversion code that must be added to the vertex shader
    pub conversion: VertexAttrPreProcess,
}

impl NamedObject for Vao {}
