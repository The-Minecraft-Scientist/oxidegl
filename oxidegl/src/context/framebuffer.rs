use std::any::Any;

use objc2_metal::MTLTexture;

use crate::{
    enums::{ClearBufferMask, TextureTarget},
    ProtoObjRef,
};

use super::state::{NamedObject, ObjectName};

pub const MAX_COLOR_ATTACHMENTS: u32 = 8;
#[derive(Debug)]
pub struct Framebuffer {
    name: ObjectName<Self>,
    internal: FramebufferInternal,
}
#[derive(Debug)]
pub struct FramebufferInternal {
    color_attachments: [Option<FramebufferAttachment>; MAX_COLOR_ATTACHMENTS as usize],
    depth_attachment: Option<FramebufferAttachment>,
    stencil_attachment: Option<FramebufferAttachment>,
}
impl Framebuffer {
    pub fn new_named(name: ObjectName<Self>) -> Self {
        Self {
            name,
            internal: FramebufferInternal::new(),
        }
    }
}
impl FramebufferInternal {
    pub fn new() -> Self {
        Self {
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
