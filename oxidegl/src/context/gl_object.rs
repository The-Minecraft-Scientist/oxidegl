use std::{cell::UnsafeCell, fmt::Debug, marker::PhantomData, num::NonZeroU32};

use objc2::rc::Retained;
use objc2_foundation::NSString;

use crate::{
    debug_unreachable,
    dispatch::{
        conversions::{GlDstType, SrcType},
        gl_types::{GLboolean, GLsizei, GLuint},
    },
    trimmed_type_name,
};

use super::{
    debug::{gl_debug, with_debug_state},
    Context,
};

/// Marker trait that marks a struct as an OpenGL object, providing information on whether it has init-at-bind (`LateInit`) semantics or normal semantics, and (optionally) how to set the underlying debug label
pub(crate) trait NamedObject: Sized + 'static {
    fn set_debug_label(
        ctx: &mut Context,
        name: ObjectName<Self>,
        label: Option<Retained<NSString>>,
    ) {
    }

    type LateInitType: GetLateInitTypes<Obj = Self> + GetCellType<Obj = Self>;

    const LATE_INIT_FUNC: <Self::LateInitType as GetLateInitTypes>::Func =
        <Self::LateInitType as GetLateInitTypes>::Func::DEFAULT;
}

pub(crate) trait GetLateInitTypes {
    type Obj: NamedObject<LateInitType = Self> + Debug;
    type Func: DefaultConstOrError;
}
pub(crate) trait DefaultConstOrError {
    const DEFAULT: Self;
}
impl DefaultConstOrError for () {
    const DEFAULT: Self = ();
}
impl<T> DefaultConstOrError for fn(ObjectName<T>) -> T {
    const DEFAULT: Self =
        panic!("Please specify a late-init function in the corresponding ObjectName impl");
}
pub(crate) struct LateInit<T>(PhantomData<fn(&T)>);
impl<T: NamedObject<LateInitType = LateInit<T>> + Debug> GetLateInitTypes for LateInit<T> {
    type Obj = T;
    type Func = fn(ObjectName<T>) -> T;
}
pub(crate) struct NoLateInit<T>(PhantomData<fn(&T)>);
impl<T: NamedObject<LateInitType = NoLateInit<T>> + Debug> GetLateInitTypes for NoLateInit<T> {
    type Obj = T;
    type Func = ();
}
pub(crate) trait GetCellType {
    type Obj;
    type Cell: NameStateInterface<Self::Obj> + Debug;
}
impl<T: NamedObject + Debug> GetCellType for LateInit<T>
where
    <T::LateInitType as GetLateInitTypes>::Func: FnOnce(ObjectName<T>) -> T,
{
    type Obj = T;
    type Cell = NameStateLateInitCell<T::LateInitType>;
}
impl<T: NamedObject + Debug> GetCellType for NoLateInit<T> {
    type Obj = T;
    type Cell = Option<T>;
}
pub(crate) trait NameStateInterface<T> {
    /// Returns None if this object name has not yet been initialized, or Some containing a shared reference to the object's state
    fn get(&self) -> Option<&T>;
    /// Returns None if this object name has not yet been initialized, or Some containing a mutable reference to the object's state
    fn get_mut(&mut self) -> Option<&mut T>;
    /// Returns a new value of type Self that has not yet been initialized
    fn new_empty(name: ObjectName<T>) -> Self;
    /// Returns a new value of type Self that has been initialized (i.e. a call to `get` on the returned value returns Some(&obj))
    fn new_init(obj: T) -> Self;
}
pub struct NameStateLateInitCell<Types: GetLateInitTypes>
where
    Types::Func: FnOnce(ObjectName<Types::Obj>) -> Types::Obj,
{
    inner: UnsafeCell<NameOrObj<Types::Obj>>,
}
enum NameOrObj<T> {
    Name(ObjectName<T>),
    Obj(T),
}
impl<T> NameOrObj<T> {
    #[inline]
    fn not_yet_initialized(&self) -> Option<ObjectName<T>> {
        match self {
            NameOrObj::Name(object_name) => Some(*object_name),
            NameOrObj::Obj(_) => None,
        }
    }
}

impl<T: GetLateInitTypes> Debug for NameStateLateInitCell<T>
where
    T::Obj: Debug,
    T::Func: FnOnce(ObjectName<T::Obj>) -> T::Obj,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameStateLateInitCell")
            .field("inner", &self.get())
            .finish()
    }
}

