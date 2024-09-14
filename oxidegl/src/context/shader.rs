use std::{cell::Cell, fmt::Debug};

use log::{debug, error, warn};
use naga::{
    front::glsl,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module, ShaderStage, WithSpan,
};

use crate::enums::ShaderType;

use super::state::{NamedObject, ObjectName};
//TODO: write more debug logging to compiler_log
#[derive(Debug)]
pub struct Shader {
    pub(crate) name: ObjectName<Shader>,
    pub(crate) stage: ShaderType,
    pub(crate) refcount: u32,
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
impl ShaderSourceType {
    pub(crate) fn is_spirv(&self) -> bool {
        matches!(self, ShaderSourceType::Spirv { .. })
    }
    //4gb shader is not real, 4gb shader cannot hurt you
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn source_len(&self) -> u32 {
        (match self {
            ShaderSourceType::Glsl { source } => source.len(),
            ShaderSourceType::Spirv { source, .. } => source.len(),
        }) as u32
    }
}
impl Shader {
    pub fn new_text_default(name: ObjectName<Self>, stage: ShaderType) -> Self {
        debug!("created new GLSL {stage:?} {name:?}");
        Self {
            name,
            stage,
            refcount: 0,
            source_type: ShaderSourceType::Glsl {
                source: String::new(),
            },
            latest_module: None,
            compiler_log: String::new(),
        }
    }
}
impl NamedObject for Shader {}

impl Shader {
    //TODO: experiment: shader parsing, translation and compilation off of the main thread (if shader compilation perf becomes an issue)
    //TODO: collapse global uniforms into uniform block (or fork naga and add global uniform support)
    pub(crate) fn compile(&mut self) {
        thread_local! {
            static GLSL_FRONTEND: Cell<Option<glsl::Frontend>> = Cell::new(Some(glsl::Frontend::default()));
        }
        self.compiler_log.clear();
        debug!("attempting to parse shader {:?}", self.name);
        match &self.source_type {
            ShaderSourceType::Glsl { source } => {
                let opts =
                    glsl::Options::from(self.stage.as_shader_stage().expect(
                        "OxideGL does not currently support geometry or tesselation shaders",
                    ));
                // This expect can only fire if the caller commits some SERIOUS (but unfortunately barely legal) crimes
                let mut frontend = GLSL_FRONTEND
                    .take()
                    .expect("naga glsl frontend should have been present!");
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
                            self.compiler_log.push_str(&err.emit_to_string_with_path(
                                source,
                                &format!("{:?}(id #{})", self.stage, self.name.to_raw()),
                            ));
                            self.compiler_log.push('\n');
                        }
                        error!(
                            "failed to parse shader {:?}. Errors:\n{}",
                            self.name,
                            &self.compiler_log.trim()
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
        #[cfg(debug_assertions)]
        self.validate();
    }
    pub(crate) fn validate(&self) -> ModuleInfo {
        if let Some(ref m) = self.latest_module {
            debug!("validating {:?}", self.name);
            let mut val = GLSL_VALIDATOR
                .take()
                .expect("Naga GLSL validator should have been present!");
            let ret = match val.validate(m) {
                Ok(info) => info,
                Err(e) => {
                    let err;
                    if let ShaderSourceType::Glsl { source } = &self.source_type {
                        err = e.emit_to_string(source);
                    } else {
                        err = e.into_inner().to_string();
                    }

                    panic!(
                        "OxideGL internal error: {:?} failed Naga validation:\n{}",
                        self.name, err
                    );
                }
            };
            GLSL_VALIDATOR.set(Some(val));
            ret
        } else {
            panic!("tried to validate a non-parsed Shader!");
        }
    }
    /// Increments the program reference count on this shader. Call this function when attaching a shader object to a program
    pub(crate) fn retain_shader(&mut self) {
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

thread_local! {
    // TODO correctly detect device capabilities
    // (see https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/mod.rs#L201)
    // (see https://github.com/gfx-rs/wgpu/blob/trunk/wgpu-hal/src/metal/adapter.rs#L842)
    static GLSL_VALIDATOR: Cell<Option<Validator>> = Cell::new(Some(Validator::new(
        ValidationFlags::all(),
        Capabilities::empty(),
    )));
}
impl ShaderType {
    //ShaderType is Copy
    #[allow(clippy::must_use_candidate)]
    pub fn as_shader_stage(self) -> Option<ShaderStage> {
        match self {
            ShaderType::FragmentShader => Some(ShaderStage::Fragment),
            ShaderType::VertexShader => Some(ShaderStage::Vertex),
            //TODO: geometry and tesselation shader emulation :)
            ShaderType::GeometryShader => todo!(),
            ShaderType::TessEvaluationShader => todo!(),
            ShaderType::TessControlShader => todo!(),
            ShaderType::ComputeShader => Some(ShaderStage::Compute),
        }
    }
}
