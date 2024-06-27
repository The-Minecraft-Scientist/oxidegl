use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn main() {
    println!("Hello, world!");
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subcommand to run
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Clone)]
enum Commands {
    /// Build liboxidegl.dylib
    Build {
        /// Whether to build OxideGL with or without debug assertions
        #[arg(short, long)]
        debug_assertions: bool,
        /// Whether to build OxideGL with the "dev" or "release" profile
        #[arg(short, long)]
        release: bool,
        /// Whether to build OxideGL targetting the current CPU
        /// Maximum logging level to compile OxideGL with (valid options are: "off", "error", "warn", "info", "debug", and "trace")
        #[arg(short, long)]
        logging_level: Option<String>,
    },
    /// Init GLFW git submodule
    GetGLFW,
    /// Build OxideGL GLFW
    BuildGLFW,
    /// Init OpenGL-Refpages and -Registry submodules (required to run codegen)
    GetKhronosStuff,

    Generate {
        /// Directory to place the generated .rs file in
        output_dir: Option<PathBuf>,
        /// Whether to generate placeholder implementations (unimplemented.rs)
        generate_placeholder: bool,
        /// Whether to generate dispatch implementations (gl_core.rs)
        generate_dispatch: bool,
        /// Whether to generate enums (enums.rs)
        generate_enums: bool,
    },
    /// Runs a "test". A test is a binary
    RunTest { test_name: Option<String> },
}
