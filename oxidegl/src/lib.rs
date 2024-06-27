#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]

pub mod context;
mod dispatch;
#[allow(non_upper_case_globals, unused)]
pub mod enums;
