use std::ptr;

use super::gltypes::GLenum;

pub trait UnsafeFromGLenum {
    unsafe fn unsafe_from_gl_enum(val: GLenum) -> Self;
}
/// Helper trait to convert from "raw" `GLenums` to wrappers around subsets of those that are valid for certain functions
pub trait GLenumExt<T> {
    unsafe fn into_enum(val: Self) -> T;
}

impl<T> GLenumExt<T> for GLenum
where
    T: UnsafeFromGLenum,
{
    unsafe fn into_enum(val: u32) -> T {
        unsafe { T::unsafe_from_gl_enum(val) }
    }
}

/// Trait that describes a type that may be returned from a gl*Get function.
pub(crate) trait GlDstType: Copy {
    fn from_uint(val: u32) -> Self;
    fn from_ulong(val: u64) -> Self;
    fn from_int(val: i32) -> Self;
    fn from_long(val: i64) -> Self;
    fn from_float(val: f32) -> Self;
    fn from_double(val: f64) -> Self;
    fn from_bool(val: bool) -> Self;
}
/// Trait that describes how to convert a given type into a value that may be returned
pub trait SrcType<Dst: GlDstType>: Copy {
    fn cast(self) -> Dst;
}
/// Trait that allows converting a value to an inferred Dst type and unsafely writing it to foreign memory
pub trait StateQueryWrite<Dst: GlDstType> {
    type It: SrcType<Dst>;
    unsafe fn write_out(&self, idx: Option<u32>, ptr: *mut Dst);
}

impl<It: SrcType<Dst>, Dst: GlDstType> StateQueryWrite<Dst> for [It] {
    type It = It;
    unsafe fn write_out(&self, idx: Option<u32>, mut ptr: *mut Dst) {
        for item in self {
            // Safety: Caller ensures that self is the correct length for the allocation being written to,
            // and that Dst is the correct type for the allocation being written to
            unsafe { ptr::write(ptr, item.cast()) }
            unsafe { ptr = ptr.add(1) }
        }
    }
}

impl<It: SrcType<Dst>, Dst: GlDstType> StateQueryWrite<Dst> for It {
    type It = Self;
    unsafe fn write_out(&self, idx: Option<u32>, ptr: *mut Dst) {
        debug_assert!(
            idx.is_none() || idx.map(|v| v == 0) == Some(true),
            "Tried to write outside the bounds of a single item"
        );
        // Safety: caller ensures that Dst is the correct type for the allocation being written to
        unsafe { ptr::write(ptr, self.cast()) }
    }
}

//TODO: wrap all of this in a macro
impl<Dst: GlDstType> SrcType<Dst> for u32 {
    fn cast(self) -> Dst {
        Dst::from_uint(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for i32 {
    fn cast(self) -> Dst {
        Dst::from_int(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for u64 {
    fn cast(self) -> Dst {
        Dst::from_ulong(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for i64 {
    fn cast(self) -> Dst {
        Dst::from_long(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for f32 {
    fn cast(self) -> Dst {
        Dst::from_float(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for f64 {
    fn cast(self) -> Dst {
        Dst::from_double(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for bool {
    fn cast(self) -> Dst {
        Dst::from_bool(self)
    }
}
impl GlDstType for u32 {
    fn from_int(val: i32) -> Self {
        val.unsigned_abs()
    }
    fn from_long(val: i64) -> Self {
        val.unsigned_abs() as Self
    }
    fn from_uint(val: u32) -> Self {
        val
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }

    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    fn from_bool(val: bool) -> Self {
        val as Self
    }
}

impl GlDstType for u64 {
    fn from_int(val: i32) -> Self {
        val.unsigned_abs() as Self
    }
    fn from_long(val: i64) -> Self {
        val.unsigned_abs()
    }
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    fn from_ulong(val: u64) -> Self {
        val
    }

    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    fn from_bool(val: bool) -> Self {
        val as Self
    }
}
impl GlDstType for i32 {
    fn from_int(val: i32) -> Self {
        val
    }
    fn from_long(val: i64) -> Self {
        val as Self
    }
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }

    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    fn from_bool(val: bool) -> Self {
        val as Self
    }
}

impl GlDstType for i64 {
    fn from_int(val: i32) -> Self {
        val as Self
    }
    fn from_long(val: i64) -> Self {
        val
    }
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    fn from_float(val: f32) -> Self {
        val.round() as Self
    }
    fn from_double(val: f64) -> Self {
        val.round() as Self
    }
    fn from_bool(val: bool) -> Self {
        val as Self
    }
}
impl GlDstType for f32 {
    fn from_int(val: i32) -> Self {
        val as Self
    }
    fn from_long(val: i64) -> Self {
        val as Self
    }
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    fn from_float(val: f32) -> Self {
        val
    }
    fn from_double(val: f64) -> Self {
        val as Self
    }
    fn from_bool(val: bool) -> Self {
        val as u8 as Self
    }
}
impl GlDstType for f64 {
    fn from_int(val: i32) -> Self {
        val as Self
    }
    fn from_long(val: i64) -> Self {
        val as Self
    }
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    fn from_float(val: f32) -> Self {
        val as Self
    }
    fn from_double(val: f64) -> Self {
        val
    }
    fn from_bool(val: bool) -> Self {
        val as u8 as Self
    }
}
impl GlDstType for bool {
    fn from_uint(val: u32) -> Self {
        val != 0
    }

    fn from_ulong(val: u64) -> Self {
        val != 0
    }

    fn from_int(val: i32) -> Self {
        val != 0
    }

    fn from_long(val: i64) -> Self {
        val != 0
    }

    fn from_float(val: f32) -> Self {
        val != 0.0
    }

    fn from_double(val: f64) -> Self {
        val != 0.0
    }

    fn from_bool(val: bool) -> Self {
        val
    }
}
