// Heavily inspired by code from wgpu's metal backend
// (see https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/mod.rs#L201, https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/adapter.rs#L842)

use objc2_foundation::{NSOperatingSystemVersion, NSProcessInfo};
use objc2_metal::{MTLDevice, MTLGPUFamily, MTLLanguageVersion, MTLPixelFormat};

use crate::util::{ProtoObjRef, bitflag_bits};

pub(crate) enum PlatformType {
    Ios,
    MacOs,
}
pub(crate) struct OsVersion {
    ty: PlatformType,
    inner: NSOperatingSystemVersion,
}

impl OsVersion {
    #[inline]
    fn at_least(&self, macos: (isize, isize), ios: (isize, isize)) -> bool {
        let v = match self.ty {
            PlatformType::Ios => ios,
            PlatformType::MacOs => macos,
        };
        self.inner.majorVersion > v.0
            || (self.inner.majorVersion == v.0 && v.1 >= self.inner.minorVersion)
    }
}

bitflag_bits! {
    pub(crate) struct TextureCaps: u8 bits: {
        /// The device can filter a texture with this pixel format during sampling
        FILTER: 0,
        /// The device can write new pixel values to a texture with this pixel format
        WRITE: 1,
        /// The device can use a texture with this pixel format as a color render target
        COLOR: 2,
        /// The device can blend a texture with this pixel format
        BLEND: 3,
        /// The device can use a texture with this pixel format as a destination for multisample antialiased (MSAA) data
        MSAA: 4,
        /// The device supports sparse-texture allocations for textures with this pixel format
        SPARSE: 5,
        /// The device can use a texture with this pixel format as a source for multisample antialias (MSAA) resolve operations
        RESOLVE: 6,
        /// The device can perform atomic operations on textures with this pixel format
        ATOMIC: 7
    }
}

impl TextureCaps {
    pub(crate) const ALL: Self = Self::from_bits(0x7F).expect("bitfield should have been correct");
    pub(crate) const NONE: Self = Self::from_bits(0).expect("bitfield should have been correct");
}
#[derive(Debug, Clone)]
pub(crate) struct MetalProperties {
    /// Metal Shading Language Version
    // INVARIANT: always Metal 2.2 or higher
    msl_version: MTLLanguageVersion,
    /// Which GPU families the current device supports
    families: Families,
    /// Which multisample counts this device supports for multisampled textures/rendering
    sample_counts: SupportedSampleCounts,
    /// The maximum amount of supported vertex amplifications for multiview/layered rendering
    max_vertex_amp: u32,
    max_texture_arguments: u32,
    apple7_8_supports_bc: bool,
}

