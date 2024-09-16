#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]

use objc2::{rc::Retained, runtime::ProtocolObject};

pub mod context;
pub mod entry_point;
#[cfg(feature = "nsgl_shim")]
pub mod nsgl_shim;

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
#[must_use]
pub fn type_name<T: ?Sized>() -> &'static str {
    std::any::type_name::<T>().split("::").last().unwrap()
}
pub type ProtoObjRef<T> = Retained<ProtocolObject<T>>;
