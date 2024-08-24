use std::{marker::PhantomData, num::NonZeroU32};

use bitflags::bitflags;
use log::debug;

use crate::{
    debug_unreachable,
    dispatch::{
        conversions::{GlDstType, SrcType},
        gl_types::{GLboolean, GLenum, GLsizei, GLuint},
    },
    enums::{
        ClearBufferMask, GL_CONTEXT_CORE_PROFILE_BIT, GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT,
        GL_CONTEXT_FLAG_NO_ERROR_BIT,
    },
    type_name,
};

use super::{
    commands::{buffer::Buffer, vao::Vao},
    shader::Shader,
};

#[derive(Debug)]
pub struct GLState {
    pub(crate) characteristics: Characteristics,
    pub(crate) buffer_bindings: BufferBindings,
    pub(crate) buffer_list: NamedObjectList<Buffer>,
    pub(crate) vao_list: NamedObjectList<Vao>,
    pub(crate) vao_binding: Option<ObjectName<Vao>>,
    pub(crate) shader_list: NamedObjectList<Shader>,
    pub(crate) point_size: f32,
    pub(crate) line_width: f32,
    pub(crate) clear_color: [f32; 4],
    pub(crate) clear_depth: f32,
    pub(crate) clear_mask: ClearBufferMask,
    pub(crate) clear_stencil: i32,
}
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NeedsRefreshBits: u32 {
        const RENDER_PASS = 0b1;
        const BUFFERS = 1 << 1;
    }
}

pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: usize = 16;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: usize = 16;
pub const MAX_TRANSFORM_FEEDBACK_BUFFER_BINDINGS: usize = 16;
pub const MAX_UNIFORM_BUFFER_BINDINGS: usize = 16;

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferBindings {
    /// Vertex attribute buffer
    pub(crate) array: Option<ObjectName<Buffer>>,
    /// Atomic counter storage
    pub(crate) atomic_counter: [Option<ObjectName<Buffer>>; MAX_ATOMIC_COUNTER_BUFFER_BINDINGS],
    /// Buffer copy source
    pub(crate) copy_read: Option<ObjectName<Buffer>>,
    /// Buffer copy destination
    pub(crate) copy_write: Option<ObjectName<Buffer>>,
    /// Indirect compute dispatch commands
    pub(crate) dispatch_indirect: Option<ObjectName<Buffer>>,
    /// Indirect draw command arguments
    pub(crate) draw_indirect: Option<ObjectName<Buffer>>,
    /// Vertex array indices
    pub(crate) element_array: Option<ObjectName<Buffer>>,
    /// Draw parameters
    pub(crate) parameter: Option<ObjectName<Buffer>>,
    /// Pixel read target
    pub(crate) pixel_pack: Option<ObjectName<Buffer>>,
    /// Texture data source
    pub(crate) pixel_unpack: Option<ObjectName<Buffer>>,
    /// Query results
    pub(crate) query: Option<ObjectName<Buffer>>,
    /// Shader storage buffers
    pub(crate) shader_storage: [Option<ObjectName<Buffer>>; MAX_SHADER_STORAGE_BUFFER_BINDINGS],
    /// Texture data buffer
    pub(crate) texture: Option<ObjectName<Buffer>>,
    /// Transform feedback result buffers
    pub(crate) transform_feedback:
        [Option<ObjectName<Buffer>>; MAX_TRANSFORM_FEEDBACK_BUFFER_BINDINGS],
    /// Uniform storage buffers
    pub(crate) uniform: [Option<ObjectName<Buffer>>; MAX_UNIFORM_BUFFER_BINDINGS],
}

