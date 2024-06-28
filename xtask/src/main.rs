use std::{env::set_current_dir, path::PathBuf};

use clap::Parser;
use xtask::tasks::TaskTrait;

fn main() {
    let args = Args::parse();
    let xtask_dir = env!("CARGO_MANIFEST_DIR");
    // Parent directory of xtask manifest dir is workspace root
    set_current_dir(PathBuf::from(xtask_dir).parent().unwrap()).unwrap();
    dbg!(&args);
    args.command.execute().unwrap();
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subcommand to run
    #[command(subcommand)]
    command: xtask::tasks::Task,
}