impl MetalProperties {
    pub(crate) fn new(device: &ProtoObjRef<dyn MTLDevice>) -> Self {
        let version = NSProcessInfo::processInfo().operatingSystemVersion();
        // common denominator of all Mac `MTLDevice`s
        #[allow(deprecated)]
        let is_mac = device.supportsFeatureSet(objc2_metal::MTLFeatureSet::macOS_GPUFamily1_v1);

        let version = OsVersion {
            ty: if is_mac {
                PlatformType::MacOs
            } else {
                PlatformType::Ios
            },
            inner: version,
        };
        assert!(
            version.at_least((10, 15), (13, 0)),
            "OxideGL requires Metal 2.2 or above. Please update your OS version"
        );

        let mut max_vertex_amp = 2u32;
        while device.supportsVertexAmplificationCount(max_vertex_amp as usize) {
            max_vertex_amp += 1;
        }
        max_vertex_amp -= 1;
        let families = get_supported_families(device);
        Self {
            msl_version: get_msl_version(&version),
            sample_counts: get_supported_sample_counts(device),
            families,
            max_vertex_amp,
            max_texture_arguments: if families.intersects(
                Families::MAC2
                    | Families::APPLE9
                    | Families::APPLE8
                    | Families::APPLE7
                    | Families::APPLE6,
            ) {
                128
            } else if families.intersects(Families::APPLE5 | Families::APPLE4) {
                96
            } else {
                31
            },
            apple7_8_supports_bc: device.supportsBCTextureCompression(),
        }
    }
    fn get_texture_caps(&self, format: MTLPixelFormat) -> Option<TextureCaps> {
        use MTLPixelFormat as MF;

        let device_families = self.families;
        let sparse = if device_families
            .intersects(Families::APPLE6 | Families::APPLE7 | Families::APPLE8 | Families::APPLE9)
        {
            TextureCaps::SPARSE
        } else {
            TextureCaps::NONE
        };

        match format {
            MF::R8Unorm
            | MF::R8Snorm
            | MF::R16Float
            | MF::RG8Unorm
            | MF::RG8Snorm
            | MF::RG16Float
            | MF::RGBA8Unorm
            | MF::RGBA8Snorm
            | MF::BGRA8Unorm
            | MF::BGR10A2Unorm
            | MF::RGBA16Float => Some(TextureCaps::ALL),

            MF::R8Uint
            | MF::R8Sint
            | MF::R16Uint
            | MF::R16Sint
            | MF::RG8Uint
            | MF::RG8Sint
            | MF::RG16Sint
            | MF::RG16Uint
            | MF::RGBA8Uint
            | MF::RGBA16Sint
            | MF::RGBA16Uint
            | MF::RGBA8Sint => {
                Some(TextureCaps::WRITE | TextureCaps::COLOR | TextureCaps::MSAA | sparse)
            }

            MF::A8Unorm => Some(if device_families == Families::APPLE2 {
                TextureCaps::FILTER
            } else {
                TextureCaps::ALL
            }),
            // non-Mac2 only
            MF::R8Unorm_sRGB | MF::RG8Unorm_sRGB => {
                if device_families == Families::MAC2 {
                    None
                } else {
                    Some(TextureCaps::ALL)
                }
            }

            MF::R16Unorm
            | MF::R16Snorm
            | MF::RG16Unorm
            | MF::RG16Snorm
            | MF::RGBA16Unorm
            | MF::RGBA16Snorm => Some(if device_families.intersects(Families::MAC2) {
                TextureCaps::ALL
            } else {
                TextureCaps::FILTER
                    | TextureCaps::WRITE
                    | TextureCaps::COLOR
                    | TextureCaps::MSAA
                    | TextureCaps::BLEND
                    | sparse
            }),
            // packed 16-bit formats
            MF::B5G6R5Unorm | MF::A1BGR5Unorm | MF::ABGR4Unorm | MF::BGR5A1Unorm => {
                if device_families == Families::MAC2 {
                    None
                } else {
                    Some(
                        TextureCaps::FILTER
                            | TextureCaps::COLOR
                            | TextureCaps::MSAA
                            | TextureCaps::RESOLVE
                            | TextureCaps::BLEND
                            | sparse,
                    )
                }
            }
            MF::R32Uint | MF::R32Sint => {
                let mut r = TextureCaps::WRITE | TextureCaps::COLOR | sparse;

                if device_families.contains(Families::MAC2) {
                    r |= TextureCaps::ATOMIC | TextureCaps::MSAA;
                }
                if device_families.intersects(
                    Families::APPLE6 | Families::APPLE7 | Families::APPLE8 | Families::APPLE9,
                ) {
                    r |= TextureCaps::ATOMIC;
                }
                Some(r)
            }
            MF::R32Float => Some(
                if device_families.intersects(Families::MAC2 | Families::APPLE9) {
                    TextureCaps::ALL
                } else {
                    TextureCaps::WRITE
                        | TextureCaps::COLOR
                        | TextureCaps::MSAA
                        | TextureCaps::BLEND
                        | sparse
                },
            ),
            MF::RGBA8Unorm_sRGB | MF::BGRA8Unorm_sRGB => {
                Some(if device_families == Families::MAC2 {
                    TextureCaps::FILTER
                        | TextureCaps::COLOR
                        | TextureCaps::MSAA
                        | TextureCaps::RESOLVE
                        | TextureCaps::BLEND
                } else {
                    TextureCaps::ALL
                })
            }
            MF::RG11B10Float | MF::RGB10A2Unorm => Some(if device_families == Families::APPLE2 {
                TextureCaps::FILTER
                    | TextureCaps::COLOR
                    | TextureCaps::MSAA
                    | TextureCaps::RESOLVE
                    | TextureCaps::BLEND
            } else {
                TextureCaps::ALL
            }),
            MF::RGB9E5Float => Some(if device_families == Families::APPLE2 {
                TextureCaps::FILTER
                    | TextureCaps::COLOR
                    | TextureCaps::MSAA
                    | TextureCaps::RESOLVE
                    | TextureCaps::BLEND
            } else if device_families == Families::MAC2 {
                TextureCaps::FILTER
            } else {
                TextureCaps::ALL
            }),
            MF::RGB10A2Uint => Some(
                sparse
                    | if device_families == Families::APPLE2 {
                        TextureCaps::COLOR | TextureCaps::MSAA
                    } else {
                        TextureCaps::WRITE | TextureCaps::COLOR | TextureCaps::MSAA
                    },
            ),
            MF::RG32Uint | MF::RG32Sint => {
                let mut r = sparse | TextureCaps::WRITE | TextureCaps::COLOR;
                if device_families.intersects(
                    Families::APPLE7 | Families::APPLE8 | Families::APPLE9 | Families::MAC2,
                ) {
                    r |= TextureCaps::MSAA;
                }
                if device_families.intersects(Families::APPLE8 | Families::APPLE9) {
                    r |= TextureCaps::ATOMIC;
                }
                Some(r)
            }
            MF::RG32Float => Some(
                if device_families.intersects(Families::APPLE9 | Families::MAC2) {
                    TextureCaps::ALL
                } else {
                    let r = sparse | TextureCaps::WRITE | TextureCaps::COLOR | TextureCaps::BLEND;
                    if device_families.intersects(Families::APPLE7 | Families::APPLE8) {
                        r | TextureCaps::MSAA
                    } else {
                        r
                    }
                },
            ),

            MF::RGBA32Uint | MF::RGBA32Sint => {
                let v = TextureCaps::WRITE | TextureCaps::COLOR | TextureCaps::SPARSE;
                Some(if device_families.intersects(Families::MAC2) {
                    v | TextureCaps::MSAA
                } else {
                    v
                })
            }
            MF::RGBA32Float => {
                let v = TextureCaps::WRITE | TextureCaps::COLOR | sparse;
                Some(
                    if device_families.intersects(Families::APPLE9 | Families::MAC2) {
                        TextureCaps::ALL
                    } else if device_families.intersects(Families::APPLE7 | Families::APPLE8) {
                        v | TextureCaps::MSAA
                    } else {
                        v
                    },
                )
            }
            MF::BC1_RGBA
            | MF::BC1_RGBA_sRGB
            | MF::BC2_RGBA
            | MF::BC2_RGBA_sRGB
            | MF::BC3_RGBA
            | MF::BC3_RGBA_sRGB
            | MF::BC4_RSnorm
            | MF::BC4_RUnorm
            | MF::BC5_RGSnorm
            | MF::BC5_RGUnorm
            | MF::BC6H_RGBFloat
            | MF::BC6H_RGBUfloat
            | MF::BC7_RGBAUnorm
            | MF::BC7_RGBAUnorm_sRGB => {
                if device_families.intersects(Families::MAC2) {
                    Some(TextureCaps::FILTER)
                } else if device_families.intersects(Families::APPLE9) || self.apple7_8_supports_bc
                {
                    Some(TextureCaps::FILTER | TextureCaps::SPARSE)
                } else {
                    None
                }
            }
            _ => panic!("invalid pixel format"),
        }
        // TODO 64 bit formats, 128 bit formats and
    }
}
fn get_msl_version(version: &OsVersion) -> MTLLanguageVersion {
    if version.at_least((15, 0), (18, 0)) {
        MTLLanguageVersion::Version3_2
    } else if version.at_least((14, 0), (17, 0)) {
        MTLLanguageVersion::Version3_1
    } else if version.at_least((13, 0), (16, 0)) {
        MTLLanguageVersion::Version3_0
    } else if version.at_least((12, 0), (15, 0)) {
        MTLLanguageVersion::Version2_4
    } else if version.at_least((11, 0), (14, 0)) {
        MTLLanguageVersion::Version2_3
    } else if version.at_least((10, 15), (13, 0)) {
        MTLLanguageVersion::Version2_2
    } else {
        unreachable!()
    }
}
bitflag_bits! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) struct Families: u32 bits: {
        APPLE1: 0,
        APPLE2: 1,
        APPLE3: 2,
        APPLE4: 3,
        APPLE5: 4,
        APPLE6: 5,
        APPLE7: 6,
        APPLE8: 7,
        APPLE9: 8,
        MAC1: 9,
        MAC2: 10,
        COMMON1: 11,
        COMMON2: 12,
        COMMON3: 13,
        METAL3: 14
    }
}
impl Families {
    fn supports_any(self, other: Families) -> bool {
        self.intersects(other)
    }
}
fn get_supported_families(device: &ProtoObjRef<dyn MTLDevice>) -> Families {
    let mut supported_families = Families::empty();
    if device.supportsFamily(MTLGPUFamily::Apple1) {
        supported_families |= Families::APPLE1;
    }
    if device.supportsFamily(MTLGPUFamily::Apple2) {
        supported_families |= Families::APPLE2;
    }
    if device.supportsFamily(MTLGPUFamily::Apple3) {
        supported_families |= Families::APPLE3;
    }
    if device.supportsFamily(MTLGPUFamily::Apple4) {
        supported_families |= Families::APPLE4;
    }
    if device.supportsFamily(MTLGPUFamily::Apple5) {
        supported_families |= Families::APPLE5;
    }
    if device.supportsFamily(MTLGPUFamily::Apple6) {
        supported_families |= Families::APPLE6;
    }
    if device.supportsFamily(MTLGPUFamily::Apple7) {
        supported_families |= Families::APPLE7;
    }
    if device.supportsFamily(MTLGPUFamily::Apple8) {
        supported_families |= Families::APPLE8;
    }
    if device.supportsFamily(MTLGPUFamily::Apple9) {
        supported_families |= Families::APPLE9;
    }
    #[allow(deprecated)]
    if device.supportsFamily(MTLGPUFamily::Mac1) {
        supported_families |= Families::MAC1;
    }
    if device.supportsFamily(MTLGPUFamily::Mac2) {
        supported_families |= Families::MAC2;
    }
    if device.supportsFamily(MTLGPUFamily::Common1) {
        supported_families |= Families::COMMON1;
    }
    if device.supportsFamily(MTLGPUFamily::Common2) {
        supported_families |= Families::COMMON2;
    }
    if device.supportsFamily(MTLGPUFamily::Common3) {
        supported_families |= Families::COMMON3;
    }
    if device.supportsFamily(MTLGPUFamily::Metal3) {
        supported_families |= Families::METAL3;
    }
    supported_families
}

bitflag_bits! {
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct SupportedSampleCounts: u8 bits: {
        SAMPLE_X2: 0,
        SAMPLE_X4: 1,
        SAMPLE_X8: 2,
        SAMPLE_X16: 3
    }
}
impl Default for SupportedSampleCounts {
    fn default() -> Self {
        // All devices support single sampling (implicitly) and 4x multisample
        Self::SAMPLE_X4
    }
}
fn get_supported_sample_counts(device: &ProtoObjRef<dyn MTLDevice>) -> SupportedSampleCounts {
    let mut sample_counts = SupportedSampleCounts::default();
    if device.supportsTextureSampleCount(2) {
        sample_counts |= SupportedSampleCounts::SAMPLE_X2;
    }
    if device.supportsTextureSampleCount(8) {
        sample_counts |= SupportedSampleCounts::SAMPLE_X8;
    }
    if device.supportsTextureSampleCount(16) {
        sample_counts |= SupportedSampleCounts::SAMPLE_X16;
    }
    sample_counts
}
