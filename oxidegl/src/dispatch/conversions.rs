use core::ptr;
use std::num::NonZeroU32;

use crate::context::state::{NamedObject, ObjectName};

use super::gl_types::GLenum;
use core::fmt::Debug;
/// Trait defined for all custom bitfield and enum types which allows them to be unsafely created
/// from an underlying `GLenum` (u32) value with checks on debug builds
pub trait UnsafeFromGLenum {
    unsafe fn unsafe_from_gl_enum(val: GLenum) -> Self;
}
/// Helper trait to convert from "raw" [`GLenums`](crate::dispatch::gl_types::GLenum) to wrappers around subsets of those that are valid for certain functions
pub trait GLenumExt<T> {
    unsafe fn into_enum(self) -> T;
}

impl<T> GLenumExt<T> for GLenum
where
    T: UnsafeFromGLenum,
{
    unsafe fn into_enum(self) -> T {
        unsafe { T::unsafe_from_gl_enum(self) }
    }
}

/// Trait that describes a type that may be returned from a glGet* function.
pub trait GlDstType: Copy {
    fn from_uint(val: u32) -> Self;
    fn from_ulong(val: u64) -> Self;
    fn from_int(val: i32) -> Self;
    fn from_long(val: i64) -> Self;
    fn from_float(val: f32) -> Self;
    fn from_double(val: f64) -> Self;
    fn from_bool(val: bool) -> Self;
}
/// Trait that describes how to convert a given type into a value that may be returned according to the
/// conversion rules for state items in the GL spec
pub trait SrcType<Dst: GlDstType>: Copy {
    /// Convert from source type Self to the given destination type
    fn convert(self) -> Dst;
}
pub trait MaybeObjectName<T> {
    fn get(self) -> Option<ObjectName<T>>;
}
impl<T> MaybeObjectName<T> for ObjectName<T> {
    fn get(self) -> Option<ObjectName<T>> {
        Some(self)
    }
}
impl<T: NamedObject> MaybeObjectName<T> for u32 {
    fn get(self) -> Option<ObjectName<T>> {
        Some(ObjectName::try_from_raw(self).expect("raw name was not a valid object name"))
    }
}
pub struct CurrentBinding;
impl<T> MaybeObjectName<T> for CurrentBinding {
    fn get(self) -> Option<ObjectName<T>> {
        None
    }
}
/// Trait that abstracts indexing for cases where a function should return
pub trait IndexType: Copy + Sized + Debug {
    fn get(self) -> Option<usize>;
    fn get_numeric(self) -> usize {
        self.get().unwrap_or(0)
    }
}
impl IndexType for u32 {
    fn get(self) -> Option<usize> {
        Some(self as usize)
    }
}
impl IndexType for Option<u32> {
    fn get(self) -> Option<usize> {
        self.map(|v| v as usize)
    }
}
impl IndexType for Option<NonZeroU32> {
    fn get(self) -> Option<usize> {
        self.map(|v| v.get() as usize)
    }
}
impl IndexType for usize {
    fn get(self) -> Option<usize> {
        Some(self)
    }
}
#[derive(Clone, Copy)]
pub struct NoIndex;
impl IndexType for NoIndex {
    #[inline]
    fn get(self) -> Option<usize> {
        None
    }
}
impl Debug for NoIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[unindexed]")
    }
}

/// Trait that allows converting a value to an inferred Dst type according to the conversion rules
/// given by the GL Spec for state entries and unsafely writing it to a type-erased allocation
pub trait StateQueryWrite<Dst: GlDstType> {
    type It: SrcType<Dst>;
    /// Write the GL state attributes stored in Self at index I to the destination pointer
    unsafe fn write_out<I: IndexType>(&self, idx: I, ptr: *mut Dst);
    /// Write the GL state attributes stored in Self to the destination pointer
    unsafe fn write_noindex(&self, ptr: *mut Dst) {
        // Safety: See Self::write_out
        unsafe { self.write_out(NoIndex, ptr) };
    }
}

