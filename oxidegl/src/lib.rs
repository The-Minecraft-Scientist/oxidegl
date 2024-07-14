#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]
#![allow(unstable_name_collisions)]

pub mod context;

#[allow(clippy::undocumented_unsafe_blocks)]
mod dispatch;

#[allow(non_upper_case_globals, unused)]
pub mod enums;

#[macro_export]
macro_rules! debug_unreachable {
    (unsafe $($msg:tt)*) => {
        #[cfg(debug_assertions)]
        unreachable!($($msg)*);
        #[cfg(not(debug_assertions))]
        unsafe {core::hint::unreachable_unchecked()}
    };
}
