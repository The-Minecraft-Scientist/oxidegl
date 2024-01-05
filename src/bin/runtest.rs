pub fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::fs::copy(
        format!("{}/target/debug/liboxidegl.dylib", dir),
        "/usr/local/bin/liboxidegl.dylib",
    )
    .unwrap();
    std::process::Command::new("cargo")
        .arg("build")
        .current_dir(&dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let c = std::process::Command::new(format!(
        "{}/oxidegl-glfw/build/examples/triangle-opengl.app/Contents/MacOS/triangle-opengl",
        dir
    ))
    .stderr(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::inherit())
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
}
