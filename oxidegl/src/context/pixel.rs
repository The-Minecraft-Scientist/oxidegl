//! # Pixel conversion utilities
//! This module emulates the OpenGL pixel pipeline on the CPU (and possibly the GPU in the future), as well as providing various traits
//! and types for working with GL pixel formats.
//!
//!
//!
use std::{borrow::Cow, marker::PhantomData, mem::MaybeUninit, ops::Deref};

use half::f16;
use objc2_metal::MTLPixelFormat;

use crate::{
    context::debug::{gl_trace, gl_warn},
    enums::{InternalFormat, PixelFormat, PixelType},
};
mod internal_formats;
trait ChannelType {}
macro_rules! decl_channels {
    ($($i:ident),+) => {
        $(
            #[derive(Clone, Copy, Debug)]
            enum $i {}
            impl ChannelType for $i {}
        )+
    };
}

decl_channels!(Red, Green, Blue, Alpha, Depth, Stencil);
/// Represents a pixel format that can provide a certain type of channel information, represented as the type Channel (see above for all of the valid types)
trait ChannelSource<Type: ChannelType>: Sized + Copy {
    type Channel;
    fn channel(self) -> Self::Channel;
    fn get<T>(self) -> T
    where
        Self::Channel: ConvertChannel<T>,
    {
        self.channel().convert_channel()
    }
}

