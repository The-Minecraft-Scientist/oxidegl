use std::{fmt::Debug, ops::Deref, ptr};

use ahash::HashSet;
use objc2_metal::{MTLBlendFactor, MTLBlendOperation};

use crate::{
    bitflag_bits,
    dispatch::gl_types::GLenum,
    enums::{
        BlendEquationModeEXT, BlendingFactor, ClearBufferMask, DepthFunction, StencilFunction,
        StencilOp, TriangleFace, GL_CONTEXT_CORE_PROFILE_BIT,
        GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT, GL_CONTEXT_FLAG_NO_ERROR_BIT,
    },
};

use super::{
    commands::{buffer::Buffer, vao::Vao},
    debug::DebugState,
    framebuffer::{DrawBuffers, Framebuffer, MAX_COLOR_ATTACHMENTS},
    gl_object::{NamedObjectList, ObjectName},
    program::Program,
    shader::Shader,
};

#[derive(Debug, Default)]
pub(crate) struct GLState {
    // Static/immutable properties, mostly for glGet. Should probably be turned into constants
    pub(crate) characteristics: Characteristics,
    /// Tracks whether a given GL capability is active or not
    pub(crate) caps: Capabilities,

    /// Current state of buffer bindings to this context
    pub(crate) buffer_bindings: BufferBindings,
    /// List of buffer object states
    pub(crate) buffer_list: NamedObjectList<Buffer>,

    /// List of VAO states
    pub(crate) vao_list: NamedObjectList<Vao>,
    /// The current VAO to render with
    pub(crate) vao_binding: Option<ObjectName<Vao>>,

    /// List of shader object states
    pub(crate) shader_list: NamedObjectList<Shader>,
    /// Shaders that are queued for deletion when their reference count reaches 0
    pub(crate) shader_deletion_queue: HashSet<ObjectName<Shader>>,

    /// List of shader program states
    pub(crate) program_list: NamedObjectList<Program>,
    /// Programs that are queued for deletion when their reference count reaches 0
    pub(crate) program_deletion_queue: HashSet<ObjectName<Program>>,
    /// Current program to render with
    pub(crate) program_binding: Option<ObjectName<Program>>,

    /// List of framebuffer object states
    pub(crate) framebufer_list: NamedObjectList<Framebuffer>,
    /// The current framebuffer to render to (None: default FB)
    pub(crate) framebuffer_binding: Option<ObjectName<Framebuffer>>,
    /// draw buffer/attachment tracking for the default framebuffer
    pub(crate) default_draw_buffers: DrawBuffers,

    //TODO: these should be arrays in order to support viewport arrays
    pub(crate) scissor_box: PixelAlignedRect,
    pub(crate) viewport: PixelAlignedRect,

    pub(crate) clear_values: ClearState,
    pub(crate) stencil: StencilState,
    pub(crate) blend: BlendState,
    pub(crate) writemasks: Writemasks,
    pub(crate) cull_face_mode: TriangleFace,

    pub(crate) depth_func: DepthFunction,

    /// storage for the debug state associated with this context (if it is not the current context). If this context is
    /// current, you'll need to use [`with_debug_state`](super::debug::with_debug_state) or
    /// [`with_debug_state_mut`](super::debug::with_debug_state_mut) to interact with the current debug state
    pub(crate) debug_state_holder: DebugStateHolder,
}

#[derive(Debug)]
pub(crate) struct DebugStateHolder(pub Option<DebugState>);
impl Default for DebugStateHolder {
    #[inline]
    fn default() -> Self {
        Self(Some(DebugState::default()))
    }
}

