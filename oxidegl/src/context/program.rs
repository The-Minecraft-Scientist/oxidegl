use ahash::{HashSet, HashSetExt};
use log::{debug, trace};
use naga::back::msl::{Options, PipelineOptions};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLFunction, MTLLibrary};

use crate::enums::{ShaderType, UseProgramStageMask};

use super::{
    shader::Shader,
    state::{NamedObject, NamedObjectList, ObjectName},
    Context,
};

#[derive(Debug)]
pub struct Program {
    name: ObjectName<Self>,
    //TODO compute shaders
    vertex_shaders: HashSet<ObjectName<Shader>>,
    fragment_shaders: HashSet<ObjectName<Shader>>,
    compute_shaders: HashSet<ObjectName<Shader>>,
    latest_linkage: Option<LinkedShaderProgram>,
}
impl Program {
    pub(crate) fn new_named(name: ObjectName<Self>) -> Self {
        debug!("created program {:?}", name);
        Self {
            name,
            vertex_shaders: HashSet::new(),
            fragment_shaders: HashSet::new(),
            compute_shaders: HashSet::new(),
            latest_linkage: None,
        }
    }
    pub(crate) fn attach_shader(&mut self, shader: &mut Shader) {
        debug!("attatched {:?} to {:?}", shader.name, self.name);
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
        debug!("detached {:?} from {:?}", shader.name, self.name);
        let set = self.get_stage_set(shader.stage);
        set.remove(&shader.name);
        shader.release_shader()
    }
    fn compile_shaders_to_lib<'a>(
        mut shaders: impl Iterator<Item = &'a Shader>,
        device: &Retained<ProtocolObject<dyn MTLDevice>>,
    ) -> (Retained<ProtocolObject<dyn MTLLibrary>>, EntryPoint) {
        //TODO handle linkage of multipler shaders per stage
        let shader = *shaders.next().as_ref().expect("shaders iter was empty");
        let module = shader
            .latest_module
            .as_ref()
            .expect("Tried to link shader before it was compiled");
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
            Err(e) => panic!("Naga shader conversion error: \n{e}"),
        }
    }
    //TODO do shader "linking" (compilation to MTL lib) off of the main thread
    pub(crate) fn link(
        &mut self,
        shader_list: &mut NamedObjectList<Shader>,
        device: &Retained<ProtocolObject<dyn MTLDevice>>,
    ) {
        debug!("attempting to link {:?}", self.name);
        trace!("Current shader program state: {:?}", &self);

        let mut libs = Vec::with_capacity(3);
        let mut vertex_entry = None;
        let mut fragment_entry = None;
        if !self.vertex_shaders.is_empty() {
            trace!("compiling vertex shaders");
            // Vertex TODO: use specialization/function constants to specialize a vertex getter function
            // that normalizes the remaining non-normed native vertex formats and casts integers
            // to float upon loading the vertex attrs
            // See: vao.rsL#1412
            let (lib, vertex) = Self::compile_shaders_to_lib(
                self.vertex_shaders.iter().map(|v| shader_list.get(*v)),
                device,
            );
            vertex_entry = Some(vertex);
            libs.push(lib);
        }
        if !self.fragment_shaders.is_empty() {
            trace!("compiling fragment shaders");
            let (lib, fragment) = Self::compile_shaders_to_lib(
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
    libs: Vec<Retained<ProtocolObject<dyn MTLLibrary>>>,
    fragment_entry: Option<EntryPoint>,
    vertex_entry: Option<EntryPoint>,
}
pub(crate) type EntryPoint = Retained<ProtocolObject<dyn MTLFunction>>;
