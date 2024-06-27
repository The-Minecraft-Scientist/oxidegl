use std::{
    mem::{ManuallyDrop, MaybeUninit},
    rc::Rc,
};

#[derive(Clone, Copy, Debug)]
pub enum OxideGLItemSingle {
    Bool(bool),
    Int(i32),
    Float(f32),
    Double(f64),
}
#[derive(Debug, Clone)]
pub enum OxideGLItem {
    Single(OxideGLItemSingle),
    Array(Rc<[OxideGLItemSingle]>),
}
impl OxideGLItem {
    pub fn arr<'a>(&'a self) -> &'a [OxideGLItemSingle] {
        let iter = match self {
            Self::Single(s) => std::slice::from_ref(s),
            Self::Array(a) => &**a,
        };
        iter
    }
}
impl OxideGLItemSingle {
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
            Self::Bool(b) => b as i32,
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
            Self::Int(i) => i as f64,
            Self::Float(f) => f as f64,
            Self::Double(d) => d,
        }
    }
}
impl From<bool> for OxideGLItemSingle {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<u32> for OxideGLItemSingle {
    fn from(value: u32) -> Self {
        Self::Int(value as i32)
    }
}
impl From<i32> for OxideGLItemSingle {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<f32> for OxideGLItemSingle {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}
impl From<f64> for OxideGLItemSingle {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}
impl<T> From<T> for OxideGLItem
where
    T: Into<OxideGLItemSingle>,
{
    fn from(value: T) -> Self {
        Self::Single(value.into())
    }
}
impl<T, const N: usize> From<[T; N]> for OxideGLItem
where
    T: Into<OxideGLItemSingle> + Copy,
{
    fn from(value: [T; N]) -> Self {
        let mut array = [MaybeUninit::<OxideGLItemSingle>::uninit(); N];
        for (idx, val) in value.into_iter().enumerate() {
            array[idx].write(val.into());
        }

        let mut m = ManuallyDrop::new(array);
        let v = *unsafe {
            std::mem::transmute::<
                &mut [MaybeUninit<OxideGLItemSingle>; N],
                &mut [OxideGLItemSingle; N],
            >(&mut m)
        };
        OxideGLItem::Array(Rc::new(v))
    }
}
