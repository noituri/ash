use crate::core::{Id, Spanned};

use super::Ty;

pub(crate) const MAX_FUNCTION_PARAMS: usize = 255;

pub(crate) type FunArg = (Id, String, Ty);

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Function,
    Method,
}

#[derive(Debug, Clone)]
pub(crate) struct Function<S> {
    pub proto: Spanned<ProtoFunction>,
    pub body: Spanned<S>,
}

impl<S> Function<S> {
    pub fn params_mut(&mut self) -> &mut [FunArg] {
        &mut self.proto.0.params
    }

    pub fn ret_ty(&self) -> Ty {
        self.proto.0.ty.fun_return_ty()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProtoFunction {
    pub id: Id,
    pub name: String,
    pub params: Vec<FunArg>,
    pub ty: Ty,
}
