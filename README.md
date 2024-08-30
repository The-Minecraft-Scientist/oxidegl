# OxideGL
An open-source OpenGL 4.6 Core implementation atop Apple's Metal API, written in Rust

## Goals
 * Easy to develop and hack on
    * builds a working artifact with a simple `cargo build`
    * reasonable build times (currently ~55s on a 2010 xeon w3690)
    * focus on safety and correctness before maximum performance (prefer safe code, optimize where needed with unsafe later)
 * Run Minecraft
 * Have decent performance
 * Automate as much tedium as possible by leveraging OpenGL's machine-readable spec and reference

## Non-Goals
 * Full OpenGL 4.6 compliance (Metal itself is not specified strongly enough for this to be worthwhile)
 * Multithreading support


## Current State
 * xtask system capable of:
    * building and testing OxideGL and its GLFW fork
    * code generating a placeholder GL implementation, as well as Rust wrapper enums for many functions that take `GLenum` parameters
 * Context creation and linkage with GLFW
 * Generic context parameter lookup (`glGet` and co.)
 * full implementation of VAOs (some features disabled due to limitations in shader translation)
 * initial implementation of buffers and buffer binding (currently missing buffer copy operations)
 * initial implementation of shaders and shader programs
 * initial implementation of shader translation with `naga` (only allows 1 shader per stage, global uniforms not working, no compute shaders)


## Building/XTasks
To get a full list of OxideGL `xtask` subcommands, run `cargo xtask --help` anywhere in this repository. 
All tasks implicitly run their dependencies (e.g. `cargo xtask build-glfw` implies `cargo xtask gen-glfw-build` etc), so you don't need to run dependencies of tasks manually.

## Linting
This project uses Clippy for linting. If you use VS Code or a derivative thereof, this should be enabled already (via a `.vscode` with the appropriate configuration in the repository root). If not, check if your IDE supports changing the rust analyzer check command or simply use `cargo clippy` in your shell.

## UB Propagation, Errors and `GL_NOERROR`
Since this implementation only provides a `GL_NOERROR` context, its behavior in an errored state may be undefined according to the GL Spec. In release builds, upon recieving erroring input, OxideGL will propagate UB (i.e. raise GL UB to Rust language level UB) where there is a measurable and significant performance advantage in doing so (e.g. skipping array bounds checking in a hot path), otherwise it will abort the calling program. However, in debug builds, OxideGL will attempt to abort with a helpful message (and a stack trace including the calling function that caused the error) as soon as possible after recieving an incorrect input.