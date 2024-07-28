#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]
#![allow(unstable_name_collisions)]

use std::error::Error;

pub mod context;
pub mod entry_point;

#[allow(clippy::undocumented_unsafe_blocks)]
mod dispatch;

#[allow(non_upper_case_globals, unused)]
pub mod enums;

#[macro_export]
/// [`unreachable!`](unreachable!), but it reduces to [`std::hint::unreachable_unchecked()`] in builds without debug assertions.
/// Usages must start with the `unsafe` keyword to indicate that this macro has the same semantics as unreachable unchecked
macro_rules! debug_unreachable {
    ($($msg:tt)*) => {
        {
            // Need to do something "unsafe" to advertise that this macro's semantics are unsafe in debug builds
            #[allow(clippy::useless_transmute)]
            let _: () = ::core::mem::transmute(());
            #[cfg(debug_assertions)]
            unreachable!($($msg)*);
            #[cfg(not(debug_assertions))]
            ::core::hint::unreachable_unchecked()
        }
    };
}

pub(crate) trait OptionResultExt<T> {
    #[track_caller]
    /// # Safety
    /// Caller must ensure that:
    /// This Result or Option is Some or Ok OR that it is acceptable to cause UB if this Result or Option is not Some or Ok
    unsafe fn debug_expect(self, msg: &str) -> T;
    #[track_caller]
    /// # Safety
    /// Caller must ensure that:
    /// This Result or Option is Some or Ok OR that it is acceptable to cause UB if this Result or Option is not Some or Ok
    unsafe fn debug_unwrap(self) -> T;
}
impl<T, E: Error> OptionResultExt<T> for Result<T, E> {
    unsafe fn debug_expect(self, msg: &str) -> T {
        #[cfg(debug_assertions)]
        return self.expect(msg);
        #[cfg(not(debug_assertions))]
        // Safety: Caller ensures self is Ok (or that we are allowed to cause UB)
        return unsafe { self.unwrap_unchecked() };
    }

    unsafe fn debug_unwrap(self) -> T {
        #[cfg(debug_assertions)]
        return self.unwrap();
        #[cfg(not(debug_assertions))]
        // Safety: Caller ensures self is Ok (or that we are allowed to cause UB)
        return unsafe { self.unwrap_unchecked() };
    }
}
impl<T> OptionResultExt<T> for Option<T> {
    unsafe fn debug_expect(self, msg: &str) -> T {
        #[cfg(debug_assertions)]
        return self.expect(msg);
        #[cfg(not(debug_assertions))]
        // Safety: Caller ensures self is Some (or that we are allowed to cause UB)
        return unsafe { self.unwrap_unchecked() };
    }

    unsafe fn debug_unwrap(self) -> T {
        #[cfg(debug_assertions)]
        return self.unwrap();
        #[cfg(not(debug_assertions))]
        // Safety: Caller ensures self is Some (or that we are allowed to cause UB)
        return unsafe { self.unwrap_unchecked() };
    }
}

pub fn type_name<T>() -> &'static str {
    std::any::type_name::<T>().split("::").last().unwrap()
}
