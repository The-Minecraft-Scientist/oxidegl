use crate::{
    context::{platform::Dirty, state::Capabilities, Context},
    dispatch::gl_types::GLuint,
    enums::EnableCap,
};

/// ### Parameters
/// `cap`
///
/// > Specifies a symbolic constant indicating a GL capability.
///
/// `index`
///
/// > Specifies the index of the switch to disable (for [**glEnablei**](crate::context::Context::oxidegl_enablei)
/// > and [**glDisablei**](crate::context::Context::oxidegl_disablei) only).
///
/// ### Description
/// [**glEnable**](crate::context::Context::oxidegl_enable) and [**glDisable**](crate::context::Context::oxidegl_disable)
/// enable and disable various capabilities. Use [**glIsEnabled**](crate::context::Context::oxidegl_is_enabled)
/// or [**glGet**](crate::context::Context::oxidegl_get) to determine the current
/// setting of any capability. The initial value for each capability with the
/// exception of [`GL_DITHER`](crate::enums::GL_DITHER) and [`GL_MULTISAMPLE`](crate::enums::GL_MULTISAMPLE)
/// is [`GL_FALSE`](crate::enums::GL_FALSE). The initial value for [`GL_DITHER`](crate::enums::GL_DITHER)
/// and [`GL_MULTISAMPLE`](crate::enums::GL_MULTISAMPLE) is [`GL_TRUE`](crate::enums::GL_TRUE).
///
/// Both [**glEnable**](crate::context::Context::oxidegl_enable) and [**glDisable**](crate::context::Context::oxidegl_disable)
/// take a single argument, `cap`, which can assume one of the following values:
///
/// Some of the GL's capabilities are indexed. [**glEnablei**](crate::context::Context::oxidegl_enablei)
/// and [**glDisablei**](crate::context::Context::oxidegl_disablei) enable
/// and disable indexed capabilities.
///
/// [`GL_BLEND`](crate::enums::GL_BLEND)
///
///
/// > If enabled, blend the computed fragment color values with the values in
/// > the color buffers. See [**glBlendFunc**](crate::context::Context::oxidegl_blend_func).
///
/// [`GL_CLIP_DISTANCE`](crate::enums::GL_CLIP_DISTANCE) *i*
///
///
/// > If enabled, clip geometry against user-defined half space *i*.
///
/// [`GL_COLOR_LOGIC_OP`](crate::enums::GL_COLOR_LOGIC_OP)
///
///
/// > If enabled, apply the currently selected logical operation to the computed
/// > fragment color and color buffer values. See [**glLogicOp**](crate::context::Context::oxidegl_logic_op).
///
/// [`GL_CULL_FACE`](crate::enums::GL_CULL_FACE)
///
///
/// > If enabled, cull polygons based on their winding in window coordinates.
/// > See [**glCullFace**](crate::context::Context::oxidegl_cull_face).
///
/// [`GL_DEBUG_OUTPUT`](crate::enums::GL_DEBUG_OUTPUT)
///
///
/// > If enabled, debug messages are produced by a debug context. When disabled,
/// > the debug message log is silenced. Note that in a non-debug context, very
/// > few, if any messages might be produced, even when [`GL_DEBUG_OUTPUT`](crate::enums::GL_DEBUG_OUTPUT)
/// > is enabled.
///
/// [`GL_DEBUG_OUTPUT_SYNCHRONOUS`](crate::enums::GL_DEBUG_OUTPUT_SYNCHRONOUS)
///
///
/// > If enabled, debug messages are produced synchronously by a debug context.
/// > If disabled, debug messages may be produced asynchronously. In particular,
/// > they may be delayed relative to the execution of GL commands, and the debug
/// > callback function may be called from a thread other than that in which
/// > the commands are executed. See [**glDebugMessageCallback**](crate::context::Context::oxidegl_debug_message_callback).
///
/// [`GL_DEPTH_CLAMP`](crate::enums::GL_DEPTH_CLAMP)
///
///
/// > If enabled, the `[inlineq]` [**glDepthRange**](crate::context::Context::oxidegl_depth_range).
///
/// [`GL_DEPTH_TEST`](crate::enums::GL_DEPTH_TEST)
///
///
/// > If enabled, do depth comparisons and update the depth buffer. Note that
/// > even if the depth buffer exists and the depth mask is non-zero, the depth
/// > buffer is not updated if the depth test is disabled. See [**glDepthFunc**](crate::context::Context::oxidegl_depth_func)
/// > and [**glDepthRange**](crate::context::Context::oxidegl_depth_range).
///
/// [`GL_DITHER`](crate::enums::GL_DITHER)
///
///
/// > If enabled, dither color components or indices before they are written
/// > to the color buffer.
///
/// [`GL_FRAMEBUFFER_SRGB`](crate::enums::GL_FRAMEBUFFER_SRGB)
///
///
/// > If enabled and the value of [`GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING`](crate::enums::GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING)
/// > for the framebuffer attachment corresponding to the destination buffer
/// > is [`GL_SRGB`](crate::enums::GL_SRGB), the R, G, and B destination color
/// > values (after conversion from fixed-point to floating-point) are considered
/// > to be encoded for the sRGB color space and hence are linearized prior to
/// > their use in blending.
///
/// [`GL_LINE_SMOOTH`](crate::enums::GL_LINE_SMOOTH)
///
///
/// > If enabled, draw lines with correct filtering. Otherwise, draw aliased
/// > lines. See [**glLineWidth**](crate::context::Context::oxidegl_line_width).
///
/// [`GL_MULTISAMPLE`](crate::enums::GL_MULTISAMPLE)
///
///
/// > If enabled, use multiple fragment samples in computing the final color
/// > of a pixel. See [**glSampleCoverage**](crate::context::Context::oxidegl_sample_coverage).
///
/// [`GL_POLYGON_OFFSET_FILL`](crate::enums::GL_POLYGON_OFFSET_FILL)
///
///
/// > If enabled, and if the polygon is rendered in [`GL_FILL`](crate::enums::GL_FILL)
/// > mode, an offset is added to depth values of a polygon's fragments before
/// > the depth comparison is performed. See [**glPolygonOffset**](crate::context::Context::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_LINE`](crate::enums::GL_POLYGON_OFFSET_LINE)
///
///
/// > If enabled, and if the polygon is rendered in [`GL_LINE`](crate::enums::GL_LINE)
/// > mode, an offset is added to depth values of a polygon's fragments before
/// > the depth comparison is performed. See [**glPolygonOffset**](crate::context::Context::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_OFFSET_POINT`](crate::enums::GL_POLYGON_OFFSET_POINT)
///
///
/// > If enabled, an offset is added to depth values of a polygon's fragments
/// > before the depth comparison is performed, if the polygon is rendered in
/// > [`GL_POINT`](crate::enums::GL_POINT) mode. See [**glPolygonOffset**](crate::context::Context::oxidegl_polygon_offset).
///
/// [`GL_POLYGON_SMOOTH`](crate::enums::GL_POLYGON_SMOOTH)
///
///
/// > If enabled, draw polygons with proper filtering. Otherwise, draw aliased
/// > polygons. For correct antialiased polygons, an alpha buffer is needed and
/// > the polygons must be sorted front to back.
///
/// [`GL_PRIMITIVE_RESTART`](crate::enums::GL_PRIMITIVE_RESTART)
///
///
/// > Enables primitive restarting. If enabled, any one of the draw commands
/// > which transfers a set of generic attribute array elements to the GL will
/// > restart the primitive when the index of the vertex is equal to the primitive
/// > restart index. See [**glPrimitiveRestartIndex**](crate::context::Context::oxidegl_primitive_restart_index).
///
/// [`GL_PRIMITIVE_RESTART_FIXED_INDEX`](crate::enums::GL_PRIMITIVE_RESTART_FIXED_INDEX)
///
///
/// > Enables primitive restarting with a fixed index. If enabled, any one of
/// > the draw commands which transfers a set of generic attribute array elements
/// > to the GL will restart the primitive when the index of the vertex is equal
/// > to the fixed primitive index for the specified index type. The fixed index
/// > is equal to `[inlineq]` *n* is equal to 8 for [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
/// > 16 for [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT) and 32 for
/// > [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
///
/// [`GL_RASTERIZER_DISCARD`](crate::enums::GL_RASTERIZER_DISCARD)
///
///
/// > If enabled, primitives are discarded after the optional transform feedback
/// > stage, but before rasterization. Furthermore, when enabled, [**glClear**](crate::context::Context::oxidegl_clear),
/// > [**glClearBufferData**](crate::context::Context::oxidegl_clear_buffer_data),
/// > [**glClearBufferSubData**](crate::context::Context::oxidegl_clear_buffer_sub_data),
/// > [**glClearTexImage**](crate::context::Context::oxidegl_clear_tex_image),
/// > and [**glClearTexSubImage**](crate::context::Context::oxidegl_clear_tex_sub_image)
/// > are ignored.
///
/// [`GL_SAMPLE_ALPHA_TO_COVERAGE`](crate::enums::GL_SAMPLE_ALPHA_TO_COVERAGE)
///
///
/// > If enabled, compute a temporary coverage value where each bit is determined
/// > by the alpha value at the corresponding sample location. The temporary
/// > coverage value is then ANDed with the fragment coverage value.
///
/// [`GL_SAMPLE_ALPHA_TO_ONE`](crate::enums::GL_SAMPLE_ALPHA_TO_ONE)
///
///
/// > If enabled, each sample alpha value is replaced by the maximum representable
/// > alpha value.
///
/// [`GL_SAMPLE_COVERAGE`](crate::enums::GL_SAMPLE_COVERAGE)
///
///
/// > If enabled, the fragment's coverage is ANDed with the temporary coverage
/// > value. If [`GL_SAMPLE_COVERAGE_INVERT`](crate::enums::GL_SAMPLE_COVERAGE_INVERT)
/// > is set to [`GL_TRUE`](crate::enums::GL_TRUE), invert the coverage value.
/// > See [**glSampleCoverage**](crate::context::Context::oxidegl_sample_coverage).
///
/// [`GL_SAMPLE_SHADING`](crate::enums::GL_SAMPLE_SHADING)
///
///
/// > If enabled, the active fragment shader is run once for each covered sample,
/// > or at fraction of this rate as determined by the current value of [`GL_MIN_SAMPLE_SHADING_VALUE`](crate::enums::GL_MIN_SAMPLE_SHADING_VALUE).
/// > See [**glMinSampleShading**](crate::context::Context::oxidegl_min_sample_shading).
///
/// [`GL_SAMPLE_MASK`](crate::enums::GL_SAMPLE_MASK)
///
///
/// > If enabled, the sample coverage mask generated for a fragment during rasterization
/// > will be ANDed with the value of [`GL_SAMPLE_MASK_VALUE`](crate::enums::GL_SAMPLE_MASK_VALUE)
/// > before shading occurs. See [**glSampleMaski**](crate::context::Context::oxidegl_sample_maski).
///
/// [`GL_SCISSOR_TEST`](crate::enums::GL_SCISSOR_TEST)
///
///
/// > If enabled, discard fragments that are outside the scissor rectangle. See
/// > [**glScissor**](crate::context::Context::oxidegl_scissor).
///
/// [`GL_STENCIL_TEST`](crate::enums::GL_STENCIL_TEST)
///
///
/// > If enabled, do stencil testing and update the stencil buffer. See [**glStencilFunc**](crate::context::Context::oxidegl_stencil_func)
/// > and [**glStencilOp**](crate::context::Context::oxidegl_stencil_op).
///
/// [`GL_TEXTURE_CUBE_MAP_SEAMLESS`](crate::enums::GL_TEXTURE_CUBE_MAP_SEAMLESS)
///
///
/// > If enabled, cubemap textures are sampled such that when linearly sampling
/// > from the border between two adjacent faces, texels from both faces are
/// > used to generate the final sample value. When disabled, texels from only
/// > a single face are used to construct the final sample value.
///
/// [`GL_PROGRAM_POINT_SIZE`](crate::enums::GL_PROGRAM_POINT_SIZE)
///
///
/// > If enabled and a vertex or geometry shader is active, then the derived
/// > point size is taken from the (potentially clipped) shader builtin [`gl_PointSize`](crate::enums::gl_PointSize)
/// > and clamped to the implementation-dependent point size range.
///
/// ### Notes
/// [`GL_PRIMITIVE_RESTART`](crate::enums::GL_PRIMITIVE_RESTART) is available
/// only if the GL version is 3.1 or greater.
///
/// [`GL_TEXTURE_CUBE_MAP_SEAMLESS`](crate::enums::GL_TEXTURE_CUBE_MAP_SEAMLESS)
/// is available only if the GL version is 3.2 or greater.
///
/// [`GL_PRIMITIVE_RESTART_FIXED_INDEX`](crate::enums::GL_PRIMITIVE_RESTART_FIXED_INDEX)
/// is available only if the GL version is 4.3 or greater.
///
/// [`GL_DEBUG_OUTPUT`](crate::enums::GL_DEBUG_OUTPUT) and [`GL_DEBUG_OUTPUT_SYNCHRONOUS`](crate::enums::GL_DEBUG_OUTPUT_SYNCHRONOUS)
/// are available only if the GL version is 4.3 or greater.
///
/// Any token accepted by [**glEnable**](crate::context::Context::oxidegl_enable)
/// or [**glDisable**](crate::context::Context::oxidegl_disable) is also accepted
/// by [**glEnablei**](crate::context::Context::oxidegl_enablei) and [**glDisablei**](crate::context::Context::oxidegl_disablei),
/// but if the capability is not indexed, the maximum value that `index` may
/// take is zero.
///
/// In general, passing an indexed capability to [**glEnable**](crate::context::Context::oxidegl_enable)
/// or [**glDisable**](crate::context::Context::oxidegl_disable) will enable
/// or disable that capability for all indices, resepectively.
///
/// ### Associated Gets
/// [**glIsEnabled**](crate::context::Context::oxidegl_is_enabled)
///
/// [**glGet**](crate::context::Context::oxidegl_get)
impl Context {
    #[inline]
    fn set_unindexed_cap_internal(&mut self, cap: EnableCap, state: bool) {
        // TODO: track dirty caps in platform state, platform is better informed about what exactly a cap state change
        // needs to be lowered to
        let bit = match cap {
            EnableCap::LineSmooth => todo!(),
            EnableCap::PolygonSmooth => todo!(),
            EnableCap::CullFace => (Dirty::UPDATE_RENDER_ENCODER, Capabilities::CULL_FACE),
            EnableCap::DepthTest => (
                Dirty::NEW_RENDER_ENCODER | Dirty::NEW_RENDER_PIPELINE,
                Capabilities::DEPTH_TEST,
            ),
            EnableCap::StencilTest => todo!(),
            EnableCap::Dither => todo!(),
            EnableCap::Blend => todo!(),
            EnableCap::ScissorTest => todo!(),
            EnableCap::Texture1D => todo!(),
            EnableCap::Texture2D => todo!(),
            EnableCap::ColorLogicOp => todo!(),
            EnableCap::PolygonOffsetPoint => todo!(),
            EnableCap::PolygonOffsetLine => todo!(),
            EnableCap::PolygonOffsetFill => todo!(),
            EnableCap::Multisample => todo!(),
            EnableCap::SampleAlphaToCoverage => todo!(),
            EnableCap::SampleAlphaToOne => todo!(),
            EnableCap::SampleCoverage => todo!(),
            EnableCap::TextureCubeMap => todo!(),
            EnableCap::ClipDistance0 => todo!(),
            EnableCap::ClipDistance1 => todo!(),
            EnableCap::ClipDistance2 => todo!(),
            EnableCap::ClipDistance3 => todo!(),
            EnableCap::ClipDistance4 => todo!(),
            EnableCap::ClipDistance5 => todo!(),
            EnableCap::ClipDistance6 => todo!(),
            EnableCap::ClipDistance7 => todo!(),
            EnableCap::RasterizerDiscard => todo!(),
            EnableCap::FramebufferSrgb => todo!(),
            EnableCap::TextureRectangle => todo!(),
            EnableCap::PrimitiveRestart => todo!(),
            EnableCap::ProgramPointSize => (Dirty::empty(), Capabilities::PROGRAM_POINT_SIZE),
            EnableCap::DepthClamp => todo!(),
            EnableCap::TextureCubeMapSeamless => todo!(),
            EnableCap::SampleMask => todo!(),
            EnableCap::SampleShading => todo!(),
            EnableCap::PrimitiveRestartFixedIndex => todo!(),
            // our debug output is synchronous by default
            // TODO: buffer debug output and try to process it (via the callback) during
            // draws instead of blocking when this cap is disabled
            EnableCap::DebugOutputSynchronous => {
                (Dirty::empty(), Capabilities::DEBUG_OUTPUT_SYNCHRONOUS)
            }
            EnableCap::VertexArray => todo!(),
            EnableCap::DebugOutput => todo!(),
        };
        if self.gl_state.caps.set_to(state, bit.1) {
            self.platform_state.dirty_state.set_bits(bit.0);
        }
    }
    fn set_indexed_cap_internal(&mut self, cap: EnableCap, state: bool, index: u32) {
        match cap {
            EnableCap::Blend => todo!(),
            EnableCap::ScissorTest => todo!(),
            _ => {
                panic!("tried to set unindexed capabiltiy with glenable/disablei")
            }
        }
    }
    pub fn oxidegl_disable(&mut self, cap: EnableCap) {
        self.set_unindexed_cap_internal(cap, false);
    }
    pub fn oxidegl_enable(&mut self, cap: EnableCap) {
        self.set_unindexed_cap_internal(cap, true);
    }
    pub fn oxidegl_enablei(&mut self, target: EnableCap, index: GLuint) {
        self.set_indexed_cap_internal(target, true, index);
    }
    pub fn oxidegl_disablei(&mut self, target: EnableCap, index: GLuint) {
        self.set_indexed_cap_internal(target, false, index);
    }
}
