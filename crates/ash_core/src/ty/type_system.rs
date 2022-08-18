use chumsky::prelude::Simple;

use crate::{
    core::{Context, Spanned},
    parser::{self, operator::BinaryOp},
    prelude::{AshResult, Span},
};

use crate::ty;

use super::{function::Function, Ty};

pub(crate) struct TypeSystem<'a> {
    context: &'a mut Context,
    errors: Vec<Simple<String>>,
    parsing_call: bool,
}

impl<'a> TypeSystem<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self {
            context,
            errors: Vec::new(),
            parsing_call: false,
        }
    }

    pub fn run(
        mut self,
        statements: Vec<Spanned<parser::Stmt>>,
    ) -> AshResult<Vec<Spanned<ty::Stmt>>, String> {
        let ast = statements
            .into_iter()
            .map(|stmt| self.type_stmt(stmt))
            .collect::<Vec<_>>();

        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        Ok(ast)
    }

    fn type_stmt(&mut self, (stmt, span): Spanned<parser::Stmt>) -> Spanned<ty::Stmt> {
        let stmt = match stmt {
            parser::Stmt::ProtoFunction(proto) => ty::Stmt::ProtoFunction(proto),
            parser::Stmt::Function(fun) => {
                let body = self.type_stmt(fun.body);
                let ty = &fun.proto.0.ty;
                if ty.fun_return_ty() != Ty::Void {
                    let body_ty = match &body.0 {
                        ty::Stmt::Expression(ty::Expr::Block(statements, _), Ty::Void) => {
                            match statements.last() {
                                Some((ty::Stmt::Return(_, ty), _)) => ty.clone(),
                                _ => Ty::Void,
                            }
                        }
                        stmt => stmt.ty(),
                    };
                    self.check_type(ty.fun_return_ty(), body_ty, span.clone());
                }
                let fun = Function {
                    body,
                    proto: fun.proto,
                };

                ty::Stmt::Function(Box::new(fun))
            }
            parser::Stmt::VariableDecl {
                id,
                name,
                ty,
                value,
            } => {
                let value = self.type_expr(value, span.clone());
                let ty = match ty {
                    Some(ty) => {
                        self.check_type(ty.clone(), value.ty(), span.clone());
                        ty
                    }
                    None => value.ty(),
                };

                if ty == Ty::Void {
                    self.new_error("Setting variables to Void is forbidden", span.clone())
                }

                // TODO: types might need to be pre evaluated first
                self.context.new_var(id, name.clone(), ty.clone());
                ty::Stmt::VariableDecl {
                    id,
                    name,
                    ty,
                    value,
                }
            }
            parser::Stmt::VariableAssign { id, name, value } => {
                let ty = self.context.var_type_at(id);
                let value = self.type_expr(value, span.clone());
                self.check_type(ty, value.ty(), span.clone());

                ty::Stmt::VariableAssign { id, name, value }
            }
            parser::Stmt::Return(expr) => {
                let expr = self.type_expr(expr, span.clone());
                let ty = expr.ty();

                ty::Stmt::Return(expr, ty)
            }
            parser::Stmt::Expression(expr) => {
                let expr = self.type_expr(expr, span.clone());
                let ty = expr.ty();

                ty::Stmt::Expression(expr, ty)
            }
            parser::Stmt::Annotation(a, stmt) => {
                ty::Stmt::Annotation(a, Box::new(self.type_stmt(*stmt)))
            }
        };

        (stmt, span)
    }

    fn type_expr(&mut self, expr: parser::Expr, span: Span) -> ty::Expr {
        match expr {
            parser::Expr::Variable(id, name) => {
                let ty = self.context.var_type_at(id);
                if !self.parsing_call && matches!(ty, Ty::Fun(_, _)) {
                    // Promotes var to call
                    self.type_call(parser::Expr::Variable(id, name), Vec::new(), span)
                } else {
                    ty::Expr::Variable(id, name, ty)
                }
            }
            parser::Expr::Literal(value) => ty::Expr::Literal(value),
            parser::Expr::Call { callee, args } => self.type_call(*callee, args, span),
            parser::Expr::Block(statements) => {
                let statements = statements
                    .into_iter()
                    .map(|stmt| self.type_stmt(stmt))
                    .collect::<Vec<_>>();

                let ty = statements
                    .last()
                    .map(|(stmt, _)| {
                        if let ty::Stmt::Expression(_, ty) = stmt {
                            ty.clone()
                        } else {
                            Ty::Void
                        }
                    })
                    .unwrap_or_default();

                ty::Expr::Block(statements, ty)
            }
            parser::Expr::Group(expr) => self.type_expr(*expr, span),
            parser::Expr::Unary { op, right } => {
                // TODO: Find trait implementation for the operator and operand
                let right = Box::new(self.type_expr(*right, span.clone()));
                self.check_type(Ty::Bool, right.ty(), span);
                ty::Expr::Unary {
                    op,
                    right,
                    ty: Ty::Bool,
                }
            }
            parser::Expr::Binary { left, op, right } => {
                let left = Box::new(self.type_expr(*left, span.clone()));
                let right = Box::new(self.type_expr(*right, span.clone()));
                if self.check_type(left.ty(), right.ty(), span.clone()) {
                    // TODO: Better implementation
                    let numeric_ops = &[BinaryOp::Div, BinaryOp::Mod, BinaryOp::Mul, BinaryOp::Sum];
                    let _other_ops = &[BinaryOp::Equal, BinaryOp::NotEqual];
                    let num_string_ops = &[BinaryOp::Sum];

                    let expected_ty =
                        if num_string_ops.contains(&op) || num_string_ops.contains(&op) {
                            vec![Ty::String, Ty::I32, Ty::F64]
                        } else if numeric_ops.contains(&op) {
                            vec![Ty::I32, Ty::F64]
                        } else {
                            Vec::new()
                        };

                    self.is_one_of(&expected_ty, &left.ty(), span);
                }

                let ty = left.ty();
                ty::Expr::Binary {
                    left,
                    op,
                    right,
                    ty,
                }
            }
        }
    }

    fn type_call(&mut self, callee: parser::Expr, args: Vec<parser::Expr>, span: Span) -> ty::Expr {
        self.parsing_call = true;
        let callee = self.type_expr(callee, span.clone());
        self.parsing_call = false;

        let fun_ty = callee.ty();
        if let Ty::Fun(params, return_ty) = fun_ty {
            // TODO: Build proper checking system
            let mut typed_args = Vec::new();
            if params.len() == args.len() {
                // TODO: Args should have their own spans
                for (i, arg) in args.into_iter().enumerate() {
                    let arg = self.type_expr(arg, span.clone());
                    self.check_type(params[i].clone(), arg.ty(), span.clone());
                    typed_args.push(arg);
                }
            } else {
                self.new_error(
                    format!(
                        "Invalid number of args. Expected {}, got {}",
                        params.len(),
                        args.len()
                    ),
                    span,
                );
            }

            ty::Expr::Call {
                callee: Box::new(callee),
                args: typed_args,
                ty: *return_ty,
            }
        } else {
            panic!("Call expression should only be able to call functions");
        }
    }

    fn is_one_of(&mut self, expected: &[Ty], received: &Ty, span: Span) {
        if !expected.contains(received) {
            self.new_error(
                format!("Expected types {:?}, got: {}", expected, received),
                span,
            );
        }
    }

    fn check_type(&mut self, expected: Ty, received: Ty, span: Span) -> bool {
        if expected != received {
            self.new_error(
                format!("Expected type {}, got: {}", expected, received),
                span,
            );

            return false;
        }

        true
    }

    fn new_error<S, P>(&mut self, err_msg: S, span: P)
    where
        S: ToString,
        P: ToOwned<Owned = Span>,
    {
        self.errors.push(Simple::custom(span.to_owned(), err_msg))
    }
}