impl<T: GetLateInitTypes> NameStateLateInitCell<T>
where
    <T as GetLateInitTypes>::Func: FnOnce(ObjectName<<T as GetLateInitTypes>::Obj>) -> T::Obj,
{
    #[inline]
    fn ensure_init(&self) -> &T::Obj {
        if let Some(name) =
            // Safety:
            // self is borrowed via a shared reference. If this cell is initialized, it
            // is always sound to create an additional shared reference to the inner value.
            // (since the inner value cannot also be borrowed mutably).
            // If this cell is not yet initialized (i.e. `inner` contains the `Name` variant of `NameOrObj`),
            // it is guaranteed that no function produces a non-temporary reference of any kind to the inner type,
            // and this struct is !Sync, so two racing executions of this function are impossible
            unsafe { self.inner.get().as_ref().unwrap_unchecked() }.not_yet_initialized()
        {
            Self::init_inner(
                // Safety: No shared references to the inner type are present if has not yet been initialized (see above)
                unsafe { self.inner.get().as_mut().unwrap_unchecked() },
                name,
            );
        }
        // Safety: valid to create shared references to initialized inner value (we just ensured it is initialized)
        match unsafe { self.inner.get().as_ref().unwrap_unchecked() } {
            // Safety: see ^
            NameOrObj::Name(_) => unsafe { debug_unreachable!("bug in NameStateCell") },
            NameOrObj::Obj(r) => r,
        }
    }
    #[inline]
    fn init_inner(inner_ref: &mut NameOrObj<T::Obj>, name: ObjectName<T::Obj>) {
        *inner_ref = NameOrObj::Obj(T::Obj::LATE_INIT_FUNC(name));
    }
    #[inline]
    fn ensure_init_mut(&mut self) -> &mut T::Obj {
        let inner_ref = self.inner.get_mut();
        if let Some(name) = inner_ref.not_yet_initialized() {
            Self::init_inner(inner_ref, name);
        }
        match self.inner.get_mut() {
            //Safety: we just ensured that the inner type is initialized, it is impossible for it to be in this state
            NameOrObj::Name(_) => unsafe { debug_unreachable!("bug in NameStateCell") },
            NameOrObj::Obj(o) => o,
        }
    }
}

impl<T: GetLateInitTypes> NameStateInterface<T::Obj> for NameStateLateInitCell<T>
where
    <T as GetLateInitTypes>::Func: FnOnce(ObjectName<<T as GetLateInitTypes>::Obj>) -> T::Obj,
{
    #[inline]
    fn get(&self) -> Option<&T::Obj> {
        Some(self.ensure_init())
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T::Obj> {
        Some(self.ensure_init_mut())
    }
    #[inline]
    fn new_empty(name: ObjectName<T::Obj>) -> Self {
        Self {
            inner: UnsafeCell::new(NameOrObj::Name(name)),
        }
    }
    #[inline]
    fn new_init(obj: T::Obj) -> Self {
        Self {
            inner: UnsafeCell::new(NameOrObj::Obj(obj)),
        }
    }
}

impl<T: NamedObject> NameStateInterface<T> for Option<T> {
    #[inline]
    fn get(&self) -> Option<&T> {
        self.as_ref()
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
    #[inline]
    fn new_empty(_name: ObjectName<T>) -> Self {
        None
    }
    #[inline]
    fn new_init(obj: T) -> Self {
        Some(obj)
    }
}

/// Represents the name of an OpenGL object of type T.
/// Note that the generic parameter is simply there to prevent accidental misuse of
/// object names, since an arbitrary `ObjectName` can be created safely (i.e. these are simply identifiers, not true "handles" in the RAII sense)
#[repr(transparent)]
pub struct ObjectName<T: ?Sized>(NonZeroU32, PhantomData<for<'a> fn(&'a T) -> &'a T>);

impl<T: ?Sized> PartialEq for ObjectName<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ?Sized> Eq for ObjectName<T> {}
impl<T: ?Sized> Clone for ObjectName<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized + 'static> std::hash::Hash for ObjectName<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        std::any::TypeId::of::<T>().hash(state);
    }
}
impl<T: ?Sized + 'static> core::fmt::Debug for ObjectName<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if let Some(label) = with_debug_state(|s| s.get_label(*self)).flatten() {
            format!("({})", label.to_str().unwrap_or("invalid utf-8 for label"))
        } else {
            String::new()
        };
        f.pad(&format!(
            "{} #{} {s}",
            trimmed_type_name::<T>(),
            self.0.get()
        ))
    }
}
impl<T: ?Sized> Copy for ObjectName<T> {}
impl<T: ?Sized> ObjectName<T> {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    /// unsafely create an `ObjectName` from a 0-indexed internal object ID (without checking overflow)
    unsafe fn from_idx(val: usize) -> Self {
        Self(
            // Safety: Caller ensures val is <= u32::MAX - 1
            unsafe { NonZeroU32::new_unchecked((val + 1) as u32) },
            PhantomData,
        )
    }
    #[inline]
    /// Try creating a new `ObjectName` from a possibly-zero input value
    pub fn try_from_raw(name: u32) -> Option<Self> {
        Some(Self(NonZeroU32::new(name)?, PhantomData))
    }
    #[inline]
    /// Create a new `ObjectName` from a possibly-zero input value, panicking on zero
    pub fn from_raw(name: u32) -> Self {
        Self(
            NonZeroU32::new(name).expect("UB: object name at 0 is reserved"),
            PhantomData,
        )
    }
    #[inline]
    /// Convert this `ObjectName` to a 0-indexed ID for usage indexing object lists
    pub fn to_idx(self) -> usize {
        (self.0.get() - 1) as usize
    }
    /// Get the raw inner value of this object name
    #[inline]
    pub fn to_raw(self) -> u32 {
        self.0.get()
    }
    /// Cast the type of object this name refers to a different type
    #[inline]
    pub fn cast<U: ?Sized>(self) -> ObjectName<U> {
        ObjectName(self.0, PhantomData)
    }
    #[inline]
    /// Pretty-prints this object name to a string with no spaces
    pub fn name_no_space(self) -> String {
        format!("{}-{}", trimmed_type_name::<T>(), self.0)
    }
}

