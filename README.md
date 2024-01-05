# OxideGL
An open-source OpenGL 4.6 Core implementation atop Apple's Metal API, written in Rust

## Goals
 * Easy to develop and hack on (builds a working artifact with a simple `cargo build`)
 * Run Minecraft
 * Be reasonably efficient
 * Leverage Rust's strengths wherever possible
 * Automate as much tedium as possible by leveraging OpenGL's machine-readable spec and reference

## Non-Goals
 * Full OpenGL 4.6 compliance

## Current State
 * everything is broken, There's so much UB hiding in this thing that it does different things based on what parent process it has
