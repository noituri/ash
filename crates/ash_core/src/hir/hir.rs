use crate::{ty::{function::{Function, ProtoFunction}, Value}, core::{Spanned, Id}, parser::{If, operator::{UnaryOp, BinaryOp}}};

pub(crate) type Body = Vec<Spanned<Stmt>>;

#[derive(Debug)]
pub(crate) enum Stmt {
    Fun(Box<Function<Body>>),
    Proto(ProtoFunction),
    DeclVar {
        id: Id,
        name: String,
        value: Option<Expr>
    },
    StoreVar {
        id: Id,
        name: Spanned<String>,
        value: Expr
    },
    While(Spanned<Expr>, Body),
    If(If<Expr, Stmt>),
    Block(Body),
    Break,
    Ret(Option<Expr>),
    Expr(Expr)
}

#[derive(Debug)]
pub(crate) enum Expr {
    LoadVar(Id, String),
    Literal(Value),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}