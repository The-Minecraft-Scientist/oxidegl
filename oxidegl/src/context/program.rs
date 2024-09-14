
use ahash::{HashSet, HashSetExt};
use log::{debug, trace};
use naga::back::msl::{Options, PipelineOptions};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLFunction, MTLLibrary};

use crate::{
    enums::ShaderType,
    RetainedObject,
};

use super::{
    shader::Shader,
    state::{NamedObject, NamedObjectList, ObjectName},
};

#[derive(Debug)]
pub struct Program {
    pub(crate) name: ObjectName<Self>,
    pub(crate) refcount: u32,
    //TODO compute shaders
    pub(crate) vertex_shaders: HashSet<ObjectName<Shader>>,
    pub(crate) fragment_shaders: HashSet<ObjectName<Shader>>,
    pub(crate) compute_shaders: HashSet<ObjectName<Shader>>,
    pub(crate) latest_linkage: Option<LinkedShaderProgram>,
    pub(crate) info_log: String,
}
impl Program {
    pub(crate) fn new_named(name: ObjectName<Self>) -> Self {
        debug!("created program {:?}", name);
        Self {
            name,
            refcount: 0,
            vertex_shaders: HashSet::new(),
            fragment_shaders: HashSet::new(),
            compute_shaders: HashSet::new(),
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
    // >u32::MAX - 1 shaders may not exist simultaneously
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn attached_shader_count(&self) -> u32 {
        (self.vertex_shaders.len() + self.fragment_shaders.len() + self.compute_shaders.len())
            as u32
    }
    pub(crate) fn debug_log_str(&mut self, msg: &str) {
        debug!("{msg}");
        self.info_log.push('\n');
        self.info_log.push_str(msg);
    }
    pub(crate) fn attach_shader(&mut self, shader: &mut Shader) {
        self.debug_log_str(&format!("attached {:?} to {:?}", shader.name, self.name));
        let set = self.get_stage_set(shader.stage);
        set.insert(shader.name);
        shader.retain_shader();
    }
    fn get_stage_set(&mut self, stage: ShaderType) -> &mut HashSet<ObjectName<Shader>> {
        match stage {
            crate::enums::ShaderType::FragmentShader => &mut self.fragment_shaders,
            crate::enums::ShaderType::VertexShader => &mut self.vertex_shaders,
            crate::enums::ShaderType::ComputeShader => &mut self.compute_shaders,
            _ => unreachable!(),
        }
    }
    /// Detaches shader. Returns whether the shader can be safely removed from the shader list if it is queued for deletion
    pub(crate) fn detach_shader(&mut self, shader: &mut Shader) -> bool {
        self.debug_log_str(&format!("detached {:?} from {:?}", shader.name, self.name));
        let set = self.get_stage_set(shader.stage);
        set.remove(&shader.name);
        shader.release_shader()
    }
    fn compile_program_stage_shaders<'a>(
        mut shaders: impl Iterator<Item = &'a Shader>,
        device: &RetainedObject<dyn MTLDevice>,
    ) -> (
        RetainedObject<dyn MTLLibrary>,
        RetainedObject<dyn MTLFunction>,
    ) {
        //TODO handle linkage of multiple shaders per stage

        let shader = *shaders.next().as_ref().expect("shaders iter was empty");
        let module = shader
            .latest_module
            .as_ref()
            .expect("Tried to link shader to a program before it was compiled");
        assert!(
            module.entry_points.len() == 1,
            "UB: multiple entry points in shader"
        );
        let info = shader.validate();

        match naga::back::msl::write_string(
            module,
            &info,
            &Options::default(),
            &PipelineOptions::default(),
        ) {
            Ok((source, info)) => {
                trace!("translated shader source:\n{source}");
                match device.newLibraryWithSource_options_error(&NSString::from_str(&source), None)
                {
                    Ok(lib) => {
                        let name = info.entry_point_names.first().unwrap().as_ref().unwrap();
                        let func = lib
                            .newFunctionWithName(&NSString::from_str(name))
                            .expect("promised entry point did not exist in compiled MTLLibrary");
                        trace!("compiled Metal library:\n{lib:?}\nwith entry point function:\n{func:?}");
                        (lib, func)
                    }
                    Err(e) => panic!("Metal shader compilation error: \n{e}"),
                }
            }
            Err(e) => panic!("Naga shader conversion error:\n{e}"),
        }
    }

    //TODO do shader "linking" (mostly metallib compilation) off of the main thread
    pub(crate) fn link(
        &mut self,
        shader_list: &mut NamedObjectList<Shader>,
        device: &RetainedObject<dyn MTLDevice>,
    ) {
        debug!("attempting to link {:?}", self.name);
        trace!("Current shader program state: {:?}", &self);

        let mut libs = Vec::with_capacity(3);
        let mut vertex_entry = None;
        let mut fragment_entry = None;
        if !self.vertex_shaders.is_empty() {
            trace!("compiling vertex shaders");
            // TODO(Vertex): use specialization/function constants to specialize a vertex getter function
            // that normalizes the remaining non-normed native vertex formats and casts integers
            // to float upon loading the vertex attrs
            // See: vao.rsL#1412
            let (lib, vertex) = Self::compile_program_stage_shaders(
                self.vertex_shaders.iter().map(|v| shader_list.get(*v)),
                device,
            );
            vertex_entry = Some(vertex);
            libs.push(lib);
        }
        if !self.fragment_shaders.is_empty() {
            trace!("compiling fragment shaders");
            let (lib, fragment) = Self::compile_program_stage_shaders(
                self.fragment_shaders.iter().map(|v| shader_list.get(*v)),
                device,
            );
            fragment_entry = Some(fragment);
            libs.push(lib);
        }

        let linked = LinkedShaderProgram {
            libs,
            fragment_entry,
            vertex_entry,
        };
        self.latest_linkage = Some(linked);
    }
}
impl NamedObject for Program {}

#[derive(Debug)]
pub struct LinkedShaderProgram {
    // TODO: compute shaders
    libs: Vec<RetainedObject<dyn MTLLibrary>>,
    fragment_entry: Option<RetainedObject<dyn MTLFunction>>,
    vertex_entry: Option<RetainedObject<dyn MTLFunction>>,
}
