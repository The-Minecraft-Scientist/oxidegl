use log::trace;
use objc2_metal::MTLPixelFormat;

use crate::enums::{InternalFormat, PixelFormat, PixelType, GL_VIEW_CLASS_128_BITS};

use super::state::ObjectName;

#[derive(Debug, Copy, Clone)]
pub struct GLPixelTypeFormat {
    ty: PixelType,
    fmt: PixelFormat,
}

pub struct Texture {
    name: ObjectName<Self>,
    format: InternalFormat,
    width: u32,
    height: u32,
    is_tex_array: bool,
}

#[allow(clippy::enum_glob_use)]
impl InternalFormat {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn mtl_format(self) -> MTLPixelFormat {
        use InternalFormat::*;
        use MTLPixelFormat as MF;
        // falback to uncompressed for "default" compressed formats. People really shouldn't be using them anyways...
        match self {
            Rgb4 | Rgba4 => MF::ABGR4Unorm,
            Rgb5 | Rgb5A1 => MF::A1BGR5Unorm,

            CompressedRgba | CompressedRgb | Rgb8 | Rgb | Rgba | Rgba8 => MF::RGBA8Unorm,
            Rgba8Snorm | Rgb8Snorm => MF::RGBA8Snorm,

            Rgb10 | Rgb10A2 => MF::RGB10A2Unorm,
            Rgba16Snorm | Rgb16Snorm => MF::RGBA16Snorm,
            Rgb16 | Rgb12 | Rgba16 | Rgba12 => MF::RGBA16Unorm,

            DepthComponent16 => MF::Depth16Unorm,
            DepthComponent | DepthComponent32f | DepthComponent32 => MF::Depth32Float,

            Depth32fStencil8 => MF::Depth32Float_Stencil8,
            DepthComponent24 | Depth24Stencil8 | DepthStencil => MF::Depth24Unorm_Stencil8,

            CompressedSrgbAlpha | CompressedSrgb | Srgb8 | Srgb | Srgb8Alpha8 | SrgbAlpha => {
                MF::RGBA8Unorm_sRGB
            }

            // etc2 and eac
            CompressedRgb8Etc2 => MF::ETC2_RGB8,
            CompressedSrgb8Etc2 => MF::ETC2_RGB8_sRGB,
            CompressedRgba8Etc2Eac => MF::ETC2_RGB8A1,
            CompressedSrgb8Alpha8Etc2Eac => MF::ETC2_RGB8A1_sRGB,
            CompressedR11Eac => MF::EAC_R11Unorm,
            CompressedSignedR11Eac => MF::EAC_R11Snorm,
            CompressedRg11Eac => MF::EAC_RG11Unorm,
            CompressedSignedRg11Eac => MF::EAC_RG11Snorm,

            // rgtc (aka BC4)
            CompressedRedRgtc1 => MF::BC4_RUnorm,
            CompressedSignedRedRgtc1 => MF::BC4_RSnorm,
            CompressedRgRgtc2 => MF::BC5_RGUnorm,
            CompressedSignedRgRgtc2 => MF::BC5_RGSnorm,

            // bptc (aka BC6H and BC7)
            CompressedRgbaBptcUnorm => MF::BC7_RGBAUnorm,
            CompressedSrgbAlphaBptcUnorm => MF::BC7_RGBAUnorm_sRGB,
            CompressedRgbBptcSignedFloat => MF::BC6H_RGBFloat,
            CompressedRgbBptcUnsignedFloat => MF::BC6H_RGBUfloat,

            Rgb32f | Rgba32f => MF::RGBA32Float,
            Rgb16f | Rgba16f => MF::RGBA16Float,
            R11fG11fB10f => MF::RG11B10Float,

            Rgb9E5 => MF::RGB9E5Float,
            Rgb32ui | Rgba32ui => MF::RGBA32Uint,
            Rgb16ui | Rgba16ui => MF::RGBA16Uint,
            Rgb8ui | Rgba8ui => MF::RGBA8Uint,

            Rgb32i | Rgba32i => MF::RGBA32Sint,
            Rgb16i | Rgba16i => MF::RGBA16Sint,
            Rgb8i | Rgba8i => MF::RGBA8Sint,
            StencilIndex1 | StencilIndex4 | StencilIndex | StencilIndex8 => MF::Stencil8,

            R8 | CompressedRed | Red => MF::R8Unorm,
            R8Snorm => MF::R8Snorm,
            R8i => MF::R8Sint,
            R8ui => MF::R8Uint,

            R16 => MF::R16Unorm,
            R16Snorm => MF::R16Snorm,
            R16i => MF::R16Sint,
            R16ui => MF::R16Uint,
            R16f => MF::R16Float,

            R32i => MF::R32Sint,
            R32ui => MF::R32Uint,
            R32f => MF::R32Float,

            CompressedRg | Rg | Rg8 => MF::RG8Unorm,
            Rg8Snorm => MF::RG8Snorm,
            Rg8i => MF::RG8Sint,
            Rg8ui => MF::RG8Uint,

            Rg16 => MF::RG16Unorm,
            Rg16Snorm => MF::RG16Snorm,
            Rg16ui => MF::RG16Uint,
            Rg16i => MF::RG16Sint,
            Rg16f => MF::RG16Float,

            Rg32f => MF::RG32Float,
            Rg32i => MF::RG32Sint,
            Rg32ui => MF::RG32Uint,

            Rgb10A2ui => MF::RGB10A2Uint,

            // would require emulating stencil test for only this stencil type
            StencilIndex16 => todo!(),
            // oddball formats that metal (rightly) dropped
            Rgb565 => todo!(),
            R3G3B2 => todo!(),
            Rgba2 => todo!(),

            // unsupported EAC modes
            CompressedRgb8PunchthroughAlpha1Etc2 => todo!(),
            CompressedSrgb8PunchthroughAlpha1Etc2 => todo!(),
        }
    }
    pub(crate) fn is_gl_copyable(self) -> bool {
        use InternalFormat::*;
        !matches!(
            self,
            CompressedRgb8Etc2
                | CompressedSrgb8Etc2
                | CompressedRgb8PunchthroughAlpha1Etc2
                | CompressedSrgb8PunchthroughAlpha1Etc2
                | CompressedRgba8Etc2Eac
                | CompressedSrgb8Alpha8Etc2Eac
                | CompressedR11Eac
                | CompressedSignedR11Eac
                | CompressedRg11Eac
                | CompressedSignedRg11Eac
        )
    }
    pub(crate) fn view_class(self) -> Option<TextureViewClass> {
        use InternalFormat::*;
        match self {
            Rgba32f | Rgba32ui | Rgba32i => Some(TextureViewClass::Bits128),
            Rgb32f | Rgb32ui | Rgb32i => Some(TextureViewClass::Bits96),
            Rgba16f | Rgba16ui | Rgba16i | Rgba16Snorm | Rgba16 | Rg32f | Rg32i | Rg32ui => {
                Some(TextureViewClass::Bits64)
            }
            Rgb16 | Rgb16Snorm | Rgb16f | Rgb16ui | Rgb16i => Some(TextureViewClass::Bits48),
            Rg16f | R11fG11fB10f | R32f | Rgb10A2ui | Rgba8ui | Rg16ui | R32ui | Rgba8i | Rg16i
            | R32i | Rgb10A2 | Rgba8 | Rg16 | Rgba8Snorm | Srgb8Alpha8 | Rgb9E5 => {
                Some(TextureViewClass::Bits32)
            }
            Rgb8 | Rgb8Snorm | Srgb8 | Rgb8i | Rgb8ui => Some(TextureViewClass::Bits24),
            R16f | Rg8ui | R16ui | Rg8i | R16i | Rg8 | R16 | Rg8Snorm | R16Snorm => {
                Some(TextureViewClass::Bits16)
            }
            R8ui | R8i | R8 | R8Snorm => Some(TextureViewClass::Bits8),
            CompressedRedRgtc1 | CompressedSignedRedRgtc1 => Some(TextureViewClass::RgtcRed),
            CompressedRgRgtc2 | CompressedSignedRgRgtc2 => Some(TextureViewClass::RgtcRg),
            CompressedRgbaBptcUnorm | CompressedSrgbAlphaBptcUnorm => {
                Some(TextureViewClass::BptcUnorm)
            }
            CompressedRgbBptcSignedFloat | CompressedRgbBptcUnsignedFloat => {
                Some(TextureViewClass::BptcFloat)
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureViewClass {
    Bits128,
    Bits96,
    Bits64,
    Bits48,
    Bits32,
    Bits24,
    Bits16,
    Bits8,
    RgtcRed,
    RgtcRg,
    BptcUnorm,
    BptcFloat,
}
impl GLPixelTypeFormat {
    pub fn new(ty: PixelType, fmt: PixelFormat) -> Self {
        Self { ty, fmt }
    }
    pub fn to_mtl_format(self) -> MTLPixelFormat {
        use MTLPixelFormat as MF;
        use PixelFormat::{
            DepthComponent, DepthStencil, Red, RedInteger, Rg, RgInteger, Rgba, RgbaInteger,
        };

        trace!(
            "converting GL Pixel format with type {:?}, and fmt {:?}",
            self.ty,
            self.fmt
        );
        match self.ty {
            PixelType::Byte => match self.fmt {
                Red => MF::R8Snorm,
                Rg => MF::RG8Snorm,
                Rgba => MF::RGBA8Snorm,
                RedInteger => MF::R8Sint,
                RgInteger => MF::RG8Sint,
                RgbaInteger => MF::RGBA8Sint,

                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedByte => match self.fmt {
                Red => MF::R8Unorm,
                Rg => MF::RG8Unorm,
                Rgba => MF::RGBA8Unorm,
                RedInteger => MF::R8Uint,
                RgInteger => MF::RG8Uint,
                RgbaInteger => MF::RGBA8Uint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::Short => match self.fmt {
                Red => MF::R16Snorm,
                Rg => MF::RG16Snorm,
                Rgba => MF::RGBA16Snorm,
                RedInteger => MF::R16Sint,
                RgInteger => MF::RG16Sint,
                RgbaInteger => MF::RGBA16Sint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedShort => match self.fmt {
                Red => MF::R16Unorm,
                Rg => MF::RG16Unorm,
                Rgba => MF::RGBA16Unorm,
                RedInteger => MF::R16Uint,
                RgInteger => MF::RG16Uint,
                RgbaInteger => MF::RGBA16Uint,
                _ => panic!("invalid source pixel format"),
            },

            // TODO: emulate GL normalization behavior for (U)Int,
            // those formats are not supported for now
            PixelType::Int => match self.fmt {
                // Red => MF::R32Uint,
                // Rg => MF::RG32Snorm,
                // Rgba => MF::RGBA32Snorm,
                RedInteger => MF::R32Sint,
                RgInteger => MF::RG32Sint,
                RgbaInteger => MF::RGBA32Sint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedInt => match self.fmt {
                // Red => MF::R32Unorm,
                // Rg => MF::RG32Unorm,
                // Rgba => MF::RGBA32Unorm,
                RedInteger => MF::R32Uint,
                RgInteger => MF::RG32Uint,
                RgbaInteger => MF::RGBA32Uint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::Float => match self.fmt {
                Red => MF::R32Float,
                Rg => MF::RG32Float,
                Rgba => MF::RGBA32Float,
                DepthComponent => MF::Depth32Float,
                DepthStencil => MF::Depth24Unorm_Stencil8,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::HalfFloat => match self.fmt {
                Red => MF::R16Float,
                Rg => MF::RG16Float,
                Rgba => MF::RGBA16Float,
                _ => panic!("invalid source pixel format"),
            },
            // special-purpose formats
            PixelType::UnsignedInt10F11F11FRev => MF::RG11B10Float,
            PixelType::UnsignedShort565 => MF::B5G6R5Unorm,
            PixelType::UnsignedInt8888 => MF::RGBA8Unorm,
            PixelType::UnsignedInt8888Rev => MF::BGRA8Unorm,
            PixelType::UnsignedInt1010102 => MF::RGB10A2Unorm,
            PixelType::UnsignedInt2101010Rev => MF::BGR10A2Unorm,

            PixelType::UnsignedByte332 => todo!(),
            PixelType::UnsignedShort4444 => todo!(),
            PixelType::UnsignedShort5551 => todo!(),
            PixelType::UnsignedByte233Rev => todo!(),
            PixelType::UnsignedShort4444Rev => MF::ABGR4Unorm,
            PixelType::UnsignedShort1555Rev => MF::A1BGR5Unorm,
            PixelType::UnsignedInt5999Rev => todo!(),
            PixelType::Float32UnsignedInt248Rev => todo!(),
            PixelType::UnsignedInt248 => todo!(),
            PixelType::UnsignedShort565Rev => todo!(),
        }
    }
}
