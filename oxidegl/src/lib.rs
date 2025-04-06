#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::missing_errors_doc,
    clippy::match_bool
)]

use objc2::{rc::Retained, runtime::ProtocolObject};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub mod context;
pub mod entry_point;

#[allow(clippy::undocumented_unsafe_blocks)]
mod dispatch;

mod device_properties;
#[cfg(feature = "nsgl_shim")]
mod nsgl_shim;

#[allow(non_upper_case_globals, unused)]
pub mod enums;

pub mod util;
