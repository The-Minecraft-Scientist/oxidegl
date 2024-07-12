use bitflags::bitflags;

use crate::{
    dispatch::gl_types::GLenum,
    enums::{
        GL_CONTEXT_CORE_PROFILE_BIT, GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT,
        GL_CONTEXT_FLAG_NO_ERROR_BIT,
    },
};

#[derive(Debug)]
pub struct GLState {
    pub(crate) characteristics: Characteristics,
    pub(crate) point_size: f32,
    pub(crate) line_width: f32,
    pub(crate) clear_color: [f32; 4],
    pub(crate) clear_depth: f32,
}
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NeedsRefreshBits: u32 {
        const RENDER_PASS = 0b1;
        const SAMPLERS = 0b10;
    }
}

impl GLState {
    pub(crate) fn default() -> Self {
        Self {
            characteristics: Characteristics::default(),
            point_size: 1.0,
            line_width: 1.0,
            clear_color: [0.0; 4],
            clear_depth: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn default() -> Self {
        Self {
            point_size_range: [1.0, 1.0],
            point_size_granularity: 0.0001,
            line_width_range: [1.0, 1.0],
            line_width_granularity: 0.0001,
            context_flags: GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT | GL_CONTEXT_FLAG_NO_ERROR_BIT,
            context_profile_mask: GL_CONTEXT_CORE_PROFILE_BIT,
            num_extensions: 0,
        }
    }
}
