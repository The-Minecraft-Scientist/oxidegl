pub fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::process::Command::new("cargo")
        .arg("build")
        .current_dir(&dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let c = std::process::Command::new(format!(
        "{}/oxidegl-glfw/debug/examples/triangle-opengl.app/Contents/MacOS/triangle-opengl",
        dir
    ))
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

    let _ = std::fs::remove_file("/usr/local/lib/liboxidegl.dylib");
    std::fs::copy(
        format!("{}/target/debug/liboxidegl.dylib", dir),
        "/usr/local/lib/liboxidegl.dylib",
    )
    .unwrap();
    dbg!(c);
}
