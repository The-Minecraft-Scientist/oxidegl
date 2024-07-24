use std::process::Command;

fn main() {
    let commit_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .unwrap_or("Commit hash N/A".to_string());
    println!("cargo:rustc-env=OXIDEGL_COMMIT_HASH={commit_hash}");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=build.rs");
}