impl<It: SrcType<Dst>, Dst: GlDstType> StateQueryWrite<Dst> for [It] {
    type It = It;
    #[inline]
    unsafe fn write_out<I: IndexType>(&self, idx: I, mut ptr: *mut Dst) {
        debug_assert!(
            ptr.is_aligned(),
            "Destination pointer passed to write_out should have been aligned correctly!"
        );
        if let Some(i) = idx.get() {
            debug_assert!(
                i < self.len(),
                "Tried to read outside the bounds of an array of items"
            );
            // Safety: Caller ensures ptr points to an allocation with the correct size and alignment to store a
            // single value of type Dst
            unsafe { ptr::write(ptr, self.get_unchecked(i).convert()) }
            return;
        }
        for item in self {
            // Safety: Caller ensures that ptr points to an allocation with the same size and alignment as this array.
            unsafe { ptr::write(ptr, item.convert()) }
            // Safety: Caller ensures the length of the allocation is equal to the length of this array
            unsafe { ptr = ptr.add(1) }
        }
    }
}

impl<It: SrcType<Dst>, Dst: GlDstType> StateQueryWrite<Dst> for It {
    type It = Self;
    #[inline]
    unsafe fn write_out<I: IndexType>(&self, idx: I, ptr: *mut Dst) {
        let i = idx.get();
        debug_assert!(
            i.is_none() || i.map(|v| v == 0) == Some(true),
            "UB: Tried to read outside the bounds of a single item"
        );
        debug_assert!(
            ptr.is_aligned(),
            "UB: Destination pointer passed to write_out should have been aligned correctly!"
        );
        // Safety: caller ensures that Dst is the correct type for the allocation being written to
        unsafe { ptr::write(ptr, self.convert()) }
    }
}

//TODO: wrap all of this in a macro
impl<Dst: GlDstType> SrcType<Dst> for u32 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_uint(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for i32 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_int(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for u64 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_ulong(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for i64 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_long(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for usize {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_ulong(self as u64)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for isize {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_long(self as i64)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for f32 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_float(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for f64 {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_double(self)
    }
}
impl<Dst: GlDstType> SrcType<Dst> for bool {
    #[inline]
    fn convert(self) -> Dst {
        Dst::from_bool(self)
    }
}
impl GlDstType for u32 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val.unsigned_abs()
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val.unsigned_abs() as Self
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val as Self
    }

    #[inline]
    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_bool(val: bool) -> Self {
        val as Self
    }
}

impl GlDstType for u64 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val.unsigned_abs() as Self
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val.unsigned_abs()
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val
    }

    #[inline]
    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_bool(val: bool) -> Self {
        val as Self
    }
}
impl GlDstType for i32 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val as Self
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val as Self
    }

    #[inline]
    fn from_float(val: f32) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_double(val: f64) -> Self {
        val.round() as Self
    }

    #[inline]
    fn from_bool(val: bool) -> Self {
        val as Self
    }
}

impl GlDstType for i64 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val as Self
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    #[inline]
    fn from_float(val: f32) -> Self {
        val.round() as Self
    }
    #[inline]
    fn from_double(val: f64) -> Self {
        val.round() as Self
    }
    #[inline]
    fn from_bool(val: bool) -> Self {
        val as Self
    }
}
impl GlDstType for f32 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val as Self
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val as Self
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    #[inline]
    fn from_float(val: f32) -> Self {
        val
    }
    #[inline]
    fn from_double(val: f64) -> Self {
        val as Self
    }
    #[inline]
    fn from_bool(val: bool) -> Self {
        val as u8 as Self
    }
}
impl GlDstType for f64 {
    #[inline]
    fn from_int(val: i32) -> Self {
        val as Self
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        val as Self
    }
    #[inline]
    fn from_uint(val: u32) -> Self {
        val as Self
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        val as Self
    }
    #[inline]
    fn from_float(val: f32) -> Self {
        val as Self
    }
    #[inline]
    fn from_double(val: f64) -> Self {
        val
    }
    #[inline]
    fn from_bool(val: bool) -> Self {
        val as u8 as Self
    }
}
impl GlDstType for bool {
    #[inline]
    fn from_uint(val: u32) -> Self {
        val != 0
    }

    #[inline]
    fn from_ulong(val: u64) -> Self {
        val != 0
    }

    #[inline]
    fn from_int(val: i32) -> Self {
        val != 0
    }

    #[inline]
    fn from_long(val: i64) -> Self {
        val != 0
    }

    #[inline]
    fn from_float(val: f32) -> Self {
        val != 0.0
    }

    #[inline]
    fn from_double(val: f64) -> Self {
        val != 0.0
    }

    #[inline]
    fn from_bool(val: bool) -> Self {
        val
    }
}
