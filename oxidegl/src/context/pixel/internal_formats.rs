use super::{
    ConvertPixel, Depth, NormalizedIntChannel, RgColorFormat, RgbaColorFormat, SingleChannelFormat,
};
use crate::enums::InternalFormat;
use half::f16;
use objc2_metal::MTLPixelFormat;

pub(crate) trait InternalFormatInfo {
    /// The underlying [`MTLPixelFormat`] used to represent this [`InternalFormat`].
    fn mtl_tex_format() -> MTLPixelFormat;
    fn copyable() -> bool;
    fn view_class() -> Option<TextureViewClass>;
    fn id() -> InternalFormat;
}

macro_rules! decl_internal_formats {
    (
        unsafe {
            $(
                #[mtl_format = $mtl_format:ident]
                repr( $( $body:tt )* ) {
                    $(
                        $( #[doc = $doc:expr] )*
                        $( #[view_class = $view_class:ident ] )?
                        $( #[not_copyable $_mark:vis] )?
                        format struct $format:ident;
                    )+

                }
            )+
        }
    ) => {
        $(

            ::concat_idents::concat_idents! {
                fmt_name = Mtl, $mtl_format {
                    #[allow(non_camel_case_types)]
                    #[derive(Clone, Copy, Debug)]
                    pub(crate) struct fmt_name( $( $body )* );
                }

            }
            $(
                ::concat_idents::concat_idents! {
                    fmt_name = Mtl, $mtl_format {
                        $( #[doc = $doc] )*
                        #[repr(transparent)]
                        #[derive(Clone, Copy, Debug)]
                        pub(crate) struct $format( fmt_name );
                    }
                }
                impl InternalFormatInfo for $format {
                    fn mtl_tex_format() -> MTLPixelFormat {
                        MTLPixelFormat::$mtl_format
                    }
                    fn copyable() -> bool {
                        decl_internal_formats! { present: $(v $_mark)? }
                    }
                    fn view_class() -> Option<TextureViewClass> {
                        decl_internal_formats! {
                            optional: $( $view_class )?
                        }
                    }
                    fn id() -> InternalFormat {
                        InternalFormat::$format
                    }
                }
            )+
        )+
        trait ConvertibleToInternalFormat: $( $(ConvertPixel<$format> + )+ )+ Copy {}
        impl<T: $( $(ConvertPixel<$format> + )+ )+ Copy> ConvertibleToInternalFormat for T {}
    };
    // helpers
    (
        present: v
    ) => {
        true
    };
    (
        present:
    ) => {
        false
    };
    (
        optional: $val:ident
    ) => {
        Some(TextureViewClass::$val)
    };
    (
        optional:
    ) => {
        None
    };
}

decl_internal_formats! {
    unsafe {
        // Red
        // 8bit
        #[mtl_format = R8Unorm]
        repr(SingleChannelFormat<NormalizedIntChannel<u8>, super::Red>) {
            #[view_class = Bits8]
            format struct R8;
            format struct CompressedRed;
            format struct Red;
        }
        #[mtl_format = R8Snorm]
        repr(SingleChannelFormat<NormalizedIntChannel<i8>, super::Red>) {
            #[view_class = Bits8]
            format struct R8Snorm;
        }
        #[mtl_format = R8Sint]
        repr(SingleChannelFormat<i8, super::Red>) {
            #[view_class = Bits8]
            format struct R8i;
        }
        #[mtl_format = R8Uint]
        repr(SingleChannelFormat<u8, super::Red>) {
            #[view_class = Bits8]
            format struct R8ui;
        }

        // 16 bit
        #[mtl_format = R16Unorm]
        repr(SingleChannelFormat<NormalizedIntChannel<u16>, super::Red>) {
            #[view_class = Bits16]
            format struct R16;
        }
        #[mtl_format = R16Snorm]
        repr(SingleChannelFormat<NormalizedIntChannel<i16>, super::Red>) {
            #[view_class = Bits16]
            format struct R16Snorm;
        }
        #[mtl_format = R16Sint]
        repr(SingleChannelFormat<i16, super::Red>) {
            #[view_class = Bits16]
            format struct R16i;
        }
        #[mtl_format = R16Uint]
        repr(SingleChannelFormat<u16, super::Red>) {
            #[view_class = Bits16]
            format struct R16ui;
        }

        // 32 bit
        // #[mtl_format = R32Unorm]
        // repr(SingleChannelFormat<NormalizedIntChannel<u32>, super::Red>) {
        //     #[view_class = Bits32]
        //     format struct R32;
        // }
        // #[mtl_format = R32Snorm]
        // repr(SingleChannelFormat<NormalizedIntChannel<i32>, super::Red>) {
        //     #[view_class = Bits32]
        //     format struct R32Snorm;
        // }
        #[mtl_format = R32Sint]
        repr(SingleChannelFormat<i32, super::Red>) {
            #[view_class = Bits32]
            format struct R32i;
        }
        #[mtl_format = R32Uint]
        repr(SingleChannelFormat<u32, super::Red>) {
            #[view_class = Bits32]
            format struct R32ui;
        }

        #[mtl_format = R32Float]
        repr(SingleChannelFormat<f32, super::Red>) {
            #[view_class = Bits32]
            format struct R32f;
        }
        #[mtl_format = R16Float]
        repr(SingleChannelFormat<f16, super::Red>) {
            #[view_class = Bits16]
            format struct R16f;
        }

        // Red Green

        // 8 bit
        #[mtl_format = RG8Unorm]
        repr(RgColorFormat<NormalizedIntChannel<u8>>) {
            #[view_class = Bits16]
            format struct Rg8;
            format struct Rg;
            format struct CompressedRg;
        }
        #[mtl_format = RG8Snorm]
        repr(RgColorFormat<NormalizedIntChannel<i8>>) {
            #[view_class = Bits16]
            format struct Rg8Snorm;
        }
        #[mtl_format = RG8Sint]
        repr(RgColorFormat<i8>) {
            #[view_class = Bits16]
            format struct Rg8i;
        }
        #[mtl_format = RG8Uint]
        repr(RgColorFormat<u8>) {
            #[view_class = Bits16]
            format struct Rg8ui;
        }

        // 16 bit
        #[mtl_format = RG16Unorm]
        repr(RgColorFormat<NormalizedIntChannel<u16>>) {
            #[view_class = Bits32]
            format struct Rg16;

        }
        #[mtl_format = RG16Snorm]
        repr(RgColorFormat<NormalizedIntChannel<i16>>) {
            #[view_class = Bits32]
            format struct Rg16Snorm;
        }
        #[mtl_format = RG16Sint]
        repr(RgColorFormat<i16>) {
            #[view_class = Bits32]
            format struct Rg16i;
        }
        #[mtl_format = RG16Uint]
        repr(RgColorFormat<u16>) {
            #[view_class = Bits32]
            format struct Rg16ui;
        }

        // 32 bit
        // #[mtl_format = RG32Unorm]
        // repr(RgColorFormat<NormalizedIntChannel<u32>>) {
        //     #[view_class = Bits64]
        //     format struct Rg32;
        // }
        // #[mtl_format = RG32Snorm]
        // repr(RgColorFormat<NormalizedIntChannel<i32>>) {
        //     #[view_class = Bits64]
        //     format struct Rg32Snorm;
        // }
        #[mtl_format = RG32Sint]
        repr(RgColorFormat<i32>) {
            #[view_class = Bits64]
            format struct Rg32i;
        }
        #[mtl_format = RG32Uint]
        repr(RgColorFormat<u32>) {
            #[view_class = Bits64]
            format struct Rg32ui;
        }

        #[mtl_format = R16Float]
        repr(RgColorFormat<f16>) {
            #[view_class = Bits16]
            format struct Rg16f;
        }
        #[mtl_format = R32Float]
        repr(RgColorFormat<f32>) {
            #[view_class = Bits32]
            format struct Rg32f;
        }


        // rgb and rgba
        // 4 bit
        #[mtl_format = ABGR4Unorm]
        repr(u16) {
            format struct Rgb4;
            #[view_class = Bits16]
            format struct Rgba4;
        }
        // 5 bit
        #[mtl_format = A1BGR5Unorm]
        repr(u16) {
            format struct Rgb5;
            #[view_class = Bits16]
            format struct Rgb5A1;
        }

        // 8 bit
        #[mtl_format = RGBA8Unorm]
        repr(RgbaColorFormat<NormalizedIntChannel<u8>>) {
            #[view_class = Bits24]
            format struct Rgb8;
            format struct Rgb;
            #[view_class = Bits32]
            format struct Rgba8;
            format struct Rgba;
        }
        #[mtl_format = RGBA8Snorm]
        repr(RgbaColorFormat<NormalizedIntChannel<i8>>) {
            #[view_class = Bits24]
            format struct Rgb8Snorm;
            #[view_class = Bits32]
            format struct Rgba8Snorm;
        }
        #[mtl_format = RGBA8Sint]
        repr(RgbaColorFormat<i8>) {
            #[view_class = Bits24]
            format struct Rgb8i;
            #[view_class = Bits32]
            format struct Rgba8i;
        }
        #[mtl_format = RGBA8Uint]
        repr(RgbaColorFormat<u8>) {
            #[view_class = Bits24]
            format struct Rgb8ui;
            #[view_class = Bits32]
            format struct Rgba8ui;
        }
        // 10 bit
        #[mtl_format = RGB10A2Unorm]
        repr(u32) {
            format struct Rgb10;
            #[view_class = Bits32]
            format struct Rgb10A2;
        }
        // 16 bit
        #[mtl_format = RGBA16Unorm]
        repr(RgbaColorFormat<NormalizedIntChannel<u16>>) {
            #[view_class = Bits48]
            format struct Rgb16;
            format struct Rgb12;
            #[view_class = Bits64]
            format struct Rgba16;
            #[view_class = Bits48]
            format struct Rgba12;
        }
        #[mtl_format = RGBA16Snorm]
        repr(RgbaColorFormat<NormalizedIntChannel<i16>>) {
            #[view_class = Bits48]
            format struct Rgb16Snorm;
            #[view_class = Bits64]
            format struct Rgba16Snorm;
        }
        #[mtl_format = RGBA16Sint]
        repr(RgbaColorFormat<i16>) {
            #[view_class = Bits48]
            format struct Rgb16i;
            #[view_class = Bits64]
            format struct Rgba16i;
        }
        #[mtl_format = RGBA16Uint]
        repr(RgbaColorFormat<u16>) {
            #[view_class = Bits48]
            format struct Rgb16ui;
            #[view_class = Bits64]
            format struct Rgba16ui;
        }

         // 32 bit
        //  #[mtl_format = RGBA32Unorm]
        //  repr(RgbaColorFormat<NormalizedIntChannel<u32>>) {
        //      #[view_class = Bits96]
        //      format struct Rgb32;
        //      #[view_class = Bits128]
        //      format struct Rgba32;
        //  }
        //  #[mtl_format = RGBA32Snorm]
        //  repr(RgbaColorFormat<NormalizedIntChannel<i32>>) {
        //      #[view_class = Bits96]
        //      format struct Rgb32Snorm;
        //      #[view_class = Bits128]
        //      format struct Rgba32Snorm;
        //  }
        #[mtl_format = RGBA32Sint]
        repr(RgbaColorFormat<i32>) {
            #[view_class = Bits96]
            format struct Rgb32i;
            #[view_class = Bits128]
            format struct Rgba32i;
        }
        #[mtl_format = RGBA32Uint]
        repr(RgbaColorFormat<u32>) {
            #[view_class = Bits96]
            format struct Rgb32ui;
            #[view_class = Bits128]
            format struct Rgba32ui;
        }
        #[mtl_format = RGBA8Unorm_sRGB]
        repr(RgbaColorFormat<NormalizedIntChannel<u8>>) {
            format struct CompressedSrgbAlpha;
            format struct CompressedSrgb;
            format struct Srgb;
            format struct SrgbAlpha;
            #[view_class = Bits24]
            format struct Srgb8;
            #[view_class = Bits32]
            format struct Srgb8Alpha8;
        }

        #[mtl_format = Stencil8]
        repr(SingleChannelFormat<u8, Stencil>) {
            format struct StencilIndex1;
            format struct StencilIndex4;
            format struct StencilIndex;
            format struct StencilIndex8;
        }
        #[mtl_format = Depth32Float_Stencil8]
        repr([u8; 5]) {
            format struct Depth32fStencil8;
        }
        #[mtl_format = Depth24Unorm_Stencil8]
        repr(u32) {
            format struct DepthComponent24;
            format struct DepthComponent24Stencil8;
            format struct DepthStencil;
        }

        #[mtl_format = Depth32Float]
        repr(SingleChannelFormat<f32, Depth>) {
            format struct DepthComponent;
            format struct DepthComponent32f;
            // should be DepthComponent32Unorm but metal doesn't have that. We just use floats instead :)
            format struct DepthComponent32;
        }


    }

}
#[allow(clippy::enum_glob_use)]
impl InternalFormat {
    #[allow(clippy::too_many_lines)]
    pub(crate) const fn mtl_texture_format(self) -> MTLPixelFormat {
        use InternalFormat::*;
        use MTLPixelFormat as M;
        // fallback to uncompressed for "default" compressed formats. People really shouldn't be using them anyways...
        // TODO should emit a performance warning from compressedXYZ fallbacks
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
            // oddball formats that metal (rightly?) dropped
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
