use std::{
    mem::{ManuallyDrop, MaybeUninit},
    rc::Rc,
};

#[derive(Clone, Copy, Debug)]
pub enum GLItemSingle {
    Bool(bool),
    Int(i32),
    Float(f32),
    Double(f64),
}
#[derive(Debug, Clone)]
pub enum OxideGLItem {
    Single(GLItemSingle),
    Array(Rc<[GLItemSingle]>),
}
impl OxideGLItem {
    pub fn arr(&self) -> &[GLItemSingle] {
        let iter = match self {
            Self::Single(s) => std::slice::from_ref(s),
            Self::Array(a) => &**a,
        };
        iter
    }
}
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss
)]
impl GLItemSingle {
    #[inline]
    pub fn into_bool(self) -> bool {
        match self {
            Self::Bool(b) => b,
            Self::Int(i) => i != 0,
            Self::Float(f) => f != 0.0,
            Self::Double(d) => d != 0.0,
        }
    }
    #[inline]
    pub fn into_float(self) -> f32 {
        match self {
            Self::Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Int(i) => i as f32,
            Self::Float(f) => f,
            Self::Double(d) => d as f32,
        }
    }
    #[inline]
    pub fn into_int(self) -> i32 {
        match self {
            Self::Bool(b) => i32::from(b),
            Self::Int(i) => i,
            Self::Float(i) => i as i32,
            Self::Double(d) => d as i32,
        }
    }
    #[inline]
    pub fn into_double(self) -> f64 {
        match self {
            Self::Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Int(i) => f64::from(i),
            Self::Float(f) => f64::from(f),
            Self::Double(d) => d,
        }
    }
}
macro_rules! impl_item_from {
    // If people want to misuse the GLGet API, they deserve to lose a bit of precision or have a value truncated
    ($i:ident, $t:ty) => {
        #[allow(
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation,
            clippy::cast_precision_loss
        )]
        impl From<$t> for GLItemSingle {
            fn from(value: $t) -> Self {
                Self::$i(value)
            }
        }
    };
}
impl_item_from!(Bool, bool);
impl_item_from!(Int, i32);
impl_item_from!(Float, f32);
impl_item_from!(Double, f64);
impl From<u32> for GLItemSingle {
    #[allow(clippy::cast_possible_wrap)]
    fn from(value: u32) -> Self {
        Self::Int(value as i32)
    }
}

impl<T> From<T> for OxideGLItem
where
    T: Into<GLItemSingle>,
{
    fn from(value: T) -> Self {
        Self::Single(value.into())
    }
}
impl<T, const N: usize> From<[T; N]> for OxideGLItem
where
    T: Into<GLItemSingle> + Copy,
{
    fn from(value: [T; N]) -> Self {
        let mut array = [MaybeUninit::<GLItemSingle>::uninit(); N];
        for (idx, val) in value.into_iter().enumerate() {
            array[idx].write(val.into());
        }

        let mut m = ManuallyDrop::new(array);
        let v = *unsafe {
            std::mem::transmute::<&mut [MaybeUninit<GLItemSingle>; N], &mut [GLItemSingle; N]>(
                &mut m,
            )
        };
        OxideGLItem::Array(Rc::new(v))
    }
}
