#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]

use objc2::{rc::Retained, runtime::ProtocolObject};

pub mod context;
pub mod entry_point;

#[allow(clippy::undocumented_unsafe_blocks)]
mod dispatch;
#[cfg(all(feature = "nsgl_shim", not(test)))]
mod nsgl_shim;

#[allow(non_upper_case_globals, unused)]
pub mod enums;

/// [`unreachable!`](unreachable!), but it reduces to [`std::hint::unreachable_unchecked()`] in builds without debug assertions.
macro_rules! debug_unreachable {
    ($($msg:tt)*) => {
        {
            // Need to do something unsafe to surface this macro's unsafety in builds with debug assertions enabled
            #[allow(clippy::useless_transmute)]
            let _: () = ::core::mem::transmute(());
            #[cfg(debug_assertions)]
            unreachable!($($msg)*);
            #[cfg(not(debug_assertions))]
            ::core::hint::unreachable_unchecked()
        }
    };
}

pub(crate) use debug_unreachable;

#[must_use]
pub(crate) fn trimmed_type_name<T: ?Sized>() -> &'static str {
    let s = std::any::type_name::<T>();

    let mut last_ident_start_index = 0;
    let mut last_substr = "";
    let mut gen_flag = false;

    for substr in s.split("::") {
        last_substr = substr;
        if substr.contains('<') {
            gen_flag = true;
            break;
        }
        last_ident_start_index += substr.len() + 2;
    }
    if gen_flag {
        &s[last_ident_start_index..]
    } else {
        last_substr
    }
}
pub type ProtoObjRef<T> = Retained<ProtocolObject<T>>;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub struct NoDebug<T> {
    inner: T,
}

impl<T> Debug for NoDebug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = trimmed_type_name::<T>();
        f.debug_struct(&format!("skipped debug for {n}")).finish()
    }
}

impl<T> From<T> for NoDebug<T> {
    fn from(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T> Deref for NoDebug<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NoDebug<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Default for NoDebug<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

macro_rules! bitflag_bits {{
    $( #[$attr:meta] )*
     $v:vis struct $name:ident: $t:tt bits: {
        $(
            $( #[doc = $doc:literal] )*
            $bit_name:ident: $bit:expr ),+ $(,)?
    }} => {
        ::bitflags::bitflags! {
            $(#[$attr])*
            $v struct $name: $t {
                $(
                    $( #[doc = $doc] )*
                    const $bit_name = 1 << $bit);+
                ;
            }
        }
    }
}

pub(crate) use bitflag_bits;
