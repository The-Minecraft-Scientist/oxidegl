use std::{
    cell::OnceCell,
    env::set_current_dir,
    iter::Once,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, OnceLock, RwLock},
};

use clap::{FromArgMatches, Parser, Subcommand};
use dashmap::DashSet;
use enum_dispatch::enum_dispatch;
use xtask::tasks::{GetKhronosStuff, Task, TaskTrait};

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
