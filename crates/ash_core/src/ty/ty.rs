use core::fmt;

use crate::prelude::Span;

// TODO: Define some of these types as a part of std lib
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ty {
    String,
    Bool,
    I32,
    F64,
    Void,
    Fun(Vec<Ty>, Box<Ty>),
    DeferTyCheck(Vec<Ty>, Span),
}

impl Default for Ty {
    fn default() -> Self {
        Self::Void
    }
}

impl From<String> for Ty {
    fn from(s: String) -> Self {
        match s.as_str() {
            "str" => Self::String,
            "bool" => Self::Bool,
            "i32" => Self::I32,
            "f64" => Self::F64,
            "void" => Self::F64,
            _ => todo!("Implement custom types"),
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match self {
            Self::Bool => "bool".to_owned(),
            Self::F64 => "f64".to_owned(),
            Self::I32 => "i32".to_owned(),
            Self::String => "str".to_owned(),
            Self::Void => "void".to_string(),
            Self::Fun(params, ty) => {
                format!("fun({:?}): {}", params, ty)
            }
            Self::DeferTyCheck(_, _) => "Deferred Type Check".to_owned(),
        };

        f.write_str(&ty)
    }
}

impl Ty {
    pub fn fun_return_ty(&self) -> Self {
        match self {
            Self::Fun(_, ty) => *ty.clone(),
            _ => panic!("Used fun_return_ty() not on a function"),
        }
    }
}
