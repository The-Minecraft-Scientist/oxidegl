# OxideGL
An open-source OpenGL 4.6 Core implementation atop Apple's Metal API, written in Rust

## Goals
 * Easy to develop and hack on (builds a working artifact with a simple `cargo build`) with reasonable initial build times
 * Run Minecraft
 * Have decent performance
 * Automate as much tedium as possible by leveraging OpenGL's machine-readable spec and reference

## Non-Goals
 * Full OpenGL 4.6 compliance (Metal itself is not specified strongly enough for this to be worthwhile)
 * Multithreading support

## Building/XTasks
To get a full list of OxideGL `xtask` subcommands, run `cargo xtask --help` anywhere in this repository. 
All tasks implicitly run their dependencies (e.g. `cargo xtask build-glfw` implies `cargo xtask gen-glfw-build` etc), so you don't need to run dependencies of tasks manually.

## Current State
 * xtask system capable of:
    * building and testing OxideGL and its GLFW fork
    * code generating a placeholder GL implementation, as well as Rust wrapper enums for many functions that take `GLenum` parameters
 * Context creation and linkage with GLFW works, the GLFW example successfully calls into OxideGL.

## UB Propagation, Errors and `GL_NOERROR`
Since this implementation only provides a `GL_NOERROR` context, its behavior in an errored state may be undefined according to the GL Spec. In release builds, upon recieving erroring input, OxideGL will propagate UB where there is a performance advantage in doing so (e.g. skipping array bounds checking), otherwise it will abort the calling program. However, in debug builds, OxideGL will attempt to abort with a helpful message (and a stack trace including the calling function that caused the error) as soon as possible after recieving an incorrect input.

