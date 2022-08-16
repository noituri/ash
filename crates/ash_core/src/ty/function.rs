use crate::common::{Spanned, Id};

use super::Ty;

pub(crate) type FunArg = (Id, String, Ty);

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Function,
    Method,
}

#[derive(Debug, Clone)]
pub(crate) struct Function<S> {
    pub id: Id,
    pub name: String,
    pub params: Vec<FunArg>,
    pub ty: Ty,
    pub body: Spanned<S>,
}
