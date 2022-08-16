use crate::{
    core::{Annotation, Id, Spanned},
    parser::operator::{BinaryOp, UnaryOp},
};

use super::{
    function::{Function, ProtoFunction},
    ty::Ty,
    Value,
};

#[derive(Debug)]
pub(crate) enum Stmt {
    Annotation(Spanned<Annotation>, Box<Spanned<Stmt>>),
    ProtoFunction(ProtoFunction),
    Function(Box<Function<Stmt>>),
    VariableDecl {
        id: Id,
        name: String,
        ty: Ty,
        value: Expr,
    },
    VariableAssign {
        id: Id,
        name: Spanned<String>,
        value: Expr,
    },
    Return(Expr, Ty),
    Expression(Expr, Ty),
}

impl Stmt {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Self::Annotation(_, stmt) => stmt.0.ty(),
            Self::ProtoFunction(proto) => proto.ty.clone(),
            Self::Function(fun) => fun.proto.0.ty.clone(),
            Self::VariableDecl { ty, .. } => ty.clone(),
            Self::VariableAssign { value, .. } => value.ty(),
            Self::Return(_, ty) => ty.clone(),
            Self::Expression(_, ty) => ty.clone(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Expr {
    Variable(Id, String, Ty),
    Literal(Value),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        ty: Ty,
    },
    Block(Vec<Spanned<Stmt>>, Ty),
    Group(Box<Expr>, Ty),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
        ty: Ty,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        ty: Ty,
    },
}

impl Expr {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Self::Variable(_, _, ty) => ty.clone(),
            Self::Literal(value) => value.ty(),
            Self::Call { ty, .. } => ty.clone(),
            Self::Block(_, ty) => ty.clone(),
            Self::Group(_, ty) => ty.clone(),
            Self::Unary { ty, .. } => ty.clone(),
            Self::Binary { ty, .. } => ty.clone(),
        }
    }
}
