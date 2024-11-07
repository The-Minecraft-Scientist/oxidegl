#![allow(unused)] /* FIXME delete when implemented */
use objc2_metal::{MTLDevice, MTLTexture};
use std::any::Any;

use crate::{
    enums::{ClearBufferMask, DrawBufferMode, TextureTarget},
    ProtoObjRef,
};

use super::state::{LateInit, NamedObject, ObjectName};

pub const MAX_COLOR_ATTACHMENTS: u32 = 8;
#[derive(Debug)]
pub struct Framebuffer {
    name: ObjectName<Self>,
    draw_buffers: DrawBuffers,
    color_attachments: [Option<FramebufferAttachment>; MAX_COLOR_ATTACHMENTS as usize],
    depth_attachment: Option<FramebufferAttachment>,
    stencil_attachment: Option<FramebufferAttachment>,
}
impl Framebuffer {
    pub fn new_default(name: ObjectName<Self>) -> Self {
        Self {
            name,
            draw_buffers: DrawBuffers::new(),
            color_attachments: [const { None }; MAX_COLOR_ATTACHMENTS as usize],
            depth_attachment: None,
            stencil_attachment: None,
        }
    }
}

impl NamedObject for Framebuffer {
    type LateInitType = LateInit<Self>;
    const LATE_INIT_FUNC: fn(ObjectName<Self>) -> Self = Self::new_default;
}
/// GL object wrapping an internal drawable
pub(crate) struct RenderBuffer {
    pub(crate) name: ObjectName<Self>,
    pub(crate) drawable: InternalDrawable,
}
pub trait AttachableTexture: Any {}
impl AttachableTexture for RenderBuffer {}

// TODO uncomment when Texture is impled :3
// impl AttachableTexture for Texture {}
#[derive(Debug)]
pub struct FramebufferAttachment {
    clear_color: [f32; 4],
    clear_bitmask: ClearBufferMask,
    target: TextureTarget,
    tex_name: ObjectName<dyn AttachableTexture>,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct DrawBuffers {
    pub(crate) modes: [Option<DrawBufferMode>; MAX_COLOR_ATTACHMENTS as usize],
}
impl Default for DrawBuffers {
    fn default() -> Self {
        let mut s = Self::new();
        s.modes[0] = Some(DrawBufferMode::FrontLeft);
        s
    }
}

impl DrawBuffers {
    pub(crate) fn new() -> Self {
        Self {
            modes: [None; MAX_COLOR_ATTACHMENTS as usize],
        }
    }
    pub(crate) fn sanitize_mode(mode: DrawBufferMode) -> Option<DrawBufferMode> {
        match mode {
            DrawBufferMode::None => None,
            v => Some(v),
        }
    }

    pub(crate) fn drawbuf_iter(&self) -> impl Iterator<Item = DrawBufferMode> + '_ {
        self.modes
            .iter()
            .copied()
            .take_while(std::option::Option::is_some)
            .flatten()
    }
}
/// A drawable for usage in rendering
#[derive(Debug, Clone)]
pub(crate) struct InternalDrawable {
    pub(crate) dimensions: (u32, u32),
    pub(crate) tex: ProtoObjRef<dyn MTLTexture>,
}
impl InternalDrawable {
    pub(crate) fn new(color: ProtoObjRef<dyn MTLTexture>, dimensions: (u32, u32)) -> Self {
        Self {
            dimensions,
            tex: color,
        }
    }
}
