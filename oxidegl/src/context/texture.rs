use std::{cell::Cell, fmt::Debug, num::NonZeroU32};

use objc2::rc::Retained;
use objc2_metal::{
    MTLSamplerAddressMode, MTLSamplerBorderColor, MTLSamplerDescriptor, MTLSamplerMinMagFilter,
    MTLSamplerMipFilter, MTLTexture, MTLTextureSwizzle, MTLTextureType,
};

use crate::{
    dispatch::conversions::{GLenumExt, SrcType},
    enums::{
        DepthFunction, InternalFormat, SamplerParameter, TextureMagFilter, TextureMinFilter,
        TextureSwizzle, TextureTarget, TextureWrapMode,
    },
    util::ProtoObjRef,
};

use super::{debug::gl_err, error::GlFallible, gl_object::ObjectName};

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
            TextureTarget::Texture1D | TextureTarget::ProxyTexture1D => Self::Type1D,
            TextureTarget::Texture1DArray | TextureTarget::ProxyTexture1DArray => Self::Type1DArray,
            TextureTarget::Renderbuffer
            | TextureTarget::TextureRectangle
            | TextureTarget::ProxyTextureRectangle
            | TextureTarget::Texture2D
            | TextureTarget::ProxyTexture2D => Self::Type2D,

            TextureTarget::Texture2DArray | TextureTarget::ProxyTexture2DArray => Self::Type2DArray,
            TextureTarget::Texture2DMultisample | TextureTarget::ProxyTexture2DMultisample => {
                Self::Type2DMultisample
            }
            TextureTarget::Texture2DMultisampleArray
            | TextureTarget::ProxyTexture2DMultisampleArray => Self::Type2DMultisampleArray,
            TextureTarget::TextureCubeMap | TextureTarget::ProxyTextureCubeMap => Self::TypeCube,
            TextureTarget::TextureCubeMapArray | TextureTarget::ProxyTextureCubeMapArray => {
                Self::TypeCubeArray
            }

            TextureTarget::TextureBuffer => Self::TypeTextureBuffer,
            TextureTarget::Texture3D | TextureTarget::ProxyTexture3D => Self::Type3D,
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
        // Next floating point value from 2.0
        const VAL: f32 = 2.0_f32.next_up();
        match val {
            1.0..2.0 => Some(Self::NoAnisotropic),
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "cast will not truncate due to range of pattern"
            )]
            VAL..=16.0 => Some(Self::Samples(val.floor() as u8)),
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
    descriptor_cache: CloneOptionCell<Retained<MTLSamplerDescriptor>>,
}

impl SamplerParams {
    fn sampler_param(
        &mut self,
        pname: SamplerParameter,
        param: impl SrcType<f32> + SrcType<i32> + SrcType<u32>,
    ) -> GlFallible {
        match pname {
            SamplerParameter::TextureMagFilter => {
                self.mag_filter = param.try_into_enum()?;
            }
            SamplerParameter::TextureMinFilter => {
                self.min_filter = param.try_into_enum()?;
            }
            SamplerParameter::TextureWrapS => {
                self.wrap_mode_s = param.try_into_enum()?;
            }
            SamplerParameter::TextureWrapT => {
                self.wrap_mode_t = param.try_into_enum()?;
            }
            SamplerParameter::TextureWrapR => {
                self.wrap_mode_r = param.try_into_enum()?;
            }
            SamplerParameter::TextureMinLod => todo!(),
            SamplerParameter::TextureMaxLod => todo!(),
            SamplerParameter::TextureLodBias => todo!(),
            SamplerParameter::TextureCompareMode => todo!(),
            SamplerParameter::TextureCompareFunc => todo!(),
            SamplerParameter::TextureMaxAnisotropy => todo!(),
            SamplerParameter::TextureBorderColor => {
                unreachable!()
            }
        }
        Ok(())
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
                _ => {
                    gl_err!(ty: Error, "unsupported texture border color used, defaulting to opaque white");
                    MTLSamplerBorderColor::OpaqueWhite
                }
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
