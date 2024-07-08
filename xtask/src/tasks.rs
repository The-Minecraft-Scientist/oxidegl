use std::{
    fs::{create_dir, read_dir},
    path::{Path, PathBuf},
    process::{self, exit, Command, Stdio},
    sync::{Arc, OnceLock},
};

use anyhow::{bail, Result};
use clap::{Args, Subcommand};
use dashmap::DashSet;
use enum_dispatch::enum_dispatch;
use log::{error, info};

use crate::{
    codegen::{get_vals, write_dispatch_impl, write_enum_impl, write_placeholder_impl},
    open_file_writer,
};

static COMPLETED_TASKS: OnceLock<Arc<DashSet<Task>>> = OnceLock::new();
fn get_completed_tasks() -> Arc<DashSet<Task>> {
    Arc::clone(COMPLETED_TASKS.get_or_init(|| Arc::new(DashSet::new())))
}
fn not_completed(t: &Task) -> bool {
    !get_completed_tasks().contains(t)
}
fn complete_task(t: Task) {
    get_completed_tasks().insert(t);
}
#[enum_dispatch]
#[derive(Subcommand, Clone, Eq, PartialEq, Hash, Debug)]
#[command(version, about, long_about = None)]
pub enum Task {
    /// Build liboxidegl.dylib
    BuildOxideGL,

    /// Build OxideGL GLFW (requires XCode command line tools for clang, cmake and make)
    BuildGLFW,

    /// Generate OxideGL rust GL bindings/placeholder impls
    GenerateBindings,

    /// Run a "test". A test is a name=relative_path_to_binary pair given in tests.txt.
    /// Paths given are relative to the workspace root.
    RunTest,

    /// Init GLFW git submodule if it hasn't been already
    GetGLFW,

    /// Trigger a download of the XCode command line tools if they aren't present
    GetXcodeCommandLineTools,

    /// Init OpenGL-Refpages and -Registry submodules (required to run codegen)
    GetKhronosStuff,

    /// Generates a GLFW build folder in oxidegl-glfw/build
    GenGLFWBuild,
}
#[enum_dispatch(Task)]
pub trait TaskTrait: Sized {
    /// Returns a list of task dependencies that must be completed before this task can run. Each individual Task in the array may be performed in parallel.
    /// If you need serial execution of a list of dependecies, chain the dependency to be executed serially from its dependent tasks' `dependencies` function.
    ///
    /// Default implemention returns `None`
    fn dependencies(&self) -> Option<Box<[Task]>> {
        None
    }
    /// Attempts to perform this task, returning whether it succeeded or not.
    ///
    /// Note that you must confirm completion of all this task's dependencies before calling this method
    fn perform(&self) -> Result<()>;

    /// Run this task and all dependencies
    fn execute(self) -> Result<()>
    where
        Task: From<Self>,
    {
        if let Some(deps) = self.dependencies() {
            for task in Vec::from(deps).into_iter().filter(not_completed) {
                task.execute()?;
            }
        };
        let res = self.perform();
        complete_task(self.into());
        res
    }
}

#[derive(Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildOxideGL {
    /// Build OxideGL with debug assertions
    #[arg(short, long, default_value_t = true)]
    debug_assertions: bool,
    /// Build OxideGL with the "release" profile instead of "dev"
    #[arg(short, long, default_value_t = false)]
    release: bool,
    /// Build OxideGL targetting the current CPU's featureset instead of the more conservative default
    #[arg(short, long, default_value_t = false)]
    target_native_cpu: bool,
    /// Maximum logging level to compile OxideGL with (valid options are: "off", "error", "warn", "info", "debug", and "trace")
    #[arg(short, long, default_value = "trace")]
    logging_level: String,
}
impl TaskTrait for BuildOxideGL {
    fn dependencies(&self) -> Option<Box<[Task]>> {
        None
    }

    fn perform(&self) -> Result<()> {
        todo!()
    }
}

#[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenGLFWBuild {
    /// Whether to generate a release or debug build configuration
    #[arg(short, long, default_value_t = false)]
    release: bool,
}
impl TaskTrait for GenGLFWBuild {
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some(Box::new([GetGLFW {}.into()]))
    }

    fn perform(&self) -> Result<()> {
        let p = PathBuf::from("oxidegl-glfw").join(if self.release { "release" } else { "debug" });
        let _ = create_dir(&p);
        if read_dir(&p).is_err() {
            bail!(
                "could not confirm creation or presence of GLFW build directory: {}",
                &p.as_os_str().to_str().unwrap()
            )
        }
        let mut c = Command::new("cmake");
        c.args(["-S", "oxidegl-glfw", "-B"])
            .arg(&p)
            .arg("-D")
            .stderr(Stdio::inherit());
        if self.release {
            c.arg("CMAKE_BUILD_TYPE=Release")
        } else {
            c.arg("CMAKE_BUILD_TYPE=Debug")
        };
        let out = c.output()?;
        if !out.status.success() {
            bail!(
                "CMake errored while generating a build directory for GLFW at {}",
                p.as_os_str().to_str().unwrap()
            )
        }
        Ok(())
    }
}