impl GLState {
    pub fn default() -> Self {
        GLState {
            characteristics: Characteristics::new(),
            buffer_bindings: BufferBindings::default(),
            buffer_list: NamedObjectList::new(),

            vao_list: NamedObjectList::new(),
            shader_list: NamedObjectList::new(),
            vao_binding: None,
            point_size: 1.0,
            line_width: 1.0,

            clear_color: [0.0; 4],
            clear_depth: 0.0,
            clear_mask: ClearBufferMask::empty(),
            clear_stencil: 0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Characteristics {
    pub(crate) point_size_range: [f32; 2],
    pub(crate) point_size_granularity: f32,
    pub(crate) line_width_range: [f32; 2],
    pub(crate) line_width_granularity: f32,
    pub(crate) context_flags: GLenum,
    pub(crate) context_profile_mask: GLenum,
    pub(crate) num_extensions: u32,
}

impl Characteristics {
    fn new() -> Self {
        Self {
            point_size_range: [1.0, 1.0],
            point_size_granularity: 0.0001,
            line_width_range: [1.0, 1.0],
            line_width_granularity: 0.0001,
            context_flags: GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT | GL_CONTEXT_FLAG_NO_ERROR_BIT,
            context_profile_mask: GL_CONTEXT_CORE_PROFILE_BIT,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
/// Tracks the state of a given *name* throughout the GL server lifetime
pub enum NameState<T> {
    /// No object is present; the object previously resident in this name has been deleted
    Empty,
    /// Indeterminate state that lies between glGen* and glBind* for non-DSA access
    Named,
    /// This object name is *bound* to an object with the given state
    Bound(T),
}
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjectName<Obj>(NonZeroU32, PhantomData<for<'a> fn(&'a Obj) -> &'a Obj>);
impl<T> Clone for ObjectName<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Obj> core::fmt::Debug for ObjectName<Obj> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{} #{}", type_name::<Obj>(), self.0.get()))
    }
}
impl<T> Copy for ObjectName<T> {}
impl<T: NamedObject> ObjectName<T> {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    unsafe fn from_idx(val: usize) -> Self {
        Self(
            //Safety: Caller ensures val is <= u32::MAX - 1
            unsafe { NonZeroU32::new_unchecked((val + 1) as u32) },
            PhantomData,
        )
    }
    pub fn from_raw(name: u32) -> Option<Self> {
        Some(Self(NonZeroU32::new(name)?, PhantomData))
    }
    #[inline]
    pub fn to_idx(self) -> usize {
        (self.0.get() - 1) as usize
    }
    pub fn to_raw(self) -> u32 {
        self.0.get()
    }
}

pub trait NamedObject {}

impl<Dst: GlDstType, T> SrcType<Dst> for Option<ObjectName<T>> {
    fn convert(self) -> Dst {
        Dst::from_uint(self.map_or(0, |v| v.0.get()))
    }
}
#[derive(Debug)]
pub struct NamedObjectList<T> {
    objects: Vec<NameState<T>>,
}

impl<Obj: NamedObject> NamedObjectList<Obj> {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::with_capacity(32),
        }
    }
    #[inline]
    pub(crate) fn get(&self, name: ObjectName<Obj>) -> Option<&Obj> {
        self.objects
            .get(name.to_idx())
            .and_then(|name_state| match name_state {
                NameState::Bound(ref b) => Some(b),
                _ => None,
            })
    }
    #[inline]
    pub(crate) unsafe fn get_unchecked(&self, name: ObjectName<Obj>) -> &Obj {
        // Safety: Caller ensures that the object at name exists in the list
        unsafe {
            match self.objects.get_unchecked(name.to_idx()) {
                NameState::Bound(ref b) => b,
                _ => {
                    // Safety: Caller ensures that the object at name is bound
                    debug_unreachable!(
                        "UB: Tried to get a buffer with a name that has not yet been initialized"
                    )
                }
            }
        }
    }
    #[inline]
    pub(crate) fn get_mut(&mut self, name: ObjectName<Obj>) -> Option<&mut Obj> {
        self.objects
            .get_mut(name.to_idx())
            .and_then(|name_state| match name_state {
                NameState::Bound(b) => Some(b),
                _ => None,
            })
    }
    #[inline]
    pub(crate) unsafe fn get_unchecked_mut(&mut self, name: ObjectName<Obj>) -> &mut Obj {
        // Safety: Caller ensures that the object at name exists in the list
        unsafe {
            match self.objects.get_unchecked_mut(name.to_idx()) {
                NameState::Bound(b) => b,
                _ => {
                    // Safety: Caller ensures that the object at name is bound
                    debug_unreachable!(
                        "UB: Tried to get an object with a name that has not yet been initialized"
                    )
                }
            }
        }
    }
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
        self.objects.push(NameState::Named);
        name
    }
    // Overflow is checked
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn new_obj(
        &mut self,
        create: impl FnOnce(ObjectName<Obj>) -> Obj,
    ) -> ObjectName<Obj> {
        debug_assert!(
            self.objects.len() < (u32::MAX - 1) as usize,
            "UB: OxideGL does not allow generation of more than u32::MAX object names"
        );
        // Safety: see assertion
        let name = unsafe { ObjectName::from_idx(self.objects.len()) };
        self.objects.push(NameState::Bound(create(name)));
        name
    }
    #[inline]
    pub(crate) fn is(&self, name: ObjectName<Obj>) -> bool {
        self.get(name).is_some()
    }
    pub(crate) fn delete(&mut self, name: ObjectName<Obj>) {
        if let Some(entry) = self.objects.get_mut(name.to_idx()) {
            let mut e = NameState::Empty;
            core::mem::swap(entry, &mut e);
            // make the drop explicit for clarity
            drop(e);
        }
    }
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
            debug!(target: "object alloc", "deleted {} {name:?}", type_name::<Obj>());
        }
    }
    #[inline]
    pub(crate) unsafe fn gen_obj(&mut self, n: GLsizei, names: *mut GLuint) {
        debug_assert!(!names.is_null(), "UB: object name array pointer was null");
        debug_assert!(
            names.is_aligned(),
            "UB: object name array pointer was not sufficiently aligned"
        );
        debug!(target: "object alloc", "writing {n} new {} names to {names:?}", type_name::<Obj>());
        let mut names = names.cast();
        for _ in 0..n {
            let name = self.new_name();
            // Safety: Caller ensures names is valid to write n buffer names to
            unsafe { core::ptr::write(names, name) }
            // Safety: See above, this pointer will point at most one item past the end of its allocation
            unsafe { names = names.add(1) }
        }
    }
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
        debug!(
            target: "object alloc",
            "writing {n} new initialized {} names to {names:?}",
            type_name::<Obj>()
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
    pub(crate) fn is_obj(&self, name: GLuint) -> GLboolean {
        ObjectName::from_raw(name).is_some_and(|name| self.is(name))
    }
    #[inline]
    pub(crate) fn ensure_init(
        &mut self,
        name: ObjectName<Obj>,
        default: impl Fn(ObjectName<Obj>) -> Obj + Copy,
    ) {
        if let Some(NameState::Named) = self.objects.get(name.to_idx()) {
            self.objects[name.to_idx()] = NameState::Bound(default(name));
        }
    }
}
