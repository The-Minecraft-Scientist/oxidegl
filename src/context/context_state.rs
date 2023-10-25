use crate::{enums::*, gl::gltypes::GLenum};
use std::cell::RefCell;

use super::{get::OxideGLItemSingle, metal_view::ContextMetalComponents, NSViewPtr};

///Marker trait for repr(u32) GLenum equivalents
pub unsafe trait GlEnum {}
impl<T: GlEnum> From<T> for OxideGLItemSingle {
    fn from(val: T) -> Self {
        let ret = unsafe { *(&val as *const T as *const u32) }.into();
        std::mem::forget(val);
        ret
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Point = GL_POINT,
    Line = GL_LINE,
    Fill = GL_FILL,
}
unsafe impl GlEnum for PolygonMode {}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullFaceMode {
    Front = GL_FRONT,
    Back = GL_BACK,
    FrontAndBack = GL_FRONT_AND_BACK,
}
unsafe impl GlEnum for CullFaceMode {}
#[derive(Debug)]
pub struct OxideGLContextState {
    //BEGIN OxideGL context state
    pub metal_components: ContextMetalComponents,
    //BEGIN OpenGL context state.
    pub characteristics: Characteristics,
    pub point_size: f32,
    pub line_width: f32,
    pub polygon_mode: PolygonMode,
    pub cull_face_mode: CullFaceMode,
}

impl OxideGLContextState {
    pub(crate) fn new(view: NSViewPtr) -> Self {
        Self {
            metal_components: ContextMetalComponents::new(view),
            characteristics: Characteristics::new(),
            point_size: 1.0,
            line_width: 1.0,
            polygon_mode: PolygonMode::Fill,
            cull_face_mode: CullFaceMode::Back,
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
        }
    }
}
