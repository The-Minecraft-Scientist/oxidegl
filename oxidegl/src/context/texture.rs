use std::{cell::Cell, fmt::Debug, num::NonZeroU32};

use log::trace;
use objc2::rc::Retained;
use objc2_metal::{
    MTLPixelFormat, MTLSamplerAddressMode, MTLSamplerBorderColor, MTLSamplerDescriptor,
    MTLSamplerMinMagFilter, MTLSamplerMipFilter, MTLTexture, MTLTextureSwizzle, MTLTextureType,
};

use crate::{
    dispatch::conversions::SrcType,
    enums::{
        DepthFunction, InternalFormat, PixelFormat, PixelType, SamplerParameter, TextureMagFilter,
        TextureMinFilter, TextureSwizzle, TextureTarget, TextureWrapMode,
    },
    ProtoObjRef,
};

use super::gl_object::ObjectName;

/// * named: name is reserved, object is considered uninitialized
/// * bound: object is initialized to default state, has no storage
/// * complete: TODO
#[derive(Debug)]
pub struct Texture {
    name: ObjectName<Self>,
    target: TextureTarget,
    sampling_state: SamplerParams,
    realized: Option<RealizedTexture>,
}
/// Represents a realized texture's storage
#[derive(Debug)]
pub struct RealizedTexture {
    mtl_tex: Option<ProtoObjRef<dyn MTLTexture>>,
    format: InternalFormat,
    width: u32,
    height: Option<NonZeroU32>,
    depth: Option<NonZeroU32>,
    array_length: Option<NonZeroU32>,
}
impl Texture {
    fn new_named(name: ObjectName<Self>, target: TextureTarget) -> Self {
        Self {
            name,
            target,
            sampling_state: SamplerParams::default(),
            realized: None,
        }
    }
    fn make_immutable_storage(&mut self, levels: u32) {}
}
impl From<TextureTarget> for MTLTextureType {
    fn from(value: TextureTarget) -> Self {
        match value {
            TextureTarget::Texture1D | TextureTarget::ProxyTexture1D => Self::MTLTextureType1D,
            TextureTarget::Texture1DArray | TextureTarget::ProxyTexture1DArray => {
                Self::MTLTextureType1DArray
            }
            TextureTarget::Renderbuffer
            | TextureTarget::TextureRectangle
            | TextureTarget::ProxyTextureRectangle
            | TextureTarget::Texture2D
            | TextureTarget::ProxyTexture2D => Self::MTLTextureType2D,

            TextureTarget::Texture2DArray | TextureTarget::ProxyTexture2DArray => {
                Self::MTLTextureType2DArray
            }
            TextureTarget::Texture2DMultisample | TextureTarget::ProxyTexture2DMultisample => {
                Self::MTLTextureType2DMultisample
            }
            TextureTarget::Texture2DMultisampleArray
            | TextureTarget::ProxyTexture2DMultisampleArray => {
                Self::MTLTextureType2DMultisampleArray
            }
            TextureTarget::TextureCubeMap | TextureTarget::ProxyTextureCubeMap => Self::Cube,
            TextureTarget::TextureCubeMapArray | TextureTarget::ProxyTextureCubeMapArray => {
                Self::CubeArray
            }

            TextureTarget::TextureBuffer => Self::TextureBuffer,
            TextureTarget::Texture3D | TextureTarget::ProxyTexture3D => Self::MTLTextureType3D,
            _ => {
                panic!("invalid texture target")
            }
        }
    }
}

