pub type ProtoObjRef<T> = Retained<ProtocolObject<T>>;
use std::fmt::Debug;
#[must_use]
/// Prints the trimmed type name (e.g. with all paths removed). May not work correctly in all cases (likely breaks for local structs, futures, closures etc)
/// # Stability note
/// The output of this function is not guaranteed to be stable across rust versions (i.e. we forward the *lack* of stability guarantees inherent to [`std::any::type_name`])
/// As such, all usages of [`trimmed_type_name`] should be purely for programmer-facing debug output,
/// program behavior should not depend on the contents of the output.
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

macro_rules! bitflag_bits {
    {
        $( #[$attr:meta] )*
        $v:vis struct $name:ident: $t:ident bits: {
            $(
                $( #[doc = $doc:literal] )*
                $bit_name:ident: $bit:expr
            ),+ $(,)?
        }
    } => {
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

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

pub(crate) use bitflag_bits;

/// [`unreachable!`](unreachable!), but it reduces to [`core::hint::unreachable_unchecked()`] in builds without debug assertions.
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

/// takes in a place expression, a value expression, and an action
macro_rules! run_if_changed {
    ( $place:expr ;= $new_val:expr => $action:expr ) => {
        if $place != $new_val {
            $place = $new_val;
            $action
        }
    };
}
use objc2::{rc::Retained, runtime::ProtocolObject};
pub(crate) use run_if_changed;

pub(crate) const unsafe fn transmute_unchecked<Src, Dst>(value: Src) -> Dst {
    union Transmute<Src, Dst> {
        src: ManuallyDrop<Src>,
        dst: ManuallyDrop<Dst>,
    }
    ManuallyDrop::into_inner(unsafe {
        Transmute {
            src: ManuallyDrop::new(value),
        }
        .dst
    })
}
