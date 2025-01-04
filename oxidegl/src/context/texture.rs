use std::num::NonZeroU32;

use log::trace;
use objc2_metal::{MTLPixelFormat, MTLTexture};

use crate::{
    enums::{InternalFormat, PixelFormat, PixelType, TextureTarget},
    ProtoObjRef,
};

use super::state::ObjectName;

#[derive(Debug)]
pub struct Texture {
    name: ObjectName<Self>,
    target: TextureTarget,
    mtl_tex: ProtoObjRef<dyn MTLTexture>,

    format: InternalFormat,
    width: u32,
    height: Option<NonZeroU32>,
    depth: Option<NonZeroU32>,
    array_length: Option<NonZeroU32>,
}

#[allow(clippy::enum_glob_use)]
impl InternalFormat {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn mtl_format(self) -> MTLPixelFormat {
        use InternalFormat::*;
        use MTLPixelFormat as M;
        // fallback to uncompressed for "default" compressed formats. People really shouldn't be using them anyways...
        match self {
            Rgb4 | Rgba4 => M::ABGR4Unorm,
            Rgb5 | Rgb5A1 => M::A1BGR5Unorm,

            CompressedRgba | CompressedRgb | Rgb8 | Rgb | Rgba | Rgba8 => M::RGBA8Unorm,
            Rgba8Snorm | Rgb8Snorm => M::RGBA8Snorm,

            Rgb10 | Rgb10A2 => M::RGB10A2Unorm,
            Rgba16Snorm | Rgb16Snorm => M::RGBA16Snorm,
            Rgb16 | Rgb12 | Rgba16 | Rgba12 => M::RGBA16Unorm,

            DepthComponent16 => M::Depth16Unorm,
            DepthComponent | DepthComponent32f | DepthComponent32 => M::Depth32Float,

            Depth32fStencil8 => M::Depth32Float_Stencil8,
            DepthComponent24 | Depth24Stencil8 | DepthStencil => M::Depth24Unorm_Stencil8,

            StencilIndex1 | StencilIndex4 | StencilIndex | StencilIndex8 => M::Stencil8,

            CompressedSrgbAlpha | CompressedSrgb | Srgb8 | Srgb | Srgb8Alpha8 | SrgbAlpha => {
                M::RGBA8Unorm_sRGB
            }

            // etc2 and eac
            CompressedRgb8Etc2 => M::ETC2_RGB8,
            CompressedSrgb8Etc2 => M::ETC2_RGB8_sRGB,
            CompressedRgba8Etc2Eac => M::ETC2_RGB8A1,
            CompressedSrgb8Alpha8Etc2Eac => M::ETC2_RGB8A1_sRGB,
            CompressedR11Eac => M::EAC_R11Unorm,
            CompressedSignedR11Eac => M::EAC_R11Snorm,
            CompressedRg11Eac => M::EAC_RG11Unorm,
            CompressedSignedRg11Eac => M::EAC_RG11Snorm,

            // rgtc (aka BC4 and 5)
            CompressedRedRgtc1 => M::BC4_RUnorm,
            CompressedSignedRedRgtc1 => M::BC4_RSnorm,
            CompressedRgRgtc2 => M::BC5_RGUnorm,
            CompressedSignedRgRgtc2 => M::BC5_RGSnorm,

            // bptc (aka BC6H and BC7)
            CompressedRgbaBptcUnorm => M::BC7_RGBAUnorm,
            CompressedSrgbAlphaBptcUnorm => M::BC7_RGBAUnorm_sRGB,
            CompressedRgbBptcSignedFloat => M::BC6H_RGBFloat,
            CompressedRgbBptcUnsignedFloat => M::BC6H_RGBUfloat,

            Rgb32f | Rgba32f => M::RGBA32Float,
            Rgb16f | Rgba16f => M::RGBA16Float,
            R11fG11fB10f => M::RG11B10Float,

            Rgb9E5 => M::RGB9E5Float,
            Rgb32ui | Rgba32ui => M::RGBA32Uint,
            Rgb16ui | Rgba16ui => M::RGBA16Uint,
            Rgb8ui | Rgba8ui => M::RGBA8Uint,

            Rgb32i | Rgba32i => M::RGBA32Sint,
            Rgb16i | Rgba16i => M::RGBA16Sint,
            Rgb8i | Rgba8i => M::RGBA8Sint,

            R8 | CompressedRed | Red => M::R8Unorm,
            R8Snorm => M::R8Snorm,
            R8i => M::R8Sint,
            R8ui => M::R8Uint,

            R16 => M::R16Unorm,
            R16Snorm => M::R16Snorm,
            R16i => M::R16Sint,
            R16ui => M::R16Uint,
            R16f => M::R16Float,

            R32i => M::R32Sint,
            R32ui => M::R32Uint,
            R32f => M::R32Float,

            CompressedRg | Rg | Rg8 => M::RG8Unorm,
            Rg8Snorm => M::RG8Snorm,
            Rg8i => M::RG8Sint,
            Rg8ui => M::RG8Uint,

            Rg16 => M::RG16Unorm,
            Rg16Snorm => M::RG16Snorm,
            Rg16ui => M::RG16Uint,
            Rg16i => M::RG16Sint,
            Rg16f => M::RG16Float,

            Rg32f => M::RG32Float,
            Rg32i => M::RG32Sint,
            Rg32ui => M::RG32Uint,

            Rgb10A2ui => M::RGB10A2Uint,

            // would require emulating stencil test for only this stencil type
            StencilIndex16 => todo!(),
            // oddball formats that metal (rightly) dropped
            Rgb565 => todo!(),
            R3G3B2 => todo!(),
            Rgba2 => todo!(),

            // unsupported ETC2 compression modes
            CompressedRgb8PunchthroughAlpha1Etc2 | CompressedSrgb8PunchthroughAlpha1Etc2 => {
                panic!("OxideGL does not support ETC2 punchthrough format variations")
            }
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
        // Note: Metal additionally requires that the bit length of a pixel format is one of: 8, 16, 32, 64, or 128
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

#[derive(Debug, Copy, Clone)]
/// A combination of a [`PixelType`] and a [`PixelFormat`], used to specify input and output formats.
/// When user code requests or supplies textures, they specify the format with this pair, as opposed to using an [`InternalFormat`]
pub struct GLPixelTypeFormat {
    ty: PixelType,
    fmt: PixelFormat,
}

impl GLPixelTypeFormat {
    pub fn new(ty: PixelType, fmt: PixelFormat) -> Self {
        Self { ty, fmt }
    }
    pub fn to_mtl_format(self) -> MTLPixelFormat {
        use MTLPixelFormat as M;
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
                Red => M::R8Snorm,
                Rg => M::RG8Snorm,
                Rgba => M::RGBA8Snorm,
                RedInteger => M::R8Sint,
                RgInteger => M::RG8Sint,
                RgbaInteger => M::RGBA8Sint,

                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedByte => match self.fmt {
                Red => M::R8Unorm,
                Rg => M::RG8Unorm,
                Rgba => M::RGBA8Unorm,
                RedInteger => M::R8Uint,
                RgInteger => M::RG8Uint,
                RgbaInteger => M::RGBA8Uint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::Short => match self.fmt {
                Red => M::R16Snorm,
                Rg => M::RG16Snorm,
                Rgba => M::RGBA16Snorm,
                RedInteger => M::R16Sint,
                RgInteger => M::RG16Sint,
                RgbaInteger => M::RGBA16Sint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedShort => match self.fmt {
                Red => M::R16Unorm,
                Rg => M::RG16Unorm,
                Rgba => M::RGBA16Unorm,
                RedInteger => M::R16Uint,
                RgInteger => M::RG16Uint,
                RgbaInteger => M::RGBA16Uint,
                _ => panic!("invalid source pixel format"),
            },

            // TODO: emulate GL normalization behavior for (U)Int,
            // those formats are not supported for now
            PixelType::Int => match self.fmt {
                // Red => M::R32Uint,
                // Rg => M::RG32Snorm,
                // Rgba => M::RGBA32Snorm,
                RedInteger => M::R32Sint,
                RgInteger => M::RG32Sint,
                RgbaInteger => M::RGBA32Sint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::UnsignedInt => match self.fmt {
                // Red => M::R32Unorm,
                // Rg => M::RG32Unorm,
                // Rgba => M::RGBA32Unorm,
                RedInteger => M::R32Uint,
                RgInteger => M::RG32Uint,
                RgbaInteger => M::RGBA32Uint,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::Float => match self.fmt {
                Red => M::R32Float,
                Rg => M::RG32Float,
                Rgba => M::RGBA32Float,
                DepthComponent => M::Depth32Float,
                DepthStencil => M::Depth24Unorm_Stencil8,
                _ => panic!("invalid source pixel format"),
            },
            PixelType::HalfFloat => match self.fmt {
                Red => M::R16Float,
                Rg => M::RG16Float,
                Rgba => M::RGBA16Float,
                _ => panic!("invalid source pixel format"),
            },
            // special-purpose formats
            PixelType::UnsignedInt10F11F11FRev => M::RG11B10Float,
            PixelType::UnsignedShort565 => M::B5G6R5Unorm,
            PixelType::UnsignedInt8888 => M::RGBA8Unorm,
            PixelType::UnsignedInt8888Rev => M::BGRA8Unorm,
            PixelType::UnsignedInt1010102 => M::RGB10A2Unorm,
            PixelType::UnsignedInt2101010Rev => M::BGR10A2Unorm,
            PixelType::UnsignedShort4444Rev => M::ABGR4Unorm,
            PixelType::UnsignedShort1555Rev => M::A1BGR5Unorm,

            PixelType::UnsignedByte332 => todo!(),
            PixelType::UnsignedShort4444 => todo!(),
            PixelType::UnsignedShort5551 => todo!(),
            PixelType::UnsignedByte233Rev => todo!(),
            PixelType::UnsignedInt5999Rev => todo!(),
            PixelType::Float32UnsignedInt248Rev => todo!(),
            PixelType::UnsignedInt248 => todo!(),
            PixelType::UnsignedShort565Rev => todo!(),
        }
    }
}