struct TextureLevel {
    /// Whether this level is considered "complete" (see the spec for a definition)
    complete: bool,
}
#[derive(Debug)]
pub struct Sampler {
    name: ObjectName<Self>,
    params: SamplerParams,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Anisotropy {
    NoAnisotropic,
    // INVARIANT field lies within [2, 16]
    Samples(u8),
}

impl Anisotropy {
    fn from_float(val: f32) -> Option<Self> {
        match val {
            1.0 => Some(Self::NoAnisotropic),
            // Next floating point value from 1.0
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "cast will not truncate due to range qualifier on match arm"
            )]
            //FIXME should be `f64::next(1.0)` or something
            1.000_000_000_000_000_2..=16.0 => Some(Self::Samples(val.floor() as u8)),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
/// Contains the sampling parameters for a texture
/// Note: must call [`SamplerParams::mark_dirty`] after modifying values in this struct
pub struct SamplerParams {
    /// Border color for border wrap mode
    pub(crate) border_color: [f32; 4],
    /// Depth comparison mode if depth comparison is enabled
    pub(crate) depth_compare: Option<DepthFunction>,
    /// Magnification filter
    pub(crate) mag_filter: TextureMagFilter,
    /// Minification filter and mipmap filter for mipmapped sampling
    pub(crate) min_filter: TextureMinFilter,
    /// Constant that is added to the shader-supplied LOD before clamping to [`min_lod`, `max_lod`] and sampling
    pub(crate) lod_bias: f32,
    pub(crate) max_lod: f32,
    pub(crate) min_lod: f32,
    pub(crate) max_anisotropy: Anisotropy,
    pub(crate) wrap_mode_s: TextureWrapMode,
    pub(crate) wrap_mode_t: TextureWrapMode,
    pub(crate) wrap_mode_r: TextureWrapMode,
    pub(crate) swizzle_r: TextureSwizzle,
    pub(crate) swizzle_g: TextureSwizzle,
    pub(crate) swizzle_b: TextureSwizzle,
    pub(crate) swizzle_a: TextureSwizzle,
    descriptor_cache: CloneOptionCell<Retained<MTLSamplerDescriptor>>,
}

impl SamplerParams {
    fn sampler_param(
        &mut self,
        pname: SamplerParameter,
        param: impl SrcType<f32> + SrcType<i32> + SrcType<u32>,
    ) {
        match pname {
            SamplerParameter::TextureMagFilter => todo!(),
            SamplerParameter::TextureMinFilter => todo!(),
            SamplerParameter::TextureWrapS => todo!(),
            SamplerParameter::TextureWrapT => todo!(),
            SamplerParameter::TextureWrapR => todo!(),
            SamplerParameter::TextureMinLod => todo!(),
            SamplerParameter::TextureMaxLod => todo!(),
            SamplerParameter::TextureLodBias => todo!(),
            SamplerParameter::TextureCompareMode => todo!(),
            SamplerParameter::TextureCompareFunc => todo!(),
            SamplerParameter::TextureMaxAnisotropy => todo!(),
            _ => {
                unreachable!()
            }
        }
    }