#[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildGLFW {
    /// Whether to build a release or debug build configuration
    #[arg(short, long, default_value_t = false)]
    release: bool,
}
impl TaskTrait for BuildGLFW {
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some(Box::new([GenGLFWBuild {
            release: self.release,
        }
        .into()]))
    }

    fn perform(&self) -> Result<()> {
        let out = Command::new("make")
            .arg("-j")
            .arg("4")
            .current_dir(PathBuf::from("oxidegl-glfw").join(if self.release {
                "release"
            } else {
                "debug"
            }))
            .stderr(Stdio::inherit())
            .output()?;
        if !out.status.success() {
            bail!("error from make");
        }
        Ok(())
    }
}

#[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenerateBindings {
    /// Directory to place the generated .rs file in
    #[arg(short, long, default_value = "xtask/generated")]
    output_dir: PathBuf,
    /// Whether to generate placeholder implementations (unimplemented.rs)
    #[arg(short, long, default_value_t = false)]
    placeholder: bool,
    /// Whether to generate dispatch implementations (gl_core.rs)
    #[arg(short, long, default_value_t = false)]
    dispatch: bool,
    /// Whether to generate enums (enums.rs)
    #[arg(short, long, default_value_t = false)]
    enums: bool,
    /// A list of OpenGL Reference page names whose functions will be omitted from the generated code
    #[arg(short = 'x', long, default_value = "")]
    exclude_modules: Vec<String>,
}
impl TaskTrait for GenerateBindings {
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some([GetKhronosStuff {}.into()].into())
    }

    fn perform(&self) -> Result<()> {
        let out_dir = PathBuf::from(&self.output_dir);
        std::fs::create_dir_all(&out_dir)?;
        let spec = std::fs::read_to_string("reference/OpenGL-Registry/xml/gl.xml")?;
        let spec_doc = roxmltree::Document::parse(&spec)?;
        let (funcs, enums, group_map) = get_vals(&spec_doc, &self.exclude_modules)?;
        if self.placeholder | self.dispatch | self.enums {
            if self.placeholder {
                let path_to_write = out_dir.join("unimplemented.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_placeholder_impl(&mut writer, &funcs)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
            }
            if self.dispatch {
                let path_to_write = out_dir.join("gl_core.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_dispatch_impl(&mut writer, &funcs)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
            }
            if self.enums {
                let path_to_write = out_dir.join("enums.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_enum_impl(&mut writer, &enums, &group_map)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
            }
        }

        Ok(())
    }
}
fn rustfmt_file(path: impl AsRef<Path>) -> Result<()> {
    let mut s = Command::new("rustfmt").arg(path.as_ref()).spawn()?;
    if !s.wait()?.success() {
        bail!("rustfmt did not exit successfully! this means we generated malformed code");
    }
    Ok(())
}

#[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RunTest {
    #[arg(short, long, default_value = "glfw-triangle")]
    test_name: String,
}
impl TaskTrait for RunTest {
    fn perform(&self) -> Result<()> {
        todo!()
    }
}
macro_rules! stub_arg {
    ($name:ident) => {
        #[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct $name {}
    };
}
stub_arg!(GetGLFW);
impl TaskTrait for GetGLFW {
    fn perform(&self) -> Result<()> {
        submodule_init(&["oxidegl-glfw"])
    }
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some([GetXcodeCommandLineTools {}.into()].into())
    }
}
stub_arg!(GetKhronosStuff);
impl TaskTrait for GetKhronosStuff {
    fn perform(&self) -> Result<()> {
        submodule_init(&["reference/OpenGL-Registry", "reference/OpenGL-Refpages"])
    }
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some([GetXcodeCommandLineTools {}.into()].into())
    }
}
stub_arg!(GetXcodeCommandLineTools);
impl TaskTrait for GetXcodeCommandLineTools {
    fn perform(&self) -> Result<()> {
        let out = std::process::Command::new("xcode-select")
            .arg("--install")
            .output()?;
        let stderr = String::from_utf8(out.stderr)?;
        if stderr.contains("already installed") {
            return Ok(());
        }
        if !out.status.success() {
            error!("xcode-select error: {stderr}");
            bail!("error from xcode-select!");
        }
        let stdout = String::from_utf8(out.stdout)?;
        if stdout.contains("install requested") {
            info!("Requested install of XCode command line tools.\nPlease confirm the installation and run this command again when it is finished.");
            exit(0);
        } else {
            bail!("unexpected successful execution of xcode-select, output: {stdout}");
        }
    }
}
fn submodule_init(paths: &[&str]) -> Result<()> {
    info!(
        "initializing git submodule(s) at: {}",
        paths
            .iter()
            .flat_map(|v| [*v, "\n"])
            .scan(None, |state, item| { std::mem::replace(state, Some(item)) })
            .collect::<String>()
    );
    if !process::Command::new("git")
        .args(["submodule", "update", "--init", "--recursive", "--"])
        .args(paths)
        .spawn()?
        .wait()?
        .success()
    {
        bail!("git process errored while trying to update a submodule")
    }
    Ok(())
}
