use std::{array, num::NonZeroU32};

use objc2_metal::MTLVertexFormat;

use crate::{
    context::{debug::gl_debug, error::gl_assert},
    dispatch::{
        conversions::sizei,
        gl_types::{GLintptr, GLsizei, GLuint, GLvoid},
    },
    enums::VertexAttribType,
};

use super::{
    commands::buffer::Buffer,
    error::{GlError, GlFallible},
    gl_object::{LateInit, NamedObject, ObjectName},
};

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
    pub(crate) fn new_default(name: ObjectName<Self>) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            name,
            attribs: array::from_fn(|i| VertexAttrib::new_default(i as u8)),
            buffer_bindings: [AttributeBufferBinding::new_default();
                MAX_VERTEX_ATTRIB_BUFFER_BINDINGS],
        }
    }
    #[track_caller]
    fn get_binding_mut(&mut self, idx: u32) -> GlFallible<&mut AttributeBufferBinding> {
        self.buffer_bindings
            .get_mut(idx as usize)
            .ok_or(GlError::InvalidValue.e())
    }
    #[track_caller]
    fn get_attrib_mut(&mut self, idx: u32) -> GlFallible<&mut VertexAttrib> {
        self.attribs
            .get_mut(idx as usize)
            .ok_or(GlError::InvalidValue.e())
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub(crate) fn attrib_format(
        &mut self,
        attrib_index: u32,
        num_components: GLuint,
        r#type: VertexAttribType,
        relative_offset: GLuint,
        integer_behavior: IntegralCastBehavior,
    ) -> GlFallible {
        let attrib = self.get_attrib_mut(attrib_index)?;
        gl_assert!(
            relative_offset <= u32::from(MAX_VERTEX_ATTRIBUTE_STRIDE),
            InvalidValue
        );
        gl_assert!(
            num_components <= u32::from(MAX_VERTEX_ATTRIBUTE_STRIDE),
            InvalidValue
        );
        // Caller ensures num_components is in-bounds
        attrib.components = num_components as u8;
        attrib.component_type = r#type;

        attrib.relative_offset = relative_offset as u16;
        attrib.validate()?;
        gl_debug!(
            "updated vertex attribute format at index {attrib_index} of {:#?} to {:?}*{num_components} with relative offset {relative_offset} and integer behavior {integer_behavior:?}",
            self.name,
            r#type
        );
        Ok(())
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub(crate) fn buffers(
        &mut self,
        idx: u32,
        buffers: impl ExactSizeIterator<Item = Option<ObjectName<Buffer>>>,
        offsets: impl ExactSizeIterator<Item = GLintptr> + Clone,
        strides: impl ExactSizeIterator<Item = GLsizei> + Clone,
    ) -> GlFallible {
        gl_assert!(
            idx as usize + buffers.len() <= MAX_VERTEX_ATTRIBUTES,
            InvalidValue
        );
        // need prepass check for errors to avoid side-effects if we error. Should also optimize out nicer
        for (offset, stride) in offsets.clone().zip(strides.clone()) {
            gl_assert!(offset >= 0 && stride >= 0, InvalidValue);
            gl_assert!(stride <= MAX_VERTEX_ATTRIBUTE_STRIDE.into(), InvalidValue);
        }

        let mut bindingindex = idx;
        for (name, (offset, stride)) in buffers.zip(offsets.zip(strides)) {
            gl_debug!("bound {name:?} to {:?} at binding index {bindingindex} with offset {offset} and stride {stride}", self.name);
            let r = self.get_binding_mut(bindingindex)?;
            r.buf = name;
            r.offset = offset as usize;
            r.stride = stride as u16;

            bindingindex += 1;
        }
        Ok(())
    }
    pub(crate) fn set_attrib_enabled(&mut self, idx: u32, state: bool) -> GlFallible {
        gl_debug!(
            "set vertex attribute at index {idx} of {:?} enabled to {}",
            self.name,
            state
        );
        self.get_attrib_mut(idx)?.enabled = state;
        Ok(())
    }
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn attrib_binding(&mut self, attrib_index: u32, binding_index: u32) -> GlFallible {
        gl_assert!(
            (binding_index as usize) < MAX_VERTEX_ATTRIB_BUFFER_BINDINGS,
            InvalidValue,
            "out of bounds attribute buffer binding index"
        );
        gl_debug!(
            "binding vertex attribute index {attrib_index} to {:?} buffer binding {binding_index}",
            self.name
        );

        self.get_attrib_mut(attrib_index)?.buffer_idx = binding_index as u8;
        Ok(())
    }
    pub(crate) fn binding_divisor(&mut self, binding_index: u32, divisor: u32) -> GlFallible {
        gl_debug!("setting vertex attribute binding divisor at buffer binding index {binding_index} to {divisor}");
        self.get_binding_mut(binding_index)?.divisor = NonZeroU32::new(divisor);
        Ok(())
    }
    #[allow(clippy::too_many_arguments, clippy::cast_possible_truncation)]
    pub(crate) fn vertex_attrib_pointer(
        &mut self,
        index: GLuint,
        num_components: GLuint,
        ty: VertexAttribType,
        stride: GLsizei,
        pointer: *const GLvoid,
        integer_behavior: IntegralCastBehavior,
        array_buffer: ObjectName<Buffer>,
    ) -> GlFallible {
        let name = self.name;
        let attr_ref = self.get_attrib_mut(index)?;
        let mut attrib = *attr_ref;

        gl_assert!(num_components <= 4, InvalidValue);
        attrib.components = num_components as u8;
        attrib.component_type = ty;

        attrib.relative_offset = 0;

        gl_debug!(
            "updated vertex attribute format at index {index} of {name:?} to {ty:?}*{num_components} with relative offset 0 and integer behavior {integer_behavior:?}",
        );
        gl_assert!(
            (index as usize) < MAX_VERTEX_ATTRIB_BUFFER_BINDINGS,
            InvalidValue,
            "out of bounds attribute buffer binding index"
        );
        gl_debug!("binding vertex attribute index {index} to {name:?} buffer binding {index}",);
        attrib.buffer_idx = index as u8;
        sizei!(stride);
        let stride = if stride > 0 {
            stride
        } else {
            u32::from(attrib.compute_stride())
        };
        attrib.validate()?;
        gl_assert!(stride <= MAX_VERTEX_ATTRIBUTE_STRIDE.into(), InvalidValue);
        // last check passed, we're allowed to have side effects now
        *attr_ref = attrib;
        let binding = self.get_binding_mut(index)?;
        binding.buf = Some(array_buffer);
        binding.offset = pointer.addr();

        binding.stride = stride as u16;
        Ok(())
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
}
