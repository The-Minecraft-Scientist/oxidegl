use core::str;
use std::{borrow::BorrowMut, cell::Cell};

use log::debug;
use naga::{
    front::{glsl, spv},
    valid::{self, Capabilities, ValidationFlags, Validator},
    Module, ShaderStage, WithSpan,
};

use crate::{
    dispatch::{conversions::GlDstType, gl_types::GLuint},
    enums::{ShaderParameterName, ShaderType},
};

use super::{
    state::{NamedObject, ObjectName},
    Context,
};

#[derive(Debug)]
pub struct Shader {
    pub(crate) name: ObjectName<Shader>,
    pub(crate) stage: ShaderType,
    pub(crate) source_type: ShaderSourceType,
    pub(crate) latest_module: Option<Module>,
    pub(crate) compiler_log: String,
}
#[derive(Debug)]
pub(crate) enum ShaderSourceType {
    Glsl {
        source: String,
    },
    Spirv {
        source: Vec<u8>,
        entry_point: String,
        specialization_constants: Vec<(u32, u32)>,
    },
}
impl Shader {
    pub fn new_text_default(name: ObjectName<Shader>, stage: ShaderType) -> Self {
        Self {
            name,
            stage,
            source_type: ShaderSourceType::Glsl {
                source: String::new(),
            },

            latest_module: None,
            compiler_log: String::new(),
        }
    }
}
impl NamedObject for Shader {}

thread_local! {
    static GLSL_FRONTEND: Cell<Option<glsl::Frontend>> = Cell::new(Some(glsl::Frontend::default()));
}
impl Shader {
    fn compile(&mut self) {
        self.compiler_log.clear();
        match &self.source_type {
            ShaderSourceType::Glsl { source } => {
                let opts =
                    glsl::Options::from(self.stage.as_shader_stage().expect(
                        "OxideGL does not currently support geometry or tesselation shaders",
                    ));
                let mut frontend = GLSL_FRONTEND
                    .take()
                    .expect("Glsl naga frontend should have been present!");
                let res = frontend.parse(&opts, source);
                GLSL_FRONTEND.set(Some(frontend));
                match res {
                    Ok(m) => {
                        debug!("successfully parsed shader {:?}", self.name);
                        self.latest_module = Some(m);
                    }
                    Err(errors) => {
                        self.latest_module = None;
                        for error in errors.errors {
                            let err = WithSpan::new(error.kind)
                                .with_context((error.meta, "parse error".to_owned()));
                            self.compiler_log.push_str(&err.emit_to_string(source));
                            self.compiler_log.push('\n');
                        }
                        debug!(
                            "failed to parse shader {:?}. Errors:\n{}",
                            self.name, &self.compiler_log
                        );
                    }
                }
            }
            ShaderSourceType::Spirv {
                source,
                entry_point,
                specialization_constants,
            } => panic!("SPIRV shader binaries currently unimplemented!"),
        }
    }
    fn validate(&self) {
        if let Some(ref m) = self.latest_module {
            let mut val = GLSL_VALIDATOR
                .take()
                .expect("Naga GLSL validator should have been present!");
            if let Err(e) = val.validate(m) {
                let err;
                if let ShaderSourceType::Glsl { source } = &self.source_type {
                    err = e.emit_to_string(source);
                } else {
                    err = e.into_inner().to_string();
                }

                panic!("{:?} failed Naga validation:\n{}", self.name, err);
            }
        }
    }
}
thread_local! {
    static GLSL_VALIDATOR: Cell<Option<Validator>> = Cell::new(Some(Validator::new(
        ValidationFlags::all(),
        Capabilities::CUBE_ARRAY_TEXTURES
            | Capabilities::CLIP_DISTANCE
            | Capabilities::SHADER_INT64_ATOMIC_MIN_MAX
            | Capabilities::MULTISAMPLED_SHADING
            | Capabilities::SHADER_INT64
            | Capabilities::SUBGROUP_BARRIER,
    )));
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
        self.gl_state
            .shader_list
            .new_obj(|name| Shader::new_text_default(name, r#type))
            .to_raw()
    }
    /// ### Parameters
    /// `shader`
    ///
    /// > Specifies the handle of the shader object whose source code is to be replaced.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements in the `string` and `length` arrays.
    ///
    /// `string`
    ///
    /// > Specifies an array of pointers to strings containing the source code to
    /// > be loaded into the shader.
    ///
    /// `length`
    ///
    /// > Specifies an array of string lengths.
    ///
    /// ### Description
    /// [**glShaderSource**](crate::context::Context::oxidegl_shader_source) sets
    /// the source code in `shader` to the source code in the array of strings
    /// specified by `string`. Any source code previously stored in the shader
    /// object is completely replaced. The number of strings in the array is specified
    /// by `count`. If `length` is [`NULL`](crate::enums::NULL), each string is
    /// assumed to be null terminated. If `length` is a value other than [`NULL`](crate::enums::NULL),
    /// it points to an array containing a string length for each of the corresponding
    /// elements of `string`. Each element in the `length` array may contain the
    /// length of the corresponding string (the null character is not counted as
    /// part of the string length) or a value less than 0 to indicate that the
    /// string is null terminated. The source code strings are not scanned or parsed
    /// at this time; they are simply copied into the specified shader object.
    ///
    /// ### Notes
    /// OpenGL copies the shader source code strings when [**glShaderSource**](crate::context::Context::oxidegl_shader_source)
    /// is called, so an application may free its copy of the source code strings
    /// immediately after the function returns.
    ///
    /// ### Associated Gets
    /// [**glGetShader**](crate::context::Context::oxidegl_get_shader) with arguments
    /// `shader` and [`GL_SHADER_SOURCE_LENGTH`](crate::enums::GL_SHADER_SOURCE_LENGTH)
    ///
    /// [**glGetShaderSource**](crate::context::Context::oxidegl_get_shader_source)
    /// with argument `shader`
    ///
    /// [**glIsShader**](crate::context::Context::oxidegl_is_shader)

    pub unsafe fn oxidegl_shader_source(
        &mut self,
        shader: GLuint,
        count: GLsizei,
        string: GLchar,
        length: *const GLint,
    ) {
        panic!("command oxidegl_shader_source not yet implemented");
    }
}