impl<Dst: GlDstType, T> SrcType<Dst> for Option<ObjectName<T>> {
    fn convert(self) -> Dst {
        Dst::from_uint(self.map_or(0, |v| v.0.get()))
    }
}
impl<T: NamedObject> Debug for NamedObjectList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamedObjectList")
            .field("objects", &self.objects)
            .finish()
    }
}
pub struct NamedObjectList<T: NamedObject> {
    objects: Vec<<T::LateInitType as GetCellType>::Cell>,
}
impl<T: NamedObject> Default for NamedObjectList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Obj: NamedObject> NamedObjectList<Obj> {
    const NON_EXISTENT: &str = "Tried to use a nonexistent object name";
    const NOT_INITIALIZED: &str =
        "Tried to use an object name that was not initialized to an object";
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::with_capacity(32),
        }
    }
    #[inline]
    pub(crate) fn get(&self, name: ObjectName<Obj>) -> &Obj {
        self.objects
            .get(name.to_idx())
            .expect(Self::NON_EXISTENT)
            .get()
            .expect(Self::NOT_INITIALIZED)
    }
    #[inline]
    pub(crate) fn get_raw(&self, name: GLuint) -> &Obj {
        self.get(ObjectName::from_raw(name))
    }
    #[inline]
    pub(crate) fn get_opt(&self, name: ObjectName<Obj>) -> Option<&Obj> {
        self.objects
            .get(name.to_idx())
            .and_then(|name_state| name_state.get())
    }
    #[inline]
    pub(crate) unsafe fn get_unchecked(&self, name: ObjectName<Obj>) -> &Obj {
        // Safety: Caller ensures that the object at name exists in the list
        unsafe {
            self.objects
                .get_unchecked(name.to_idx())
                .get()
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub(crate) fn get_mut(&mut self, name: ObjectName<Obj>) -> &mut Obj {
        self.objects
            .get_mut(name.to_idx())
            .expect(Self::NON_EXISTENT)
            .get_mut()
            .expect(Self::NOT_INITIALIZED)
    }
    #[inline]
    pub(crate) fn get_raw_mut(&mut self, name: GLuint) -> &mut Obj {
        self.get_mut(ObjectName::from_raw(name))
    }
    #[inline]
    pub(crate) fn get_opt_mut(&mut self, name: ObjectName<Obj>) -> Option<&mut Obj> {
        self.objects
            .get_mut(name.to_idx())
            .and_then(|name_state| name_state.get_mut())
    }
    #[inline]
    pub(crate) unsafe fn get_unchecked_mut(&mut self, name: ObjectName<Obj>) -> &mut Obj {
        // Safety: Caller ensures that the object at name exists in the list
        unsafe {
            self.objects
                .get_unchecked_mut(name.to_idx())
                .get_mut()
                .unwrap_unchecked()
        }
    }
    /// Generates a new valid object name (e.g. for glGen*)
    // Overflow is checked
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn new_name(&mut self) -> ObjectName<Obj> {
        debug_assert!(
            self.objects.len() < (u32::MAX - 1) as usize,
            "UB: OxideGL does not allow generation of more than u32::MAX object names"
        );

        // Safety: see assertion
        let name = unsafe { ObjectName::from_idx(self.objects.len()) };
        self.objects
            .push(<Obj::LateInitType as GetCellType>::Cell::new_empty(name));
        name
    }

    /// Generates a new object name, calls the given initialization
    /// function on the name, adds the newly created object to this list,
    /// and returns the name of the newly generated object
    // Overflow is checked
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn new_obj(
        &mut self,
        create: impl FnOnce(ObjectName<Obj>) -> Obj,
    ) -> ObjectName<Obj> {
        debug_assert!(
            self.objects.len() < (u32::MAX - 1) as usize,
            "UB: OxideGL does not allow generation of more than u32::MAX - 1 object names"
        );
        // Safety: see assertion
        let name = unsafe { ObjectName::from_idx(self.objects.len()) };
        self.objects
            .push(<Obj::LateInitType as GetCellType>::Cell::new_init(create(
                name,
            )));
        name
    }
    /// Whether the given object name points to a valid and initialized object
    #[inline]
    pub(crate) fn is(&self, name: ObjectName<Obj>) -> bool {
        self.get_opt(name).is_some()
    }
    /// Immediately remove an object from the list
    #[inline]
    pub(crate) fn delete(&mut self, name: ObjectName<Obj>) {
        if let Some(entry) = self.objects.get_mut(name.to_idx()) {
            let mut e = <Obj::LateInitType as GetCellType>::Cell::new_empty(name);
            core::mem::swap(entry, &mut e);
            // make the drop explicit for clarity
            drop(e);
        }
    }
    /// Helper implementing the glDelete* pattern
    #[inline]
    pub(crate) unsafe fn delete_objects(&mut self, n: GLsizei, to_delete: *const GLuint) {
        debug_assert!(
            !to_delete.is_null(),
            "UB: object name array pointer was null"
        );
        debug_assert!(
            to_delete.is_aligned(),
            "UB: object name array pointer was not sufficiently aligned"
        );
        for &name in
            // Safety: caller ensures that n and buffers form a valid reference to a u32 slice. Cast from [u32] to
            // [Option<ReprTransparentStruct(NonZeroU32)>] is guaranteed to be valid by Option niche opt guarantees
            unsafe {
                core::slice::from_raw_parts(to_delete.cast::<Option<ObjectName<Obj>>>(), n as usize)
            }
            .iter()
            .flatten()
        {
            self.delete(name);
            gl_debug!(target: "object alloc", "deleted {} {name:?}", trimmed_type_name::<Obj>());
        }
    }
    /// Helper implementing the glGen* pattern
    #[inline]
    pub(crate) unsafe fn gen_obj(&mut self, n: GLsizei, names: *mut GLuint) {
        debug_assert!(!names.is_null(), "UB: object name array pointer was null");
        debug_assert!(
            names.is_aligned(),
            "UB: object name array pointer was not sufficiently aligned"
        );
        gl_debug!(target: "oxidegl::object_alloc", "writing {n} new {} names to {names:?}", trimmed_type_name::<Obj>());
        let mut names = names.cast();
        for _ in 0..n {
            let name = self.new_name();
            // Safety: Caller ensures names is valid to write n buffer names to
            unsafe { core::ptr::write(names, name) }
            // Safety: See above, this pointer will point at most one item past the end of its allocation
            unsafe { names = names.add(1) }
        }
    }
    /// Helper implementing the glCreate* pattern, using an initialization function to fully initialize the objects in question
    #[inline]
    pub(crate) unsafe fn create_obj(
        &mut self,
        create_func: impl Fn(ObjectName<Obj>) -> Obj + Copy,
        n: GLsizei,
        names: *mut GLuint,
    ) {
        debug_assert!(!names.is_null(), "UB: object name array pointer was null");
        debug_assert!(
            names.is_aligned(),
            "UB: object name array pointer was not sufficiently aligned"
        );
        gl_debug!(
            target: "oxidegl::object_alloc",
            "writing {n} new initialized {} names to {names:?}",
            trimmed_type_name::<Obj>()
        );
        let mut names = names.cast();
        for _ in 0..n {
            let name = self.new_obj(create_func);
            // Safety: Caller ensures names is valid to write n object names to
            unsafe { core::ptr::write(names, name) }
            // Safety: See above, this pointer will point at most one item past the end of its allocation
            unsafe { names = names.add(1) }
        }
    }
    #[inline]
    /// Helper wrapping `is` to take a raw, possibly-zero `GLenum`
    pub(crate) fn is_obj(&self, name: GLuint) -> GLboolean {
        ObjectName::try_from_raw(name).is_some_and(|name| self.is(name))
    }
    #[inline]
    /// Helper to ensure that a given object name is fully initialized prior to use
    /// (to support GL's cursed init-at-first-binding pattern)
    pub(crate) fn ensure_init(
        &mut self,
        name: ObjectName<Obj>,
        default: impl Fn(ObjectName<Obj>) -> Obj,
    ) {
        if let Some(v) = self.objects.get_mut(name.to_idx()) {
            if v.get().is_some() {
                return;
            }
            *v = <Obj::LateInitType as GetCellType>::Cell::new_init(default(name));
        } else {
            panic!("object name wasnot initialized!");
        }
    }
}
