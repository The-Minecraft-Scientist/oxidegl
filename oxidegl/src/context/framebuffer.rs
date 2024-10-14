use std::any::Any;

use objc2_metal::{MTLDevice, MTLTexture};
use objc2_quartz_core::CAMetalDrawable;

use crate::{
    enums::{ClearBufferMask, DrawBufferMode, TextureTarget},
    ProtoObjRef,
};

use super::state::{NamedObject, ObjectName};

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
    pub fn new_named(name: ObjectName<Self>) -> Self {
        Self {
            name,
            draw_buffers: DrawBuffers::new(),
            color_attachments: [const { None }; MAX_COLOR_ATTACHMENTS as usize],
            depth_attachment: None,
            stencil_attachment: None,
        }
    }
}
impl NamedObject for Framebuffer {}
/// A Texture with extra steps (Metal doesn't support doing fancy things for render-only targets)
pub struct RenderBuffer {
    name: ObjectName<Self>,
    mtl: ProtoObjRef<dyn MTLTexture>,
}
pub trait AttachableTexture: Any {}
impl AttachableTexture for RenderBuffer {}
#[derive(Debug)]
pub struct FramebufferAttachment {
    clear_color: [f32; 4],
    clear_bitmask: ClearBufferMask,
    target: TextureTarget,
    tex_name: ObjectName<dyn AttachableTexture>,
}
#[derive(Debug, Clone, Copy)]
pub struct DrawBuffers {
    draw_buffers: [Option<DrawBufferMode>; MAX_COLOR_ATTACHMENTS as usize],
}

impl DrawBuffers {
    pub(crate) fn new() -> Self {
        Self {
            draw_buffers: [None; MAX_COLOR_ATTACHMENTS as usize],
        }
    }
    pub(crate) fn new_defaultfb() -> Self {
        let mut s = Self::new();
        s.draw_buffers[0] = Some(DrawBufferMode::FrontLeft);
        s
    }
    pub(crate) fn drawbuf_iter(&self) -> impl Iterator<Item = DrawBufferMode> + '_ {
        self.draw_buffers
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
    pub(crate) color: ProtoObjRef<dyn MTLTexture>,
    pub(crate) depth: Option<ProtoObjRef<dyn MTLTexture>>,
    pub(crate) stencil: Option<ProtoObjRef<dyn MTLTexture>>,
}
impl InternalDrawable {
    #[expect(clippy::cast_possible_truncation)]
    pub(crate) fn with_ca_metal_drawable(&mut self, d: &ProtoObjRef<dyn CAMetalDrawable>) {
        let tex = unsafe { d.texture() };
        debug_assert_eq!(self.dimensions, (tex.width() as u32, tex.height() as u32),);
        self.color = tex;
    }
    pub(crate) fn new(color: ProtoObjRef<dyn MTLTexture>, dimensions: (u32, u32)) -> Self {
        Self {
            dimensions,
            color,
            depth: None,
            stencil: None,
        }
    }
}
