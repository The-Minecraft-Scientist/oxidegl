use core::ptr;

use crate::context::state::{NamedObject, ObjectName};

use super::gl_types::GLenum;
use core::fmt::Debug;
/// Trait defined for all custom bitfield and enum types which allows them to be unsafely created
/// from an underlying `GLenum` (u32) value with checks on debug builds
pub(crate) trait UnsafeFromGLenum {
    unsafe fn unsafe_from_gl_enum(val: GLenum) -> Self;
}
/// Helper trait to convert from "raw" [`GLenums`](crate::dispatch::gl_types::GLenum) to wrappers around subsets of those that are valid for certain functions
pub(crate) trait GLenumExt<T> {
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

pub(crate) trait MaybeObjectName<T> {
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
pub(crate) struct CurrentBinding;
impl<T> MaybeObjectName<T> for CurrentBinding {
    fn get(self) -> Option<ObjectName<T>> {
        None
    }
}

/// Trait that describes a type that may be returned from a glGet* function.
pub(crate) trait GlDstType: Copy {
    fn from_uint(val: u32) -> Self;
    fn from_ulong(val: u64) -> Self;
    fn from_int(val: i32) -> Self;
    fn from_long(val: i64) -> Self;
    fn from_float(val: f32) -> Self;
    fn from_double(val: f64) -> Self;
    fn from_bool(val: bool) -> Self;
}
/// Trait that describes how to convert a given type into a value that may be returned according to the
/// conversion rules for state items laid out by the GL spec
pub(crate) trait SrcType<Dst: GlDstType>: Copy {
    /// Convert from source type Self to the given destination type
    fn convert(self) -> Dst;
}

pub(crate) trait GlGetItem<Dst: GlDstType> {
    unsafe fn write_out(&self, ptr: *mut Dst);
}
impl<Dst: GlDstType, T: SrcType<LocalWrap<Dst>>> GlGetItem<Dst> for T {
    #[inline]
    unsafe fn write_out(&self, ptr: *mut Dst) {
        // Safety: caller ensures `ptr` is valid for a write of type `Dst`
        unsafe {
            ptr::write(ptr, LocalInto::into(self.convert()));
        }
    }
}

impl<Dst: GlDstType, T: SrcType<LocalWrap<Dst>>, const N: usize> GlGetItem<Dst> for [T; N] {
    #[inline]
    unsafe fn write_out(&self, mut ptr: *mut Dst) {
        for i in self {
            // Safety: caller ensures `ptr` is valid for writes
            unsafe { ptr::write(ptr, LocalInto::into(i.convert())) };
            // Safety: caller ensures `ptr` points to an allocation with enough space for a [Dst; N]
            unsafe { ptr = ptr.add(1) };
        }
    }
}

// Either an index or a lazily evaluated panic
pub(crate) trait MaybeIndex: Copy + Sized + Debug {
    fn get(self) -> usize;
    fn get_opt(self) -> Option<usize> {
        Some(self.get())
    }
}
impl MaybeIndex for u32 {
    fn get(self) -> usize {
        self as usize
    }
}

impl MaybeIndex for usize {
    fn get(self) -> usize {
        self
    }
}
#[derive(Clone, Copy)]
pub(crate) struct NoIndex;
impl MaybeIndex for NoIndex {
    #[inline]
    fn get(self) -> usize {
        panic!("Tried to index an array but no NoIndex was specified!")
    }
    #[inline]
    fn get_opt(self) -> Option<usize> {
        None
    }
}
impl Debug for NoIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[unindexed]")
    }
}

pub(crate) trait GlGetItemSliceExt<Dst: GlDstType> {
    type SliceItem;
    unsafe fn write_out_index_mapped<
        I: MaybeIndex,
        Out: GlGetItem<Dst>,
        F: FnOnce(&Self::SliceItem) -> Out,
    >(
        &self,
        idx: I,
        ptr: *mut Dst,
        map: F,
    );
    unsafe fn write_out_index<I: MaybeIndex>(&self, idx: I, ptr: *mut Dst)
    where
        Self::SliceItem: GlGetItem<Dst>;
}
impl<T, Dst: GlDstType> GlGetItemSliceExt<Dst> for [T] {
    type SliceItem = T;

    unsafe fn write_out_index_mapped<
        I: MaybeIndex,
        Out: GlGetItem<Dst>,
        F: FnOnce(&Self::SliceItem) -> Out,
    >(
        &self,
        idx: I,
        ptr: *mut Dst,
        map: F,
    ) {
        unsafe { map(&self[idx.get()]).write_out(ptr) }
    }

    unsafe fn write_out_index<I: MaybeIndex>(&self, idx: I, ptr: *mut Dst)
    where
        Self::SliceItem: GlGetItem<Dst>,
    {
        unsafe { self[idx.get()].write_out(ptr) };
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct LocalWrap<T>(T);
impl<T> LocalFrom<LocalWrap<T>> for T {
    #[inline]
    fn from(value: LocalWrap<T>) -> Self {
        value.0
    }
}
trait LocalFrom<T> {
    fn from(value: T) -> Self;
}
trait LocalInto<T> {
    fn into(self) -> T;
}
impl<U, T: LocalFrom<U>> LocalInto<T> for U {
    #[inline]
    fn into(self) -> T {
        T::from(self)
    }
}
impl<T: GlDstType> GlDstType for LocalWrap<T> {
    #[inline]
    fn from_uint(val: u32) -> Self {
        Self(T::from_uint(val))
    }
    #[inline]
    fn from_ulong(val: u64) -> Self {
        Self(T::from_ulong(val))
    }
    #[inline]
    fn from_int(val: i32) -> Self {
        Self(T::from_int(val))
    }
    #[inline]
    fn from_long(val: i64) -> Self {
        Self(T::from_long(val))
    }
    #[inline]
    fn from_float(val: f32) -> Self {
        Self(T::from_float(val))
    }
    #[inline]
    fn from_double(val: f64) -> Self {
        Self(T::from_double(val))
    }
    #[inline]
    fn from_bool(val: bool) -> Self {
        Self(T::from_bool(val))
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
