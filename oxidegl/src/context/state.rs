use bitflags::bitflags;

use crate::{
    dispatch::gl_types::GLenum,
    enums::{
        ClearBufferMask, GL_CONTEXT_CORE_PROFILE_BIT, GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT,
        GL_CONTEXT_FLAG_NO_ERROR_BIT,
    },
};

use super::commands::buffers::{BufferList, BufferName};

#[derive(Debug)]
pub struct GLState {
    pub(crate) characteristics: Characteristics,
    pub(crate) bindings: BufferBindings,
    pub(crate) buffer_list: BufferList,
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
    pub(crate) array: Option<BufferName>,
    /// Atomic counter storage
    pub(crate) atomic_counter: [Option<BufferName>; MAX_ATOMIC_COUNTER_BUFFER_BINDINGS],
    /// Buffer copy source
    pub(crate) copy_read: Option<BufferName>,
    /// Buffer copy destination
    pub(crate) copy_write: Option<BufferName>,
    /// Indirect compute dispatch commands
    pub(crate) dispatch_indirect: Option<BufferName>,
    /// Indirect draw command arguments
    pub(crate) draw_indirect: Option<BufferName>,
    /// Vertex array indices
    pub(crate) element_array: Option<BufferName>,
    /// Draw parameters
    pub(crate) parameter: Option<BufferName>,
    /// Pixel read target
    pub(crate) pixel_pack: Option<BufferName>,
    /// Texture data source
    pub(crate) pixel_unpack: Option<BufferName>,
    /// Query results
    pub(crate) query: Option<BufferName>,
    /// Shader storage buffers
    pub(crate) shader_storage: [Option<BufferName>; MAX_SHADER_STORAGE_BUFFER_BINDINGS],
    /// Texture data buffer
    pub(crate) texture: Option<BufferName>,
    /// Transform feedback result buffers
    pub(crate) transform_feedback: [Option<BufferName>; MAX_TRANSFORM_FEEDBACK_BUFFER_BINDINGS],
    /// Uniform storage buffers
    pub(crate) uniform: [Option<BufferName>; MAX_UNIFORM_BUFFER_BINDINGS],
}

impl GLState {
    pub fn default() -> Self {
        GLState {
            characteristics: Characteristics::new(),

            bindings: BufferBindings::default(),
            buffer_list: BufferList::new(),
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