    fn sampler_desc(&self) -> Retained<MTLSamplerDescriptor> {
        if let Some(d) = self.descriptor_cache.clone_out() {
            return d;
        }
        let desc = MTLSamplerDescriptor::new();
        if [self.wrap_mode_r, self.wrap_mode_s, self.wrap_mode_t]
            .contains(&TextureWrapMode::ClampToBorder)
        {
            let border_color = match self.border_color {
                [0.0, 0.0, 0.0, v] => match v {
                    1.0 => MTLSamplerBorderColor::OpaqueBlack,
                    _ => MTLSamplerBorderColor::TransparentBlack,
                },
                [1.0, 1.0, 1.0, 1.0] => MTLSamplerBorderColor::OpaqueWhite,
                _ => panic!("tried to use an unsupported sampler border color!"),
            };
            desc.setBorderColor(border_color);
        }
        if let Some(depth_compare_func) = self.depth_compare {
            desc.setCompareFunction(depth_compare_func.into());
        }
        desc.setMagFilter(self.mag_filter.into());
        if let Anisotropy::Samples(n) = self.max_anisotropy {
            desc.setMaxAnisotropy(n as usize);
        }
        let (minification, mip) = self.min_filter.into();
        desc.setMinFilter(minification);
        desc.setMipFilter(mip);
        desc.setSAddressMode(self.wrap_mode_s.into());
        desc.setTAddressMode(self.wrap_mode_t.into());
        desc.setRAddressMode(self.wrap_mode_r.into());
        self.descriptor_cache.set(Some(desc.clone()));
        desc
    }
    fn mark_dirty(&self) {
        self.descriptor_cache.set(None);
    }
}
impl Default for SamplerParams {
    fn default() -> Self {
        Self {
            border_color: [0.0; 4],
            depth_compare: None,
            mag_filter: TextureMagFilter::Linear,
            min_filter: TextureMinFilter::NearestMipmapLinear,
            lod_bias: 0.0,
            max_lod: 1000.0,
            min_lod: -1000.0,
            max_anisotropy: Anisotropy::NoAnisotropic,
            wrap_mode_s: TextureWrapMode::Repeat,
            wrap_mode_t: TextureWrapMode::Repeat,
            wrap_mode_r: TextureWrapMode::Repeat,
            swizzle_r: TextureSwizzle::Red,
            swizzle_g: TextureSwizzle::Green,
            swizzle_b: TextureSwizzle::Blue,
            swizzle_a: TextureSwizzle::Alpha,
            descriptor_cache: CloneOptionCell::new(None),
        }
    }
}
impl From<TextureSwizzle> for MTLTextureSwizzle {
    fn from(value: TextureSwizzle) -> Self {
        match value {
            TextureSwizzle::Zero => MTLTextureSwizzle::Zero,
            TextureSwizzle::One => MTLTextureSwizzle::One,
            TextureSwizzle::Red => MTLTextureSwizzle::Red,
            TextureSwizzle::Green => MTLTextureSwizzle::Green,
            TextureSwizzle::Blue => MTLTextureSwizzle::Blue,
            TextureSwizzle::Alpha => MTLTextureSwizzle::Alpha,
        }
    }
}
impl From<TextureWrapMode> for MTLSamplerAddressMode {
    fn from(value: TextureWrapMode) -> Self {
        match value {
            TextureWrapMode::Repeat => MTLSamplerAddressMode::Repeat,
            TextureWrapMode::MirroredRepeat => MTLSamplerAddressMode::MirrorRepeat,
            TextureWrapMode::ClampToEdge => MTLSamplerAddressMode::ClampToEdge,
            TextureWrapMode::ClampToBorder => MTLSamplerAddressMode::ClampToBorderColor,
        }
    }
}
impl From<TextureMagFilter> for MTLSamplerMinMagFilter {
    fn from(value: TextureMagFilter) -> Self {
        match value {
            TextureMagFilter::Nearest => Self::Nearest,
            TextureMagFilter::Linear => Self::Linear,
        }
    }
}
impl From<TextureMinFilter> for (MTLSamplerMinMagFilter, MTLSamplerMipFilter) {
    fn from(value: TextureMinFilter) -> Self {
        use objc2_metal::{MTLSamplerMinMagFilter as MinMag, MTLSamplerMipFilter as Mip};
        match value {
            TextureMinFilter::Nearest => (MinMag::Nearest, Mip::NotMipmapped),
            TextureMinFilter::Linear => (MinMag::Linear, Mip::NotMipmapped),
            TextureMinFilter::NearestMipmapNearest => (MinMag::Nearest, Mip::Nearest),
            TextureMinFilter::LinearMipmapNearest => (MinMag::Linear, Mip::Nearest),
            TextureMinFilter::NearestMipmapLinear => (MinMag::Nearest, Mip::Linear),
            TextureMinFilter::LinearMipmapLinear => (MinMag::Linear, Mip::Linear),
        }
    }
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
/// When user CPU code requests or supplies textures, they specify the format with this pair, as opposed to using an [`InternalFormat`]
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
struct CloneOptionCell<T> {
    inner: Cell<Option<T>>,
}
impl<T: Clone> Clone for CloneOptionCell<T> {
    fn clone(&self) -> Self {
        let v = self.inner.take();
        self.inner.set(v.clone());
        Self {
            inner: Cell::new(v),
        }
    }
}
impl<T: Clone + Debug> Debug for CloneOptionCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloneOptionCell")
            .field("inner", &self.clone_out())
            .finish()
    }
}
impl<T> CloneOptionCell<T> {
    fn clone_out(&self) -> Option<T>
    where
        T: Clone,
    {
        let out = self.inner.take();
        self.inner.set(out.clone());
        out
    }
    fn set(&self, val: Option<T>) {
        self.inner.set(val);
    }
    fn new(val: Option<T>) -> Self {
        Self {
            inner: Cell::new(val),
        }
    }
}
