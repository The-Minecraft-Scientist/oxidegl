# OxideGL
An open-source OpenGL 4.6 Core implementation atop Apple's Metal API, written in Rust

## Goals
 * Easy to develop and hack on
    * builds a working artifact with a simple `cargo build`
    * reasonable initial build times (around a minute on a 2010 6 core xeon)
    * focus on safety and correctness before maximum performance (prefer safe code, optimize where needed with unsafe later)
 * Run Minecraft
 * Have decent performance
 * Automate as much tedium as possible by leveraging OpenGL's machine-readable spec and reference

## Non-Goals
 * Full OpenGL 4.6 compliance (Metal itself is not specified strongly enough for this to be worthwhile)
 * Multithreading support/share context (this may actually be practical, `Context` is currently still `Send`)


## Current State
 * xtask system capable of:
    * building and OxideGL and its GLFW fork and running other utilities
    * code generation of a placeholder GL implementation, as well as Rust enums for the allowed parameters of many functions that take `GLenum`
 * Context creation and linkage with GLFW
 * Shim reimplementation of NSOpenGLContext for convenient testing with programs using off-the-shelf GLFW/SDL/etc builds
 * Generic context parameter lookup (`glGet` and co.)
 * full implementation of VAOs (some features disabled due to limitations in shader translation)
 * initial implementation of buffers and buffer binding (currently missing buffer copy operations)
 * initial implementation of shaders and shader programs
 * initial implementation of shader translation using the `glslang` and `spirv_cross2` binding crates
 * initial GL state -> metal state translation

## Version Requirements
MSRV is the latest stable release.
Minimum supported Metal version is Metal 2.2 (corresponds to MacOS 10.15 and iOS 13)

## Building/XTasks
To get a full list of OxideGL `xtask` subcommands, run `cargo xtask --help` anywhere in this repository. 
All tasks implicitly run the necessary dependencies (e.g. `cargo xtask build-glfw` implies `cargo xtask gen-glfw-build` etc), so you don't need to run dependencies of tasks manually.

## Linting
This project uses Clippy for linting. If you use VS Code or a derivative thereof, this should be enabled already (via a `.vscode` with the appropriate configuration in the repository root). If not, check if your IDE supports changing the rust analyzer check command or simply run `cargo clippy` from your shell.

## UB Propagation, Errors and `GL_NOERROR`
Since this implementation only provides a `GL_NOERROR` context, its behavior in an errored state may be undefined according to the GL spec. In release builds, upon recieving erroring input, OxideGL will propagate UB (i.e. raise GL UB to Rust language UB) where there is a measurable and significant performance advantage in doing so (e.g. skipping array bounds checking in a hot path), otherwise it will abort the calling program. However, in debug builds, OxideGL will attempt to abort with a helpful message (and a stack trace including the calling function that caused the error) as soon as possible after recieving an incorrect input.