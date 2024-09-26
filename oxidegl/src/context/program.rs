use std::mem;

use ahash::{HashSet, HashSetExt};
use glslang::{Compiler as GlslLangCompiler, Program as GlslLangProgram};
use log::{debug, trace};
//use naga::back::msl::{Options, PipelineOptions};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLFunction, MTLLibrary};
use spirv_cross2::{
    compile::{msl::ShaderInterfaceVariable, CompilableTarget, CompiledArtifact},
    targets::Msl,
    Compiler, Module,
};

use crate::{context::shader::ShaderInternal, enums::ShaderType, NoDebug, ProtoObjRef};

use super::{
    shader::Shader,
    state::{NamedObject, NamedObjectList, ObjectName},
};
#[derive(Debug)]
pub enum ProgramStageBinding {
    Unbound,
    //Spirv shader stage is only allowed to have
    Spirv(ObjectName<Shader>),
    Glsl(HashSet<ObjectName<Shader>>),
}
impl ProgramStageBinding {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "more than u32::MAX shaders cannot exist at once"
    )]
    pub(crate) fn shader_count(&self) -> u32 {
        match self {
            ProgramStageBinding::Unbound => 0,
            ProgramStageBinding::Spirv(_) => 1,
            ProgramStageBinding::Glsl(hash_set) => hash_set.len() as u32,
        }
    }
    #[inline]
    pub(crate) fn new_glsl(obj: ObjectName<Shader>) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(obj);
        Self::Glsl(set)
    }
    #[inline]
    pub(crate) fn new_spirv(obj: ObjectName<Shader>) -> Self {
        Self::Spirv(obj)
    }
    #[inline]
    pub(crate) fn add_shader(&mut self, shader: &Shader) {
        match (self, &shader.internal) {
            (binding , internal) if matches!(binding, Self::Unbound) => match internal {
                super::shader::ShaderInternal::Glsl(_) => {
                    *binding = ProgramStageBinding::new_glsl(shader.name);
                }
                super::shader::ShaderInternal::Spirv(_) => {
                    *binding = ProgramStageBinding::new_spirv(shader.name);
                }
            },
            (Self::Glsl(hash_set), super::shader::ShaderInternal::Glsl(_)) => {
                hash_set.insert(shader.name);
            }
            (_, _) => panic!("tried to mix GLSL and SPIR-V shaders in a single stage or tried to link multiple SPIR-V shaders to the same stage"),
        };
    }
    /// remove a shader from this program stage binding point. Returns whether the shader was present in the binding before removal
    #[inline]
    pub(crate) fn remove_shader(&mut self, name: ObjectName<Shader>) -> bool {
        let b;
        (*self, b) = match self {
            ProgramStageBinding::Unbound => (Self::Unbound, false),
            Self::Spirv(bound_name) => (Self::Unbound, *bound_name == name),
            ProgramStageBinding::Glsl(hash_set) => {
                let has = hash_set.remove(&name);
                (
                    if hash_set.is_empty() {
                        ProgramStageBinding::Unbound
                    } else {
                        ProgramStageBinding::Glsl(mem::take(hash_set))
                    },
                    has,
                )
            }
        };
        b
    }
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Self::Unbound)
    }
}
#[derive(Debug)]
pub struct Program {
    pub(crate) name: ObjectName<Self>,
    pub(crate) refcount: u32,
    //TODO compute shaders
    pub(crate) vertex_shaders: ProgramStageBinding,
    pub(crate) fragment_shaders: ProgramStageBinding,
    pub(crate) compute_shaders: ProgramStageBinding,
    pub(crate) latest_linkage: Option<LinkedProgram>,
    pub(crate) info_log: String,
}
impl Program {
    pub(crate) fn new_named(name: ObjectName<Self>) -> Self {
        debug!("created program {:?}", name);
        Self {
            name,
            refcount: 0,
            vertex_shaders: ProgramStageBinding::Unbound,
            fragment_shaders: ProgramStageBinding::Unbound,
            compute_shaders: ProgramStageBinding::Unbound,
            latest_linkage: None,
            info_log: String::new(),
        }
    }
    pub(crate) fn retain_program(&mut self) {
        self.refcount += 1;
    }
    pub(crate) fn release_program(&mut self) -> bool {
        if self.refcount <= 1 {
            self.refcount = 0;
            return true;
        }
        self.refcount -= 1;
        false
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "more than u32::MAX shaders cannot exist at once"
    )]
    #[inline]
    pub(crate) fn attached_shader_count(&self) -> u32 {
        (self.vertex_shaders.shader_count()
            + self.fragment_shaders.shader_count()
            + self.compute_shaders.shader_count()) as u32
    }
    #[inline]
    pub(crate) fn debug_log_str(&mut self, msg: &str) {
        debug!("{msg}");
        self.info_log.push('\n');
        self.info_log.push_str(msg);
    }
    #[inline]
    pub(crate) fn attach_shader(&mut self, shader: &mut Shader) {
        self.get_stage_binding(shader.stage).add_shader(shader);
        self.debug_log_str(&format!("attached {:?} to {:?}", shader.name, self.name));
        shader.retain();
    }
    fn get_stage_binding(&mut self, stage: ShaderType) -> &mut ProgramStageBinding {
        match stage {
            crate::enums::ShaderType::FragmentShader => &mut self.fragment_shaders,
            crate::enums::ShaderType::VertexShader => &mut self.vertex_shaders,
            crate::enums::ShaderType::ComputeShader => &mut self.compute_shaders,
            _ => unreachable!(),
        }
    }
    /// Detaches shader. Returns whether the shader can be safely removed from the shader list if it is queued for deletion
    #[inline]
    pub(crate) fn detach_shader(&mut self, shader: &mut Shader) -> bool {
        self.debug_log_str(&format!("detached {:?} from {:?}", shader.name, self.name));
        if self
            .get_stage_binding(shader.stage)
            .remove_shader(shader.name)
        {
            shader.release_shader()
        } else {
            false
        }
    }
    #[inline]
    fn link_stage(
        shader_list: &NamedObjectList<Shader>,
        device: &ProtoObjRef<dyn MTLDevice>,
        b: &mut ProgramStageBinding,
        glslang_compiler: &GlslLangCompiler,
    ) -> Result<LinkedShaderStage, String> {
        let stage_spirv = match b {
            ProgramStageBinding::Unbound => unreachable!(),
            ProgramStageBinding::Spirv(_) => todo!(),
            ProgramStageBinding::Glsl(hash_set) => {
                let mut program = glslang_compiler.create_program();
                let mut stage = None;
                for shader in hash_set.iter().copied().map(|name| shader_list.get(name)) {
                    let ShaderInternal::Glsl(internal) = &shader.internal else {
                        unreachable!()
                    };
                    stage = Some(shader.stage);
                    let Some(glslang_shader) = &internal.latest_shader else {
                        panic!("Tried to link a program with a shader that did not compile!")
                    };
                    program.add_shader(glslang_shader);
                }

                let spirv_src = program
                    .compile(
                        stage
                            .expect("stage should have been set")
                            .to_glslang_stage(),
                    )
                    .map_err(|e| e.to_string())?;
                let module = Module::from_words(&spirv_src);
                Compiler::<Msl>::new(module).map_err(|e| e.to_string())?
            }
        };
        let artifact = stage_spirv
            .compile(&Msl::options())
            .map_err(|e| e.to_string())?;

        let msl_src = format!("{artifact}");
        trace!("transformed metal sources for stage:\n{msl_src}");

        let lib = device
            .newLibraryWithSource_options_error(&NSString::from_str(&msl_src), None)
            .map_err(|e| e.to_string())?;

        let entry_points = artifact
            .entry_points()
            .map_err(|e| e.to_string())?
            .collect::<Vec<_>>();

        assert!(
            entry_points.len() == 1,
            "Cannot have more than one entry point per program stage"
        );
        // TODO: coalesce ungrouped (named) uniforms into a single uniform block with a hashmap for by-identifier uniform lookup
        Ok(LinkedShaderStage {
            function: lib
                .newFunctionWithName(&NSString::from_str(&format!(
                    // this is cursed
                    "{}0",
                    entry_points.first().unwrap().name
                )))
                .expect("Metal library did not contain the entry point named by spirv-cross!"),
            lib,
            artifact: artifact.into(),
        })
    }
    //TODO async shader compilation
    #[inline]
    pub(crate) fn link(
        &mut self,
        shader_list: &mut NamedObjectList<Shader>,
        device: &ProtoObjRef<dyn MTLDevice>,
    ) {
        self.latest_linkage = None;
        debug!("attempting to link {:?}", self.name);
        trace!("Current shader program state: {:?}", &self);
        let glslang_compiler =
            GlslLangCompiler::acquire().expect("failed to acquire glslang instance");
        let mut new_linkage = LinkedProgram {
            fragment_entry: None,
            vertex_entry: None,
            compute_entry: None,
        };
        if !self.vertex_shaders.is_empty() {
            trace!("linking vertex shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.vertex_shaders,
                glslang_compiler,
            ) {
                Ok(v) => new_linkage.vertex_entry = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        if !self.fragment_shaders.is_empty() {
            trace!("linking fragment shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.fragment_shaders,
                glslang_compiler,
            ) {
                Ok(v) => new_linkage.vertex_entry = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        if !self.compute_shaders.is_empty() {
            trace!("linking compute shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.compute_shaders,
                glslang_compiler,
            ) {
                Ok(v) => new_linkage.vertex_entry = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        self.latest_linkage = Some(new_linkage);
    }
}

impl NamedObject for Program {}

#[derive(Debug)]
pub struct LinkedProgram {
    pub(crate) fragment_entry: Option<LinkedShaderStage>,
    pub(crate) vertex_entry: Option<LinkedShaderStage>,
    pub(crate) compute_entry: Option<LinkedShaderStage>,
}
#[derive(Debug)]
pub struct LinkedShaderStage {
    /// the entry point for this stage
    function: ProtoObjRef<dyn MTLFunction>,
    /// a retained reference to the metal library that contains the entry point function for this stage
    lib: ProtoObjRef<dyn MTLLibrary>,
    /// the spirv_cross artifact/module that was compiled to the metal lib given above
    artifact: NoDebug<CompiledArtifact<Msl>>,
}
