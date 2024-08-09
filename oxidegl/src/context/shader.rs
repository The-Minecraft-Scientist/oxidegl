use naga::{front::glsl, Module, ShaderStage};

use crate::{dispatch::gl_types::GLuint, enums::ShaderType};

use super::{
    state::{NamedObject, ObjectName},
    Context,
};

pub struct Shader {
    pub(crate) name: ObjectName<Shader>,
    pub(crate) stage: ShaderType,
    pub(crate) source: Vec<u8>,
    pub(crate) spv: bool,
    pub(crate) module: Option<Module>,
    pub(crate) compiler_log: String,
}
impl NamedObject for Shader {}

thread_local! {
    static GLSL_FRONTEND: glsl::Frontend = glsl::Frontend::default();
}
impl Shader {
    fn compile(&mut self) {}
}
impl ShaderType {
    #[must_use]
    pub fn as_shader_stage(self) -> Option<ShaderStage> {
        match self {
            ShaderType::FragmentShader => Some(ShaderStage::Fragment),
            ShaderType::VertexShader => Some(ShaderStage::Vertex),
            ShaderType::GeometryShader => todo!(),
            ShaderType::TessEvaluationShader => todo!(),
            ShaderType::TessControlShader => todo!(),
            ShaderType::ComputeShader => Some(ShaderStage::Compute),
        }
    }
}

impl Context {
    /// ### Parameters
    /// `shaderType`
    ///
    /// > Specifies the type of shader to be created. Must be one of [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER),
    /// > [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER), [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER),
    /// > [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER),
    /// > [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER), or [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER).
    ///
    /// ### Description
    /// [**glCreateShader**](crate::context::Context::oxidegl_create_shader) creates
    /// an empty shader object and returns a non-zero value by which it can be
    /// referenced. A shader object is used to maintain the source code strings
    /// that define a shader. `shaderType` indicates the type of shader to be created.
    /// Five types of shader are supported. A shader of type [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER)
    /// is a shader that is intended to run on the programmable compute processor.
    /// A shader of type [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER) is
    /// a shader that is intended to run on the programmable vertex processor.
    /// A shader of type [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER)
    /// is a shader that is intended to run on the programmable tessellation processor
    /// in the control stage. A shader of type [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER)
    /// is a shader that is intended to run on the programmable tessellation processor
    /// in the evaluation stage. A shader of type [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER)
    /// is a shader that is intended to run on the programmable geometry processor.
    /// A shader of type [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER)
    /// is a shader that is intended to run on the programmable fragment processor.
    ///
    /// When created, a shader object's [`GL_SHADER_TYPE`](crate::enums::GL_SHADER_TYPE)
    /// parameter is set to either [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER),
    /// [`GL_VERTEX_SHADER`](crate::enums::GL_VERTEX_SHADER), [`GL_TESS_CONTROL_SHADER`](crate::enums::GL_TESS_CONTROL_SHADER),
    /// [`GL_TESS_EVALUATION_SHADER`](crate::enums::GL_TESS_EVALUATION_SHADER),
    /// [`GL_GEOMETRY_SHADER`](crate::enums::GL_GEOMETRY_SHADER) or [`GL_FRAGMENT_SHADER`](crate::enums::GL_FRAGMENT_SHADER),
    /// depending on the value of `shaderType`.
    ///
    /// ### Notes
    /// Like buffer and texture objects, the name space for shader objects may
    /// be shared across a set of contexts, as long as the server sides of the
    /// contexts share the same address space. If the name space is shared across
    /// contexts, any attached objects and the data associated with those attached
    /// objects are shared as well.
    ///
    /// Applications are responsible for providing the synchronization across API
    /// calls when objects are accessed from different execution threads.
    ///
    /// [`GL_COMPUTE_SHADER`](crate::enums::GL_COMPUTE_SHADER) is available only
    /// if the GL version is 4.3 or higher.
    ///
    /// ### Associated Gets
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with a valid
    /// shader object and the parameter to be queried
    ///
    /// [**glGetShaderInfoLog**](crate::context::Context::oxidegl_get_shader_info_log)
    /// with a valid shader object
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    /// with a valid shader object
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)
    pub fn oxidegl_create_shader(&mut self, r#type: ShaderType) -> GLuint {
        panic!("command oxidegl_create_shader not yet implemented");
    }
}
