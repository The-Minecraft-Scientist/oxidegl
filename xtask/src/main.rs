use std::{env::set_current_dir, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use xtask::tasks::TaskTrait;

fn main() -> Result<()> {
    let args = Args::parse();
    // Parent directory of xtask manifest dir is workspace root
    let xtask_dir = env!("CARGO_MANIFEST_DIR");
    set_current_dir(PathBuf::from(xtask_dir).parent().unwrap()).unwrap();
    dbg!(&args);
    args.command.execute()
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subcommand to run
    #[command(subcommand)]
    command: xtask::tasks::Task,
}
