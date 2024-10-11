use log::trace;
use objc2_metal::MTLPixelFormat;

use crate::enums::{PixelFormat, PixelType};

#[derive(Debug, Copy, Clone)]
pub struct GLPixelTypeFormat {
    ty: PixelType,
    fmt: PixelFormat,
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
            PixelType::UnsignedShort4444Rev => todo!(),
            PixelType::UnsignedShort1555Rev => todo!(),
            PixelType::UnsignedInt5999Rev => todo!(),
            PixelType::Float32UnsignedInt248Rev => todo!(),
            PixelType::UnsignedInt248 => todo!(),
            PixelType::UnsignedShort565Rev => todo!(),
        }
    }
}
