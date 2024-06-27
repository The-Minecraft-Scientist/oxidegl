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
    command: Tasks,
}
#[derive(Subcommand, Clone)]
enum Tasks {
    /// Build liboxidegl.dylib
    Build {
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
    },
    /// Init GLFW git submodule
    GetGLFW,
    /// Build OxideGL GLFW
    BuildGLFW,
    /// Init OpenGL-Refpages and -Registry submodules (required to run codegen)
    GetKhronosStuff,

    Generate {
        /// Directory to place the generated .rs file in
        #[arg(short, long, default_value = ".")]
        output_dir: PathBuf,
        /// Whether to generate placeholder implementations (unimplemented.rs)
        #[arg(short = 'p', long, default_value_t = true)]
        generate_placeholder: bool,
        /// Whether to generate dispatch implementations (gl_core.rs)
        #[arg(short = 'd', long, default_value_t = true)]
        generate_dispatch: bool,
        /// Whether to generate enums (enums.rs)
        #[arg(short = 'e', long, default_value_t = true)]
        generate_enums: bool,
    },
    /// Run a "test". A test is a name=path_to_binary pair given in tests.txt.
    /// Paths given are relative to the workspace root
    RunTest {
        #[arg(short, long, default_value = "glfw-triangle")]
        test_name: String,
    },
}
