# OxideGL
An open-source OpenGL 4.6 Core implementation atop Apple's Metal API, written in Rust

## Goals
 * Easy to develop and hack on (builds a working artifact with a simple `cargo build`) with reasonable initial build times
 * Run Minecraft
 * Have decent performance
 * Automate as much tedium as possible by leveraging OpenGL's machine-readable spec and reference

## Current Non-Goals
 * Full OpenGL 4.6 compliance
 * Multithreading support

## UB Propagation, Errors and `GL_NOERROR`
Since this implementation only provides a `GL_NOERROR` context, its behavior in an errored state may be undefined according to the GL Spec. In release builds, upon recieving erroring input, OxideGL will propagate UB where there is a performance advantage in doing so (e.g. skipping array bounds checking), otherwise it will abort the calling program. However, in debug builds, OxideGL will attempt to abort with a helpful message (and a stack trace including the calling function that caused the error) as soon as possible after recieving an incorrect input.

## Current State
 * xtask system capable of 
  * building and testing OxideGL and its GLFW fork
  * code generating a placeholder GL implementation, as well as Rust wrapper enums for many functions that take `GLenum` parameters
 * Context creation and linkage with GLFW works, the GLFW example successfully calls into OxideGL
