use std::{
    fs::{copy, create_dir, read_dir, remove_file},
    path::{Path, PathBuf},
    process::{self, exit, Command, Stdio},
    sync::{Arc, OnceLock},
};

use anyhow::{bail, Result};
use clap::{Args, Subcommand};
use dashmap::DashSet;
use enum_dispatch::enum_dispatch;

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
    #[command(name = "build")]
    BuildOxideGL,

    /// Build OxideGL GLFW (requires XCode command line tools for clang, cmake and make)
    BuildGLFW,

    /// Generate OxideGL rust GL bindings/placeholder impls
    GenerateBindings,

    /// Init GLFW git submodule if it hasn't been already
    GetGLFW,

    /// Trigger a download of the XCode command line tools if they aren't present
    GetXcodeCommandLineTools,

    /// Init OpenGL-Refpages and -Registry submodules (required to run codegen)
    GetKhronosStuff,

    /// Generates a GLFW build folder in oxidegl-glfw/build
    GenGLFWBuild,

    /// Runs `cargo fix --allow-dirty && cargo clippy --fix --allow-dirty`
    #[command(name = "fix")]
    CargoFix,
}
#[enum_dispatch(Task)]
pub trait TaskTrait: Sized {
    /// Returns a list of task dependencies that must be completed before this task can run. Each individual Task in the array may be performed in parallel.
    /// If you need serial execution of a list of dependecies, chain the dependency to be executed serially from its dependent tasks' `dependencies` function.
    ///
    /// Default implemention returns `None`
    ///
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
    /// Whether to force enable debug assertions
    #[arg(short, long, default_value_t = true)]
    debug_assertions: bool,
    /// Build OxideGL with the "release" profile instead of "dev"
    #[arg(short, long, default_value_t = false)]
    release: bool,
    /// Build OxideGL targetting the current CPU's featureset instead of the more conservative default
    #[arg(short = 'n', long, default_value_t = false)]
    target_cpu_native: bool,
    /// Maximum logging level to compile OxideGL with (valid options are: "off", "error", "warn", "info", "debug", and "trace")
    #[arg(short, long, default_value = "trace")]
    logging_level: String,
    /// Install the compiled binary to /usr/local/lib, replacing the current one if it exists.
    #[arg(short, long, default_value_t = false)]
    install: bool,
}
impl TaskTrait for BuildOxideGL {
    fn dependencies(&self) -> Option<Box<[Task]>> {
        None
    }

    fn perform(&self) -> Result<()> {
        println!("Building OxideGL");
        let mut c = Command::new("cargo");
        c.arg("build");

        if self.release {
            c.arg("-r");
            c.env("OXIDEGL_RELEASE", "1");
        }
        c.env("CARGO_TARGET_DIR", "/tmp/oxidegl-target/");
        c.args(["--features", &format!("max_log_{}", self.logging_level)]);
        if self.debug_assertions {
            c.args(["--config", "build-override.debug-assertions=true"]);
        }

        if self.target_cpu_native {
            c.args([
                "--config",
                "build.rustflags=[\"-C\", \"target-cpu=native\"]",
            ]);
        }
        c.current_dir("oxidegl");
        if !c.spawn()?.wait()?.success() {
            bail!("failed to compile OxideGL!");
        }
        if self.install {
            let _ = remove_file("/usr/local/lib/liboxidegl.dylib");
            copy(
                format!(
                    "/tmp/oxidegl-target/{}/liboxidegl.dylib",
                    if self.release { "release" } else { "debug" }
                ),
                "/usr/local/lib/liboxidegl.dylib",
            )?;
            println!("Installed OxideGL to /usr/local/lib/liboxidegl.dylib");
        }
        Ok(())
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
        Some(Box::new([GetGLFW.into()]))
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
        println!(
            "Generated out of tree build directory for OxideGL-GLFW at {:?}",
            &p.canonicalize().unwrap()
        );
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
        println!("Built OxideGL-GLFW");
        Ok(())
    }
}

#[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenerateBindings {
    /// Directory to place the generated .rs files in
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
        let (funcs, enums, group_map) = get_vals(&spec_doc)?;
        if self.placeholder | self.dispatch | self.enums {
            if self.placeholder {
                let path_to_write = out_dir.join("unimplemented.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_placeholder_impl(&mut writer, &funcs)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
                println!("generated unimplemented.rs placeholder");
            }

            if self.dispatch {
                let path_to_write = out_dir.join("gl_core.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_dispatch_impl(&mut writer, &funcs)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
                println!("generated gl_core.rs dispatch");
            }
            if self.enums {
                let path_to_write = out_dir.join("enums.rs");
                let mut writer = open_file_writer(&path_to_write)?;
                write_enum_impl(&mut writer, &enums, &group_map)?;
                drop(writer);
                rustfmt_file(path_to_write)?;
                println!("generated enums.rs enums and groups");
            }
        }

        Ok(())
    }
}
fn rustfmt_file(path: impl AsRef<Path>) -> Result<()> {
    let mut s = Command::new("rustfmt").arg(path.as_ref()).spawn()?;
    if !s.wait()?.success() {
        bail!("rustfmt did not exit successfully! this means codegen generated malformed code!");
    }
    Ok(())
}

// #[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
// pub struct RunTest {
//     #[arg(short, long, default_value = "glfw-triangle")]
//     test_name: String,
// }
// impl TaskTrait for RunTest {
//     fn perform(&self) -> Result<()> {
//         todo!()
//     }
// }
macro_rules! stub_arg {
    ($name:ident) => {
        #[derive(clap::Args, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct $name;
    };
}
stub_arg!(GetGLFW);
impl TaskTrait for GetGLFW {
    fn perform(&self) -> Result<()> {
        submodule_init(&["oxidegl-glfw"])
    }
    fn dependencies(&self) -> Option<Box<[Task]>> {
        Some([GetXcodeCommandLineTools.into()].into())
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
stub_arg!(CargoFix);
impl TaskTrait for CargoFix {
    fn perform(&self) -> Result<()> {
        std::process::Command::new("cargo")
            .args(["fix", "--allow-dirty"])
            .spawn()?
            .try_wait()?;
        std::process::Command::new("cargo")
            .arg("clippy")
            .args(["--fix", "--allow-dirty"])
            .spawn()?
            .try_wait()?;
        Ok(())
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
            bail!("error from xcode-select!");
        }
        let stdout = String::from_utf8(out.stdout)?;
        if stdout.contains("install requested") {
            println!("Requested install of XCode command line tools.\nPlease confirm the installation and run this command again when it is finished.");
            exit(0);
        } else {
            bail!("unexpected successful execution of xcode-select, output: {stdout}");
        }
    }
}
fn submodule_init(paths: &[&str]) -> Result<()> {
    println!("Checking submodule(s) at: {}", paths.join(", "));
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
