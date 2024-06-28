use std::{
    path::PathBuf,
    process::{self, exit},
    sync::{Arc, OnceLock},
};

use anyhow::{bail, Context, Result};
use clap::{builder::Str, Args, Subcommand};
use dashmap::DashSet;
use enum_dispatch::enum_dispatch;
use log::{error, info};

use crate::{
    open_file_writer,
    spec_parse::{get_vals, write_dispatch_impl, write_enum_impl, write_placeholder_impl},
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
    /// Init GLFW git submodule if it hasn't been already
    GetGLFW,
    /// Trigger a download of the XCode command line tools if they aren't present
    GetXcodeCommandLineTools,
    /// Init OpenGL-Refpages and -Registry submodules (required to run codegen)
    GetKhronosStuff,
    /// Generates a GLFW build folder in oxidegl-glfw/build
    GenGLFWBuild,
    /// Build OxideGL GLFW (requires XCode command line tools for clang, cmake and make)
    BuildGLFW,
    /// Generate OxideGL rust GL bindings/placeholder impls
    GenerateBindings,
    /// Run a "test". A test is a name=path_to_binary pair given in tests.txt.
    /// Paths given are relative to the workspace root.
    RunTest,
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
        None
    }

    fn perform(&self) -> Result<()> {
        todo!()
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
        None
    }

    fn perform(&self) -> Result<()> {
        todo!()
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
        let (funcs, enums) = get_vals(&spec_doc)?;
        if self.placeholder | self.dispatch | self.enums {
            if self.placeholder {
                let mut writer = open_file_writer(out_dir.join("unimplemented.rs"))?;
                write_placeholder_impl(&mut writer, &funcs)?;
            }
            if self.dispatch {
                let mut writer = open_file_writer(out_dir.join("gl_core.rs"))?;
                write_dispatch_impl(&mut writer, &funcs)?;
            }
            if self.enums {
                let mut writer = open_file_writer(out_dir.join("enums.rs"))?;
                write_enum_impl(&mut writer, &enums)?;
            }
        }

        Ok(())
    }
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
        submodule_init(&["xtask/OpenGL-Registry", "xtask/OpenGL-Refpages"])
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