/// Performs a pixel channel format conversion as specified by OpenGL
trait ConvertChannel<T> {
    fn convert_channel(self) -> T;
}
// identity impl
impl<T> ConvertChannel<T> for T {
    fn convert_channel(self) -> T {
        self
    }
}
trait ConvertPixel<T> {
    fn convert_pixel(self) -> T;
}
/// Represents the default value for this channel according to the OpenGL pixel interchange specification
struct DefaultChannelValue<Ctype>(PhantomData<Ctype>);
impl<T> DefaultChannelValue<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! channelsource_default {
    (
        default impl $(  ($($imp:tt)*) )? ChannelSource<$ty:ty> for $struct:ty {}
    ) => {
        impl$(<$($imp)*>)? ChannelSource<$ty> for $struct {
            type Channel = DefaultChannelValue<$ty>;
            fn channel(self) -> Self::Channel {
                DefaultChannelValue::new()
            }
        }
    };
}
const fn const_arr_max<const N: usize>(arr: &[usize; N]) -> usize {
    let mut g = 0;
    let mut index = 0;
    while index < arr.len() {
        if arr[index] > g {
            g = arr[index];
        }
        index += 1;
    }
    g
}
const fn assert_covered<const N: usize>(arr: &[usize; N]) {
    let mut v = 0;
    let l = const_arr_max(arr);
    while v <= l {
        assert!(
            arr_contains(v, arr),
            "Bad simple color format definition, all array indices must be occupied"
        );
        v += 1;
    }
}
const fn arr_contains(val: usize, arr: &[usize]) -> bool {
    let mut v = 0;
    while v < arr.len() {
        if arr[v] == val {
            return true;
        }
        v += 1;
    }
    false
}
macro_rules! decl_simple_formats {
    (   $(
            $( #[doc = $doc:expr] )*
            $vis:vis struct $fmt_name:ident  {
                $( ($idx:literal : $chan:ty) ),+ default: [$($dty:ty),+]
            }
        ),+
        $(,)?
    ) => {
        $(
            $( #[doc = $doc] )*
            #[derive(Debug, Clone, Copy)]
            #[repr(C)]
            $vis struct $fmt_name<T> {
                colors: [T; const { const_arr_max( &[ $( $idx ),+ ] ) + 1 }]
            }
            impl<C, T: $(ChannelSource<$chan, Channel: ConvertChannel<C>> +)+ Copy> ConvertPixel<$fmt_name<C>> for T {

                #[inline]

                fn convert_pixel(self) -> $fmt_name<C> {
                    const __INDICES: [usize; const { [ $( $idx ),+ ].len() }] = [ $( $idx ),+ ];
                    const __ARR_LEN: usize = const { const_arr_max( &__INDICES ) + 1 };
                    const { assert_covered(&__INDICES) }

                    let mut uninit = [const { MaybeUninit::<C>::uninit() }; __ARR_LEN];
                    $(
                        uninit[$idx].write(<Self as ChannelSource<$chan>>::channel(self).convert_channel());
                    )+
                    $fmt_name {
                        colors: unsafe { crate::util::transmute_unchecked(uninit) },
                    }
                }
            }
            $(
                impl<T: Copy> ChannelSource<$chan> for $fmt_name<T> {
                    type Channel = T;
                    #[inline]
                    fn channel(self) -> Self::Channel {
                        self.colors[$idx]
                    }
                }
            )+
            $(
                channelsource_default! {
                    default impl(T: Copy) ChannelSource<$dty> for $fmt_name<T> {}
                }
            )+
        )+
    };
}

decl_simple_formats! {
    pub(crate) struct RgColorFormat { (0: Red), (1: Green) default: [Blue, Alpha, Depth, Stencil] },
    pub(crate) struct RgbColorFormat { (0: Red), (1: Green), (2: Blue) default: [Alpha, Depth, Stencil] },
    pub(crate) struct RgbaColorFormat { (0: Red), (1: Green), (2: Blue), (3: Alpha) default: [Depth, Stencil] },
    pub(crate) struct BgrColorFormat { (0: Blue), (1: Green), (2: Red) default: [Alpha, Depth, Stencil] },
    pub(crate) struct BgraColorFormat { (0: Blue), (1: Green), (2: Red), (3: Alpha) default: [Depth, Stencil] },
}

#[derive(Debug, Clone, Copy)]
struct SingleChannelFormat<T, Ctype: ChannelType> {
    value: T,
    phantom: PhantomData<Ctype>,
}
impl<T, U: ChannelType> SingleChannelFormat<T, U> {
    fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}
macro_rules! impl_single_color {
    (
        $( $prety:ident ),* | $t:ident, $( $postty:ident, )*
    ) => {
        $(
            channelsource_default! {
                default impl(T: Copy) ChannelSource<$prety> for SingleChannelFormat<T, $t> {}
            }
        )*
        $(
            channelsource_default! {
                default impl(T: Copy) ChannelSource<$postty> for SingleChannelFormat<T, $t> {}
            }
        )*
        impl<T: Copy> ChannelSource<$t> for SingleChannelFormat<T, $t> {
            type Channel = T;
            fn channel(self) -> Self::Channel {
                self.value
            }
        }
        impl_single_color! {
            $t $(, $prety )*  | $( $postty,)*
        }
    };
    ( $($pre:ident),+ | ) => {};
}
impl_single_color! {
    | Red, Green, Blue, Alpha, Depth, Stencil, // <---- load bearing comma, do not remove :)
}
impl<Ctype: ChannelType, Cs: ConvertChannel<C>, C, T: ChannelSource<Ctype, Channel = Cs>>
    ConvertPixel<SingleChannelFormat<C, Ctype>> for T
{
    fn convert_pixel(self) -> SingleChannelFormat<C, Ctype> {
        SingleChannelFormat {
            value: <Cs as ConvertChannel<C>>::convert_channel(self.channel()),
            phantom: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
struct NormalizedIntChannel<Int: GlIntegerType + Copy>(Int);

impl<Int: GlIntegerType + Copy> ConvertChannel<f16> for NormalizedIntChannel<Int> {
    fn convert_channel(self) -> f16 {
        f16::from_f32(self.0.normalized_to_float())
    }
}
impl<Int: GlIntegerType + Copy> ConvertChannel<f32> for NormalizedIntChannel<Int> {
    fn convert_channel(self) -> f32 {
        self.0.normalized_to_float()
    }
}

// Trait that generalizes over integers of all bitwidths supported as OpenGL pixel types
trait GlIntegerType: Sized + Copy {
    /// Whether this integer is signed
    const SIGNED: bool;
    /// number of bits in this integer's representation
    const BITWIDTH: u8;
    /// power of two to divide by when normalizing this value into a float
    const NORMALIZATION_FACTOR: u8 = Self::BITWIDTH - Self::SIGNED as u8;
    /// Interprets this integer as a normalized fixed-point float according the the GL spec, returning the closest 32 bit floating point value to the normalized result
    fn normalized_to_float(self) -> f32;
    /// Creates an instance of this number from a u32, truncating the most significant bits to size
    fn from_bits(bits: u32) -> Self;
    /// bitcasts this number into (the low bits of) a u32
    fn to_bits(self) -> u32;
}

fn convert_normalized_int<IntInitial: GlIntegerType + Copy, IntFinal: GlIntegerType + Copy>(
    val: NormalizedIntChannel<IntInitial>,
) -> NormalizedIntChannel<IntFinal> {
    let mut int: u32 = val.0.to_bits();
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    if IntInitial::SIGNED != IntFinal::SIGNED {
        int = match IntInitial::SIGNED {
            // clamp negative values to 0 for signed -> unsigned
            true => (int as i32).clamp(0, i32::MAX) as u32,
            false => int.clamp(0, i32::MAX as u32),
        }
    }
    // truncates to size
    NormalizedIntChannel(IntFinal::from_bits(int))
}

macro_rules! impl_gl_integer {
    ( $( ($t:ident, $signed:expr, $width:expr, $ftype:ty) ),+ ) => {
        $(
            impl GlIntegerType for $t {
                const SIGNED: bool = $signed;
                const BITWIDTH: u8 = $width;
                #[allow(clippy::cast_possible_truncation)]
                fn normalized_to_float(self) -> f32 {
                    let f1 = <$ftype>::from(self);
                    let factor = <$ftype>::from(2u8).powi(i32::from(Self::NORMALIZATION_FACTOR) );
                    (f1 / factor) as f32
                }
                fn from_bits(bits: u32) -> Self {
                    const { assert!((Self::BITWIDTH / 8) as usize <= 4, "Bad GlInteger implementation") }
                    // Safety: checked by const assertion above
                    Self::from_le_bytes(unsafe { *bits.to_le_bytes().first_chunk::<{ (Self::BITWIDTH / 8) as usize }>().unwrap_unchecked() })
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_lossless, clippy::cast_sign_loss, reason = "truncation is desired here")]
                fn to_bits(self) -> u32 {
                    self as u32
                }
            }
        )+
        impl_gl_integer! {
            convertchannel: | $($t,)+
        }
    };
    ( convertchannel: $( $prety:ident ),* | $t:ident, $( $postty:ident, )* ) => {
        $(
            impl ConvertChannel<NormalizedIntChannel<$prety>> for NormalizedIntChannel<$t> {
                fn convert_channel(self) -> NormalizedIntChannel<$prety> {
                    convert_normalized_int::<$t, $prety>(self)
                }
            }
        )*
        $(
            impl ConvertChannel<NormalizedIntChannel<$postty>> for NormalizedIntChannel<$t> {
                fn convert_channel(self) -> NormalizedIntChannel<$postty> {
                    convert_normalized_int::<$t, $postty>(self)
                }
            }
        )*
        impl_gl_integer! {
            convertchannel: $t $(, $prety )*  | $( $postty,)*
        }
    };
    ( convertchannel: $($pre:ident),+ | ) => {};
}
impl_gl_integer! {
    (u8, false, 8, f32),
    (u16, false, 16, f32),
    (u32, false, 32, f64),
    (i8, true, 8, f32),
    (i16, true, 16, f32),
    (i32, true, 32, f64)
}

unsafe fn convert_pixel_array<T: Copy + ConvertPixel<Out>, Out: Copy>(
    swap_bytes: bool,
    buf: &[T],
) -> Box<[Out]> {
    let mut out = Vec::with_capacity(buf.len());
    for i in buf.iter().copied() {
        out.push(i.convert_pixel());
    }
    out.into_boxed_slice()
}
//fn internal_format_ispatch<T>(pix_buf: )
fn convert_dispatch_test(buf: *const u8, fmt: GlPixelTypeFormat, dst: InternalFormat) {
    // special type handling
    match fmt.ty {
        // "normal" pixel types
        PixelType::Byte => match fmt.fmt {
            PixelFormat::DepthComponent => todo!(),
            PixelFormat::Red => todo!(),
            PixelFormat::Green => todo!(),
            PixelFormat::Blue => todo!(),
            PixelFormat::Alpha => todo!(),
            PixelFormat::Rgb => todo!(),
            PixelFormat::Rgba => todo!(),
            PixelFormat::Bgr => todo!(),
            PixelFormat::Bgra => todo!(),
            PixelFormat::RedInteger => todo!(),
            PixelFormat::GreenInteger => todo!(),
            PixelFormat::BlueInteger => todo!(),
            PixelFormat::RgbInteger => todo!(),
            PixelFormat::RgbaInteger => todo!(),
            PixelFormat::BgrInteger => todo!(),
            PixelFormat::BgraInteger => todo!(),
            PixelFormat::DepthStencil => todo!(),
            PixelFormat::Rg => todo!(),
            PixelFormat::RgInteger => todo!(),
            PixelFormat::StencilIndex => todo!(),
            _ => unreachable!(),
        },
        PixelType::UnsignedByte => todo!(),
        PixelType::Short => todo!(),
        PixelType::UnsignedShort => todo!(),
        PixelType::Int => todo!(),
        PixelType::UnsignedInt => todo!(),
        PixelType::Float => todo!(),
        PixelType::HalfFloat => todo!(),

        PixelType::UnsignedByte332 => todo!(),
        PixelType::UnsignedShort4444 => todo!(),
        PixelType::UnsignedShort5551 => todo!(),
        PixelType::UnsignedInt8888 => todo!(),
        PixelType::UnsignedInt1010102 => todo!(),
        PixelType::UnsignedByte233Rev => todo!(),
        PixelType::UnsignedShort565 => todo!(),
        PixelType::UnsignedShort565Rev => todo!(),
        PixelType::UnsignedShort4444Rev => todo!(),
        PixelType::UnsignedShort1555Rev => todo!(),
        PixelType::UnsignedInt8888Rev => todo!(),
        PixelType::UnsignedInt2101010Rev => todo!(),
        PixelType::UnsignedInt5999Rev => todo!(),
        PixelType::Float32UnsignedInt248Rev => todo!(),
        PixelType::UnsignedInt248 => todo!(),
        PixelType::UnsignedInt10F11F11FRev => todo!(),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct BinaryCompatibleFormat(InternalFormat);

#[derive(Debug, Copy, Clone)]
/// A combination of a [`PixelType`] and a [`PixelFormat`], used to specify input and output formats.
/// When user CPU code requests or supplies textures, they specify the format with this pair, as opposed to using an [`InternalFormat`]
pub struct GlPixelTypeFormat {
    ty: PixelType,
    fmt: PixelFormat,
}

impl GlPixelTypeFormat {
    pub fn new(ty: PixelType, fmt: PixelFormat) -> Self {
        Self { ty, fmt }
    }
    /// Returns the `InternalFormat` that matches this type format pair, if any. No ABI compatability guarantees are made about the format of this [`GlPixelTypeFormat`] and the returned [`InternalFormat`].
    pub(crate) fn equivalent_internal_format(self) -> Option<InternalFormat> {
        use InternalFormat as IF;
        use PixelFormat::{
            DepthComponent, DepthStencil, Red, RedInteger, Rg, RgInteger, Rgba, RgbaInteger,
            StencilIndex,
        };

        gl_trace!(
            "attempting to lower (type {:?}, fmt {:?}) tuple to a metal pixel format",
            self.ty,
            self.fmt
        );
        // 'none: {
        //     return Some(match self.ty {
        //         PixelType::Byte => match self.fmt {
        //             Red => IF::R8Snorm,
        //             Rg => IF::Rg8Snorm,
        //             Rgba => IF::Rgb8,
        //             RedInteger => IF::R8i,
        //             RgInteger => IF::Rg8i,
        //             RgbaInteger => IF::Rgba8i,
        //             StencilIndex => IF::StencilIndex8,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedByte => match self.fmt {
        //             Red => IF::R8,
        //             Rg => IF::Rg8,
        //             Rgba => IF::Rgba8,
        //             RedInteger => IF::R8ui,
        //             RgInteger => IF::Rg8ui,
        //             RgbaInteger => IF::Rgba8ui,
        //             StencilIndex => IF::StencilIndex8,
        //             _ => break 'none,
        //         },
        //         PixelType::Short => match self.fmt {
        //             Red => IF::R16Snorm,
        //             Rg => IF::Rg16Snorm,
        //             Rgba => IF::Rgba16Snorm,
        //             RedInteger => IF::R16i,
        //             RgInteger => IF::Rg16i,
        //             RgbaInteger => IF::Rgba16i,

        //             DepthComponent => IF::DepthComponent16,
        //             StencilIndex => IF::StencilIndex16,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedShort => match self.fmt {
        //             Red => IF::R16,
        //             Rg => IF::Rg16,
        //             Rgba => IF::Rgba16,
        //             RedInteger => IF::R16ui,
        //             RgInteger => IF::Rg16ui,
        //             RgbaInteger => IF::Rgba16ui,

        //             DepthComponent => IF::DepthComponent16,
        //             StencilIndex => IF::StencilIndex16,
        //             _ => break 'none,
        //         },

        //         // TODO: emulate GL normalization behavior for (U)Int,
        //         // those formats are not supported for now
        //         PixelType::Int => match self.fmt {
        //             // Red => IF::R32ui,
        //             // Rg => IF::RG32Snorm,
        //             // Rgba => IF::Rgba32Snorm,
        //             RedInteger => IF::R32i,
        //             RgInteger => IF::Rg32i,
        //             RgbaInteger => IF::Rgba32i,

        //             DepthComponent => IF::DepthComponent32,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedInt => match self.fmt {
        //             // Red => IF::R32Unorm,
        //             // Rg => IF::RG32Unorm,
        //             // Rgba => IF::Rgba32Unorm,
        //             RedInteger => IF::R32ui,
        //             RgInteger => IF::Rg32ui,
        //             RgbaInteger => IF::Rgba32ui,

        //             DepthComponent => IF::DepthComponent32,
        //             _ => break 'none,
        //         },
        //         PixelType::Float => match self.fmt {
        //             Red => IF::R32f,
        //             Rg => IF::Rg32f,
        //             Rgba => IF::Rgba32f,
        //             DepthComponent => IF::DepthComponent32f,
        //             DepthStencil => IF::Depth24Stencil8,
        //             _ => break 'none,
        //         },
        //         PixelType::HalfFloat => match self.fmt {
        //             Red => IF::R16f,
        //             Rg => IF::Rg16f,
        //             Rgba => IF::Rgba16f,
        //             _ => break 'none,
        //         },
        //         // special-purpose formats
        //         PixelType::UnsignedInt10F11F11FRev => todo!(),
        //         PixelType::UnsignedShort565 => IF::Rgb565,
        //         PixelType::UnsignedInt8888 => IF::Rgba8,
        //         PixelType::UnsignedInt1010102 => IF::Rgb10A2,
        //         PixelType::UnsignedInt2101010Rev => IF::BGR10A2Unorm,
        //         PixelType::UnsignedShort4444Rev => IF::ABGR4Unorm,
        //         PixelType::UnsignedShort1555Rev => IF::A1BGR5Unorm,

        //         PixelType::UnsignedInt8888Rev => break 'none,
        //         PixelType::UnsignedByte332 => todo!(),
        //         PixelType::UnsignedShort4444 => todo!(),
        //         PixelType::UnsignedShort5551 => todo!(),
        //         PixelType::UnsignedByte233Rev => todo!(),
        //         PixelType::UnsignedInt5999Rev => todo!(),
        //         PixelType::Float32UnsignedInt248Rev => todo!(),
        //         PixelType::UnsignedInt248 => todo!(),
        //         PixelType::UnsignedShort565Rev => todo!(),
        //     });
        // }
        todo!();
        gl_warn!(
            ty: Performance,
            "failed to lower format+type tuple (type {:?}, fmt {:?}) to an internal format.
            OxideGL will attempt to perform conversions but good performance is not guaranteed",
            self.ty,
            self.fmt
        );
        None
    }
}
