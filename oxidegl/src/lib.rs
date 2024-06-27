#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

pub mod context;
#[allow(non_upper_case_globals, unused)]
pub mod enums;
mod gl;
