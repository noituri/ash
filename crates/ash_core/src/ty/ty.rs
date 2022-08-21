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
            "String" => Self::String,
            "Bool" => Self::Bool,
            "I32" => Self::I32,
            "F64" => Self::F64,
            "Void" => Self::F64,
            _ => todo!("Implement custom types"),
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match self {
            Self::Bool => "Bool".to_owned(),
            Self::F64 => "F64".to_owned(),
            Self::I32 => "I32".to_owned(),
            Self::String => "String".to_owned(),
            Self::Void => "Void".to_string(),
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
