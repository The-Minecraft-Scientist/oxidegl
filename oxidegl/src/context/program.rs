use std::{mem, ptr::NonNull};

use ahash::{HashSet, HashSetExt};
use glslang::Compiler as GlslLangCompiler;
use objc2::{rc::Retained, AllocAnyThread, ClassType};
//use naga::back::msl::{Options, PipelineOptions};
use crate::{
    context::{
        debug::{gl_debug, gl_trace, with_debug_state},
        shader::ShaderInternal,
    },
    enums::ShaderType,
    NoDebug, ProtoObjRef,
};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLFunction, MTLLibrary};
use spirv_cross2::{
    compile::{msl::CompilerOptions, CompiledArtifact},
    reflect::ResourceIter,
    targets::Msl,
    Compiler, Module, SpirvCrossError,
};

use super::{
    gl_object::{NamedObject, NamedObjectList, NoLateInit, ObjectName},
    shader::Shader,
};
#[derive(Debug)]
pub enum ProgramStageBinding {
    Unbound,
    //Spirv shader stage is only allowed to have

    // what did he mean by this?? ^^
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
            ProgramStageBinding::Glsl(set) => set.len() as u32,
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
            (_, _) => panic!("tried to mix GLSL and SPIR-V shaders in a single stage or tried to link multiple SPIR-V shader objects to the same stage"),
        };
    }
    /// remove a shader from this program stage binding point. Returns whether the shader was present in the binding before removal
    #[inline]
    fn remove_shader(&mut self, name: ObjectName<Shader>) -> bool {
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
    pub(crate) vertex_shaders: ProgramStageBinding,
    pub(crate) fragment_shaders: ProgramStageBinding,
    pub(crate) compute_shaders: ProgramStageBinding,
    pub(crate) latest_linkage: Option<LinkedProgram>,
    pub(crate) info_log: String,
}
impl Program {
    pub(crate) fn new_named(name: ObjectName<Self>) -> Self {
        gl_debug!("created program {:?}", name);
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
    #[inline]
    pub(crate) fn attached_shader_count(&self) -> u32 {
        self.vertex_shaders.shader_count()
            + self.fragment_shaders.shader_count()
            + self.compute_shaders.shader_count()
    }
    #[inline]
    pub(crate) fn debug_log_str(&mut self, msg: &str) {
        gl_debug!("{msg}");
        self.info_log.push('\n');
        self.info_log.push_str(msg);
    }
    #[inline]
    pub(crate) fn attach_shader(&mut self, shader: &mut Shader) {
        self.get_stage_binding(shader.stage).add_shader(shader);
        self.debug_log_str(&format!("attached {:?} to {:?}", shader.name, self.name));
        shader.retain();
    }
    #[inline]
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
        label: Option<&Retained<NSString>>,
    ) -> Result<LinkedShaderStage, Box<str>> {
        macro_rules! err_ret {
            ($e:expr) => {
                return Err($e.to_string().into_boxed_str())
            };
        }
        let mut used_shaders = Vec::with_capacity(1);
        let mut stage = None;
        let mut stage_spirv = match b {
            ProgramStageBinding::Unbound => unreachable!(),
            ProgramStageBinding::Spirv(_) => todo!(),
            ProgramStageBinding::Glsl(hash_set) => {
                let mut program = glslang_compiler.create_program();
                for shader in hash_set.iter().copied().map(|name| shader_list.get(name)) {
                    used_shaders.push(shader.name.to_raw().to_string().into_boxed_str());
                    let ShaderInternal::Glsl(internal) = &shader.internal else {
                        unreachable!()
                    };
                    stage = Some(shader.stage);
                    let Some(glslang_shader) = &internal.latest_shader else {
                        err_ret!("Tried to link a program with a shader that did not compile!");
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
        stage_spirv
            .add_discrete_descriptor_set(3)
            .map_err(|e| e.to_string().into_boxed_str())?;

        let entry_name = format!("{:?}_{}_main", stage.unwrap(), used_shaders.join("_"));

        let model = stage_spirv
            .execution_model()
            .expect("failed to get execution model");
        let previous_entry_cleansed = stage_spirv
            .cleansed_entry_point_name("main", model)
            .expect("failed to cleanse entry point name")
            .expect("cleansed entry point was null");
        stage_spirv
            .rename_entry_point(previous_entry_cleansed, entry_name.clone(), model)
            .expect("failed to rename spirv entry point");

        let mut opts = CompilerOptions::default();
        opts.version = (2, 1).into();
        opts.argument_buffers = true;
        let artifact = stage_spirv.compile(&opts).map_err(|e| e.to_string())?;

        let msl_src = format!("{artifact}");
        gl_trace!(src: ShaderCompiler, "transformed metal sources for stage:\n{msl_src}");

        let lib = device
            .newLibraryWithSource_options_error(&NSString::from_str(&msl_src), None)
            .map_err(|e| e.to_string())?;
        if let Some(label) = label {
            lib.setLabel(Some(label));
        }
        // TODO: coalesce ungrouped (named) uniforms into a single uniform block with a hashmap for by-identifier uniform lookup
        Ok(LinkedShaderStage {
            function: lib
                .newFunctionWithName(&NSString::from_str(&entry_name))
                .unwrap(),
            lib,
            resources: LinkedProgramResources::from_compiler(&artifact)
                .expect("failed to get resource bindings during program linkage!"),
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
        gl_debug!(src: ShaderCompiler, "attempting to link {:?}", self.name);
        let glslang_compiler =
            GlslLangCompiler::acquire().expect("failed to acquire glslang instance");
        let mut new_linkage = LinkedProgram {
            fragment: None,
            vertex: None,
            compute: None,
        };
        let label = with_debug_state(|state| state.get_label(self.name))
            .flatten()
            .as_deref()
            .map(|c| unsafe {
                let alloc = NSString::alloc();
                NSString::initWithCString_encoding(
                    alloc,
                    NonNull::new_unchecked(c.as_ptr().cast_mut()),
                    NSString::defaultCStringEncoding(),
                )
                .expect("failed to create NSString")
            });
        if !self.vertex_shaders.is_empty() {
            gl_trace!(src: ShaderCompiler, "linking vertex shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.vertex_shaders,
                glslang_compiler,
                label.as_ref(),
            ) {
                Ok(v) => new_linkage.vertex = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        if !self.fragment_shaders.is_empty() {
            gl_trace!(src: ShaderCompiler, "linking fragment shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.fragment_shaders,
                glslang_compiler,
                label.as_ref(),
            ) {
                Ok(v) => new_linkage.fragment = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        if !self.compute_shaders.is_empty() {
            gl_trace!(src: ShaderCompiler, "linking compute shaders");
            match Self::link_stage(
                shader_list,
                device,
                &mut self.compute_shaders,
                glslang_compiler,
                label.as_ref(),
            ) {
                Ok(v) => new_linkage.vertex = Some(v),
                Err(s) => {
                    self.debug_log_str(&s);
                    return;
                }
            }
        }
        self.latest_linkage = Some(new_linkage);
    }
}

impl NamedObject for Program {
    type LateInitType = NoLateInit<Self>;
}

#[derive(Debug)]
pub struct LinkedProgram {
    pub(crate) fragment: Option<LinkedShaderStage>,
    pub(crate) vertex: Option<LinkedShaderStage>,
    pub(crate) compute: Option<LinkedShaderStage>,
}
#[inline]
fn to_resource_vec(
    iter: ResourceIter<'_>,
    compiler: &Compiler<Msl>,
) -> Result<Vec<ProgramResource>, SpirvCrossError> {
    let mut vec = Vec::with_capacity(iter.len());
    for v in iter {
        vec.push(ProgramResource {
            name: v.name.to_string().into_boxed_str(),
            binding: compiler
                .decoration(v.id, spirv_cross2::spirv::Decoration::Binding)?
                .map(|v| v.as_literal().expect("failed to convert literal")),
            location: compiler
                .decoration(v.id, spirv_cross2::spirv::Decoration::Location)?
                .map(|v| v.as_literal().expect("failed to convert literal")),
        });
    }
    Ok(vec)
}
#[derive(Debug)]
pub struct LinkedProgramResources {
    pub(crate) uniform_buffers: Vec<ProgramResource>,
    pub(crate) shader_storage_buffers: Vec<ProgramResource>,
    pub(crate) atomic_counter_buffers: Vec<ProgramResource>,
    pub(crate) stage_inputs: Vec<ProgramResource>,
    pub(crate) plain_uniforms: Vec<ProgramResource>,
}
impl LinkedProgramResources {
    //TODO XFBs
    fn from_compiler(spirvc: &Compiler<Msl>) -> Result<Self, SpirvCrossError> {
        let value = spirvc.shader_resources()?;
        let uniform_buffers = to_resource_vec(
            value.resources_for_type(spirv_cross2::reflect::ResourceType::UniformBuffer)?,
            spirvc,
        )?;
        let shader_storage_buffers = to_resource_vec(
            value.resources_for_type(spirv_cross2::reflect::ResourceType::StorageBuffer)?,
            spirvc,
        )?;
        let atomic_counter_buffers = to_resource_vec(
            value.resources_for_type(spirv_cross2::reflect::ResourceType::AtomicCounter)?,
            spirvc,
        )?;
        let stage_inputs = to_resource_vec(
            value.resources_for_type(spirv_cross2::reflect::ResourceType::StageInput)?,
            spirvc,
        )?;
        let plain_uniforms = to_resource_vec(
            value.resources_for_type(spirv_cross2::reflect::ResourceType::GlPlainUniform)?,
            spirvc,
        )?;
        Ok(Self {
            uniform_buffers,
            shader_storage_buffers,
            atomic_counter_buffers,
            stage_inputs,
            plain_uniforms,
        })
    }
}
#[derive(Debug)]
pub struct ProgramResource {
    pub(crate) name: Box<str>,
    pub(crate) binding: Option<u32>,
    pub(crate) location: Option<u32>,
}
#[derive(Debug)]
pub struct LinkedShaderStage {
    /// the entry point for this stage
    pub(crate) function: ProtoObjRef<dyn MTLFunction>,
    /// a retained reference to the metal library that contains the entry point function for this stage
    pub(crate) lib: ProtoObjRef<dyn MTLLibrary>,
    /// the `spirv_cross` artifact/module that was compiled to the metal lib given above
    pub(crate) artifact: NoDebug<CompiledArtifact<Msl>>,
    /// Resources
    pub(crate) resources: LinkedProgramResources,
}
