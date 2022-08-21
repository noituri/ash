use crate::{
    core::{Annotation, Id, Spanned},
    parser::{
        operator::{BinaryOp, UnaryOp},
        If,
    },
    ty::{
        function::{Function, ProtoFunction},
        Ty, Value,
    },
};

#[derive(Debug)]
pub(crate) enum Stmt {
    Annotation(Spanned<Annotation>, Box<Spanned<Stmt>>),
    ProtoFunction(ProtoFunction),
    Function(Box<Function<Vec<Spanned<Stmt>>>>),
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
    If(If<Expr, Stmt>),
    Return(Option<Expr>, Ty),
    Expression(Expr, Ty),
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
