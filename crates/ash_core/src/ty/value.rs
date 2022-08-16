use super::ty::Ty;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
}

impl Value {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Self::String(_) => Ty::String,
            Self::I32(_) => Ty::I32,
            Self::F64(_) => Ty::F64,
            Self::Bool(_) => Ty::Bool,
        }
    }
}
