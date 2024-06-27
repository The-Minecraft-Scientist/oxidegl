#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]

pub mod context;

#[allow(clippy::undocumented_unsafe_blocks)]
mod dispatch;

#[allow(non_upper_case_globals, unused)]
pub mod enums;
