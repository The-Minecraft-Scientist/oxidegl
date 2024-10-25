use std::{fmt::Debug, mem};

use crate::{enums::ShaderType, NoDebug};
use glslang::{
    Compiler as GlslangCompiler, CompilerOptions, Shader as GlslLangShader, ShaderInput,
    ShaderMessage, ShaderSource, ShaderStage, SourceLanguage, Target,
};
// use naga::{
//     front::glsl,
//     valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
//     ShaderStage, WithSpan,
// };
use spirv_cross2::Module;

use super::{
    debug::gl_debug,
    state::{NamedObject, NoLateInit, ObjectName},
};
//TODO: write more debug logging to compiler_log
#[derive(Debug)]
pub struct Shader {
    pub(crate) name: ObjectName<Shader>,
    pub(crate) stage: ShaderType,
    pub(crate) refcount: u32,
    pub(crate) internal: ShaderInternal,
    pub(crate) compiler_log: String,
}
#[derive(Debug, Default)]
pub struct GlslShaderInternal {
    pub(crate) source: String,
    pub(crate) latest_shader: Option<NoDebug<GlslLangShader<'static>>>,
}
#[derive(Debug)]
pub struct SpirvShaderInternal {
    pub(crate) source: Vec<u32>,
    pub(crate) latest_module: Option<NoDebug<Module<'static>>>,
}
#[derive(Debug)]
pub(crate) enum ShaderInternal {
    Glsl(GlslShaderInternal),
    Spirv(SpirvShaderInternal),
}
impl ShaderInternal {
    pub(crate) fn is_spirv(&self) -> bool {
        matches!(self, ShaderInternal::Spirv(_))
    }
    //4gb shader is not real, 4gb shader cannot hurt you
    #[expect(clippy::cast_possible_truncation)]
    pub(crate) fn source_len(&self) -> u32 {
        (match self {
            ShaderInternal::Glsl(internal) => internal.source.len(),
            // 4 byte words
            ShaderInternal::Spirv(internal) => internal.source.len() * 4,
        }) as u32
    }
    /// Returns whether the last attempted compilation of this internal shader succeeded
    pub(crate) fn compile_status(&self) -> bool {
        match self {
            ShaderInternal::Glsl(glsl_shader_internal) => {
                glsl_shader_internal.latest_shader.is_some()
            }
            ShaderInternal::Spirv(_spirv_shader_internal) => todo!(),
        }
    }
}
impl Shader {
    pub fn new_text_default(name: ObjectName<Self>, stage: ShaderType) -> Self {
        gl_debug!(src: ShaderCompiler,"created new GLSL {stage:?} {name:?}");
        Self {
            name,
            stage,
            refcount: 0,
            internal: ShaderInternal::Glsl(GlslShaderInternal::default()),
            compiler_log: String::new(),
        }
    }
}
impl NamedObject for Shader {
    type LateInitType = NoLateInit<Self>;
}

impl Shader {
    pub(crate) fn compile(&mut self) {
        match &mut self.internal {
            ShaderInternal::Glsl(glsl_shader_internal) => {
                // Clear the previous compilation attempt
                glsl_shader_internal.latest_shader = None;
                let source = ShaderSource::from(mem::take(&mut glsl_shader_internal.source));
                let comp = GlslangCompiler::acquire().expect("failed to acquire Glslang compiler");

                let opts = CompilerOptions {
                    source_language: SourceLanguage::GLSL,
                    target: Target::OpenGL {
                        version: glslang::OpenGlVersion::OpenGL4_5,
                        spirv_version: Some(glslang::SpirvVersion::SPIRV1_0),
                    },
                    version_profile: None,
                    messages: ShaderMessage::RELAXED_ERRORS
                        | ShaderMessage::ENHANCED
                        | ShaderMessage::DEBUG_INFO
                        | ShaderMessage::ONLY_PREPROCESSOR
                        // VULKAN_RULES_RELAXED
                        | ShaderMessage::from_bits_retain(1 << 2),
                };

                let input = match ShaderInput::new(
                    &source,
                    self.stage.to_glslang_stage(),
                    &opts,
                    None,
                    None,
                ) {
                    Ok(input) => input,
                    Err(err) => {
                        self.write_to_compiler_log(&err.to_string());
                        return;
                    }
                };
                let shader = match comp.create_shader(input) {
                    Ok(shader) => shader,
                    Err(e) => {
                        self.write_to_compiler_log(&e.to_string());
                        return;
                    }
                };
                glsl_shader_internal.latest_shader = Some(shader.into());
            }
            ShaderInternal::Spirv(_spirv_shader_internal) => todo!(),
        }
    }
    pub(crate) fn write_to_compiler_log(&mut self, info: &str) {
        gl_debug!(src: ShaderCompiler, "{:?} compiler log: {info}", self.name);
        self.compiler_log.push_str(info);
        self.compiler_log.push('\n');
    }
    /// Increments the program reference count on this shader. Call this function when attaching a shader object to a program
    pub(crate) fn retain(&mut self) {
        self.refcount += 1;
    }
    /// Decrements the program reference count on this shader. Call this function when detaching a shader object from a program.
    ///
    /// Returns `true` if it is OK to deallocate this shader object after the decrement (i.e. it is not currently attached to any program objects)
    pub(crate) fn release_shader(&mut self) -> bool {
        if self.refcount <= 1 {
            self.refcount = 0;
            true
        } else {
            self.refcount -= 1;
            false
        }
    }
}

// TODO correctly detect device capabilities
// (see https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/mod.rs#L201)
// (see https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/adapter.rs#L842)
impl ShaderType {
    #[must_use]
    pub fn to_glslang_stage(self) -> ShaderStage {
        match self {
            ShaderType::FragmentShader => ShaderStage::Fragment,
            ShaderType::VertexShader => ShaderStage::Vertex,
            //TODO: geometry and tesselation shader emulation :)
            ShaderType::GeometryShader => ShaderStage::Geometry,
            ShaderType::TessEvaluationShader => ShaderStage::TesselationEvaluation,
            ShaderType::TessControlShader => ShaderStage::TesselationControl,
            ShaderType::ComputeShader => ShaderStage::Compute,
        }
    }
}
