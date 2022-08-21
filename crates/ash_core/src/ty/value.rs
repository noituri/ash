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

    pub(crate) fn default_for_ty(ty: Ty) -> Self {
        match ty {
            Ty::String => Self::String(String::new()),
            Ty::Bool => Self::Bool(false),
            Ty::I32 => Self::I32(0),
            Ty::F64 => Self::F64(0.0),
            Ty::Void => unreachable!(),
            Ty::Fun(_, _) => todo!(),
            Ty::DeferTyCheck(types, _) => Self::default_for_ty(types[0].clone()),
        }
    }
}
