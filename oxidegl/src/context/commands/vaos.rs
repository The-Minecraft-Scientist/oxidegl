use crate::context::{
    state::{NamedObject, ObjectName},
    Context,
};

impl Context {}
#[derive(Debug)]
pub struct Vao {}
impl NamedObject for Vao {}

enum VertexAttribState {
    VaoBound(ObjectName<Vao>),

    Unbound,
}

enum ConstVertexAttrib {
    IVec([i32; 4]),
    UVec([u32; 4]),
    Vec([f32; 4]),
}
