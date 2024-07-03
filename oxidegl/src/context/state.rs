use crate::{
    dispatch::gltypes::GLenum,
    enums::{
        GL_BACK, GL_CONTEXT_CORE_PROFILE_BIT, GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT,
        GL_CONTEXT_FLAG_NO_ERROR_BIT, GL_FILL, GL_FRONT, GL_FRONT_AND_BACK, GL_LINE, GL_POINT,
    },
};

#[derive(Debug)]
pub struct GLState {
    pub characteristics: Characteristics,
    pub point_size: f32,
    pub line_width: f32,
}

impl GLState {
    pub(crate) fn new() -> Self {
        Self {
            characteristics: Characteristics::new(),
            point_size: 1.0,
            line_width: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Characteristics {
    pub point_size_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_range: [f32; 2],
    pub line_width_granularity: f32,
    pub context_flags: GLenum,
    pub context_profile_mask: GLenum,
    pub num_extensions: u32,
}
impl Characteristics {
    pub fn new() -> Self {
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
