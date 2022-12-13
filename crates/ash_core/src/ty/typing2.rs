use chumsky::prelude::Simple;

use crate::{core::{Context, Spanned, Id}, hir::{Stmt, Expr}, prelude::{AshResult, Span}, parser::operator::UnaryOp};

use super::{Value, Ty};

pub(crate) struct Typing<'a> {
    ctx: &'a mut Context,
    errors: Vec<Simple<String>>,
}

impl<'a> Typing<'a> {
    pub fn run(ctx: &'a mut Context, ast: Vec<Spanned<Stmt>>) -> AshResult<Vec<Spanned<super::Stmt>>, String> {
        let mut typing = Self {
            ctx,
            errors: Vec::new()
        };

        if !typing.errors.is_empty() {
            return Err(typing.errors);
        }

        Ok(typing.multiple_stmt(ast))
    }

    fn multiple_stmt(&mut self, stmts: Vec<Spanned<Stmt>>) -> Vec<Spanned<super::Stmt>> {
        stmts
            .into_iter()
            .map(|stmt| self.stmt(stmt))
            .collect()
    }

    fn stmt(&mut self, (stmt, span): Spanned<Stmt>) -> Spanned<super::Stmt> {
        match stmt {
            Stmt::Fun(_) => todo!(),
            Stmt::Proto(_) => todo!(),
            Stmt::DeclVar { id, name, ty, value } => todo!(),
            Stmt::StoreVar { id, name, value } => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::If(_) => todo!(),
            Stmt::Block(_) => todo!(),
            Stmt::Break => todo!(),
            Stmt::Ret(_) => todo!(),
            Stmt::Expr(expr) => self.stmt_expr(expr, span),
        }
    }

    fn expr(&mut self, expr: Expr, span: Span) -> super::Expr {
        match expr {
            Expr::LoadVar(id, name) => self.load_var(id, name),
            Expr::Literal(value) => self.literal(value),
            Expr::Call { callee, args } => todo!(),
            Expr::Unary { op, right } => self.unary(op, *right, span),
            Expr::Binary { left, op, right } => todo!(),
        }
    }

    fn stmt_expr(&mut self, expr: Expr, span: Span) -> Spanned<super::Stmt> {
        let expr = self.expr(expr, span.clone());
        let ty = expr.ty();
        (super::Stmt::Expr(expr, ty), span)
    }

    // TODO: check if calling function before they were defined works
    fn load_var(&mut self, id: Id, name: String) -> super::Expr {
        let ty = self.ctx.var_data(id).and_then(|v| v.ty);
        let ty = match ty {
            Some(ty) => ty,
            None => self.var_pointing_ty(id)
        };

        super::Expr::LoadVar(id, name, ty)
    }

    fn literal(&mut self, value: Value) -> super::Expr {
        super::Expr::Literal(value)
    }

    fn unary(&mut self, op: UnaryOp, right: Expr, span: Span) -> super::Expr {
        let right = Box::new(self.expr(right, span.clone()));
        let ty = right.ty();

        let expected_types = match op {
            UnaryOp::Neg => vec![Ty::F64, Ty::I32],
            UnaryOp::Not => vec![Ty::Bool]
        };

        self.expect_one_of(&expected_types, &ty, span);

        super::Expr::Unary {
            op,
            right,
            ty
        }
    }

    fn var_pointing_ty(&self, id: Id) -> Ty {
        self.ctx.get_pointed_local(id).ty.clone().unwrap()
    }

    fn expect_ty(&mut self, expected_ty: &Ty, received_ty: &Ty, span: Span) {
        if expected_ty != received_ty {
            self.new_error(
                format!("Expected type {expected_ty}, got {received_ty}"),
                span
            );
        }
    }

    fn expect_one_of(&mut self, expected_types: &[Ty], received_ty: &Ty, span: Span) {
        if !expected_types.contains(received_ty) {
            self.new_error(
                format!("Expected types {:?}, got: {}", expected_types, received_ty),
                span,
            );
        }
    }

    fn new_error<S, P>(&mut self, err_msg: S, span: P)
    where
        S: ToString,
        P: ToOwned<Owned = Span>,
    {
        self.errors.push(Simple::custom(span.to_owned(), err_msg))
    }
}