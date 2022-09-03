use crate::{
    core::{Annotation, Id, Spanned},
    parser::{
        operator::{BinaryOp, UnaryOp},
        If,
    },
};

use super::{
    function::{Function, ProtoFunction},
    ty::Ty,
    TypeSystem, Value,
};

#[derive(Debug, Clone)]
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
    While(Spanned<Expr>, Vec<Spanned<Stmt>>),
    Return(Option<Expr>, Ty),
    Expression(Expr, Ty),
}

impl Stmt {
    pub(crate) fn ty(&mut self, ts: &mut TypeSystem) -> Ty {
        match self {
            Self::Annotation(_, stmt) => stmt.0.ty(ts),
            Self::ProtoFunction(proto) => proto.ty.clone(),
            Self::Function(fun) => fun.proto.0.ty.clone(),
            Self::VariableDecl { ty, .. } => ty.clone(),
            Self::VariableAssign { value, .. } => value.ty(ts),
            Self::Return(_, ty) => ty.clone(),
            Self::Expression(_, ty) => ty.clone(),
            Self::While(_, _) => Ty::Void,
        }
    }

    pub fn to_expr(self) -> Expr {
        match self {
            Self::Expression(expr, _) => expr,
            _ => panic!("Not an expression: {:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Variable(Id, String, Ty),
    Literal(Value),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        ty: Ty,
    },
    Block(Vec<Spanned<Stmt>>, Ty),
    If(If<Expr, Stmt>, Ty),
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
    pub(crate) fn ty(&mut self, ts: &mut TypeSystem) -> Ty {
        let ty = match self {
            Self::Variable(_, _, ty) => ty.clone(),
            Self::Literal(value) => value.ty(),
            Self::Call { ty, .. } => ty.clone(),
            Self::Block(_, ty) => ty.clone(),
            Self::If(_, ty) => ty.clone(),
            Self::Unary { ty, .. } => ty.clone(),
            Self::Binary { ty, .. } => ty.clone(),
        };

        if let Ty::DeferTyCheck(mut types, span) = ty {
            let first_ty = types.remove(0);
            for ty in types {
                if !ts.check_type(first_ty.clone(), ty, span.clone()) {
                    break;
                }
            }

            self.update_ty(first_ty.clone());
            first_ty
        } else {
            ty
        }
    }

    fn update_ty(&mut self, new_ty: Ty) {
        match self {
            Self::Variable(_, _, ty) => *ty = new_ty,
            Self::Call { ty, .. } => *ty = new_ty,
            Self::Block(_, ty) => *ty = new_ty,
            Self::If(_, ty) => *ty = new_ty,
            Self::Unary { ty, .. } => *ty = new_ty,
            Self::Binary { ty, .. } => *ty = new_ty,
            Self::Literal(_) => {}
        }
    }
}