impl Default for DepthFunction {
    #[inline]
    fn default() -> Self {
        DepthFunction::Less
    }
}
impl Default for TriangleFace {
    #[inline]
    fn default() -> Self {
        TriangleFace::Back
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ClearState {
    pub(crate) color: [f32; 4],
    pub(crate) depth: f32,
    pub(crate) stencil: u32,
    pub(crate) mask: ClearBufferMask,
}
impl Default for ClearState {
    #[inline]
    fn default() -> Self {
        Self {
            color: [0.0; 4],
            depth: 1.0,
            stencil: 0,
            mask: ClearBufferMask::empty(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct StencilState {
    pub(crate) front: StencilFaceState,
    pub(crate) back: StencilFaceState,
}

#[derive(Clone, Copy, Debug)]
pub struct StencilFaceState {
    /// Comparison function used to decide whether to discard the fragment or not
    pub(crate) func: StencilFunction,
    /// Bitmask whose least-significant N bits (where N is the bit-depth of the stencil component of the stencil buffer's pixel format)
    /// are bitwise AND-ed together with both the stencil value and reference before comparison
    pub(crate) mask: u32,
    /// reference value for comparison
    pub(crate) reference: u32,
    pub(crate) fail_action: StencilOp,
    pub(crate) depth_fail_action: StencilOp,
    pub(crate) depth_pass_action: StencilOp,
}
impl Default for StencilFaceState {
    #[inline]
    fn default() -> Self {
        Self {
            func: StencilFunction::Always,
            mask: u32::MAX,
            reference: 0,
            fail_action: StencilOp::Keep,
            depth_fail_action: StencilOp::Keep,
            depth_pass_action: StencilOp::Keep,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DrawbufferBlendState {
    pub(crate) blend_enabled: bool,
    pub(crate) src_rgb: BlendingFactor,
    pub(crate) src_alpha: BlendingFactor,
    pub(crate) dst_rgb: BlendingFactor,
    pub(crate) dst_alpha: BlendingFactor,
    pub(crate) eq_rgb: BlendEquationModeEXT,
    pub(crate) eq_alpha: BlendEquationModeEXT,
}
impl Default for DrawbufferBlendState {
    #[inline]
    fn default() -> Self {
        Self {
            blend_enabled: false,
            src_rgb: BlendingFactor::One,
            src_alpha: BlendingFactor::One,
            dst_rgb: BlendingFactor::Zero,
            dst_alpha: BlendingFactor::Zero,
            eq_rgb: BlendEquationModeEXT::FuncAdd,
            eq_alpha: BlendEquationModeEXT::FuncAdd,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct BlendState {
    pub(crate) blend_color: [f32; 4],
    pub(crate) drawbuffer_states: [DrawbufferBlendState; MAX_COLOR_ATTACHMENTS as usize],
}

impl From<BlendingFactor> for MTLBlendFactor {
    #[inline]
    fn from(value: BlendingFactor) -> Self {
        #[allow(clippy::enum_glob_use)]
        use BlendingFactor::*;

        match value {
            // constants
            Zero => Self::Zero,
            One => Self::One,

            // source color/alpha
            SrcColor => Self::SourceColor,
            OneMinusSrcColor => Self::OneMinusSourceColor,
            SrcAlpha => Self::SourceAlpha,
            OneMinusSrcAlpha => Self::OneMinusSourceAlpha,
            SrcAlphaSaturate => Self::SourceAlphaSaturated,

            // destination color/alpha
            DstColor => Self::DestinationColor,
            OneMinusDstColor => Self::OneMinusDestinationColor,
            DstAlpha => Self::DestinationAlpha,
            OneMinusDstAlpha => Self::OneMinusDestinationAlpha,

            // constant color/alpha
            ConstantColor => Self::BlendColor,
            OneMinusConstantColor => Self::OneMinusBlendColor,
            ConstantAlpha => Self::BlendAlpha,
            OneMinusConstantAlpha => Self::OneMinusBlendAlpha,

            // source #2 color/alpha (for dual-source blending)
            Src1Alpha => Self::Source1Alpha,
            Src1Color => Self::Source1Color,
            OneMinusSrc1Color => Self::OneMinusSource1Color,
            OneMinusSrc1Alpha => Self::OneMinusSource1Alpha,
        }
    }
}
impl From<BlendEquationModeEXT> for MTLBlendOperation {
    #[inline]
    fn from(value: BlendEquationModeEXT) -> Self {
        match value {
            BlendEquationModeEXT::FuncAdd => Self::Add,
            BlendEquationModeEXT::FuncReverseSubtract => Self::ReverseSubtract,
            BlendEquationModeEXT::FuncSubtract => Self::Subtract,
            BlendEquationModeEXT::Min => Self::Min,
            BlendEquationModeEXT::Max => Self::Max,
        }
    }
}

bitflag_bits! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Capabilities: u64 bits: {

        // don't include indexed caps
        // BLEND: 0,
        // SCISSOR_TEST: 19,

        MULTISAMPLE: 1,
        SAMPLE_ALPHA_TO_COVERAGE: 2,
        SAMPLE_ALPHA_TO_ONE: 3,
        SAMPLE_COVERAGE: 4,
        RASTERIZER_DISCARD: 5,
        FRAMEBUFFER_SRGB: 6,
        DEPTH_CLAMP: 7,
        TEXTURE_CUBE_MAP_SEAMLESS: 8,
        SAMPLE_MASK: 9,
        SAMPLE_SHADING: 10,
        DEBUG_OUTPUT_SYNCHRONOUS: 11,
        DEBUG_OUTPUT: 12,
        LINE_SMOOTH: 13,
        POLYGON_SMOOTH: 14,
        CULL_FACE: 15,
        DEPTH_TEST: 16,
        STENCIL_TEST: 17,
        DITHER: 18,
        POINT_SMOOTH: 20,
        LINE_STIPPLE: 21,
        POLYGON_STIPPLE: 22,
        LIGHTING: 23,
        COLOR_MATERIAL: 24,
        FOG: 25,
        NORMALIZE: 26,
        ALPHA_TEST: 27,
        TEXTURE_GEN_S: 28,
        TEXTURE_GEN_T: 29,
        TEXTURE_GEN_R: 30,
        TEXTURE_GEN_Q: 31,
        AUTO_NORMAL: 32,
        COLOR_LOGIC_OP: 33,
        POLYGON_OFFSET_POINT: 34,
        POLYGON_OFFSET_LINE: 35,
        POLYGON_OFFSET_FILL: 36,
        INDEX_LOGIC_OP: 37,
        NORMAL_ARRAY: 38,
        COLOR_ARRAY: 39,
        INDEX_ARRAY: 40,
        TEXTURE_COORD_ARRAY: 41,
        EDGE_FLAG_ARRAY: 42,
        PROGRAM_POINT_SIZE: 43,
        PRIMITIVE_RESTART: 44,
        PRIMITIVE_RESTART_FIXED_INDEX: 45,
    }

}
impl Capabilities {
    /// Returns true if any set bit in `cap` is also set in `self`
    #[inline]
    pub(crate) fn is_any_enabled(self, cap: Self) -> bool {
        self.intersects(cap)
    }
    #[inline]
    pub(crate) fn disable(&mut self, cap: Self) {
        *self = self.difference(cap);
    }
    #[inline]
    pub(crate) fn enable(&mut self, cap: Self) {
        *self = self.union(cap);
    }
    #[inline]
    pub(crate) fn set_to(&mut self, to: bool, cap: Self) -> bool {
        let mut r = self.intersects(cap);
        if to {
            r = !r;
            self.enable(cap);
        } else {
            self.disable(cap);
        }
        r
    }
}
impl Default for Capabilities {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub(crate) struct PixelAlignedRect {
    // INVARIANT do not change layout without changing Deref impl!
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
impl Deref for PixelAlignedRect {
    type Target = [u32; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: repr(C) layout guarantees of PixelAlignedRef give it an identical repr to [u32; 4]
        // reference creation is infallible because the pointer is originally derived from a reference and cannot be null
        unsafe {
            ptr::from_ref(self)
                .cast::<[u32; 4]>()
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: usize = 16;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: usize = 16;
pub const MAX_TRANSFORM_FEEDBACK_BUFFER_BINDINGS: usize = 16;
pub const MAX_UNIFORM_BUFFER_BINDINGS: usize = 16;

/// Keeps track of all buffer bindings in this OpenGL context
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

#[derive(Debug, Copy, Clone)]
pub(crate) struct Writemasks {
    pub(crate) color: [ColorWriteMask; MAX_COLOR_ATTACHMENTS as usize],
    pub(crate) depth: bool,
    pub(crate) stencil_front: u32,
    pub(crate) stencil_back: u32,
}
impl Default for Writemasks {
    fn default() -> Self {
        Self {
            color: [[true; 4].into(); MAX_COLOR_ATTACHMENTS as usize],
            depth: true,
            // all bits set for bitmasks
            stencil_front: u32::MAX,
            stencil_back: u32::MAX,
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ColorWriteMask {
    pub(crate) red: bool,
    pub(crate) green: bool,
    pub(crate) blue: bool,
    pub(crate) alpha: bool,
}
// both of these impls should be inlined to no-ops, no need to transmute
impl From<ColorWriteMask> for [bool; 4] {
    #[inline]
    fn from(value: ColorWriteMask) -> Self {
        [value.red, value.green, value.blue, value.alpha]
    }
}
impl From<[bool; 4]> for ColorWriteMask {
    #[inline]
    fn from(value: [bool; 4]) -> Self {
        Self {
            red: value[0],
            green: value[1],
            blue: value[2],
            alpha: value[3],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Characteristics {
    pub(crate) point_size: f32,
    pub(crate) point_size_range: [f32; 2],
    pub(crate) point_size_granularity: f32,
    pub(crate) line_width: f32,
    pub(crate) context_flags: GLenum,
    pub(crate) context_profile_mask: GLenum,
    pub(crate) num_extensions: u32,
}
impl Default for Characteristics {
    fn default() -> Self {
        Self {
            point_size_range: [1.0, 1.0],
            point_size_granularity: 0.0001,
            context_flags: GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT | GL_CONTEXT_FLAG_NO_ERROR_BIT,
            context_profile_mask: GL_CONTEXT_CORE_PROFILE_BIT,
            num_extensions: 1,
            point_size: 1.0,
            line_width: 1.0,
        }
    }
}
