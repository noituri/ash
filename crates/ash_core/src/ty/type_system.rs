use chumsky::prelude::Simple;

use crate::{
    core::{Context, Spanned},
    parser::{self, conditional::IfInner, operator::BinaryOp, If},
    prelude::{AshResult, Span},
};

use crate::ty;

use super::{function::Function, Ty};

pub(crate) struct TypeSystem<'a> {
    context: &'a mut Context,
    errors: Vec<Simple<String>>,
    parsing_call: bool,
    current_func_returns: Option<Ty>,
    last_expr: bool,
}

impl<'a> TypeSystem<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self {
            context,
            errors: Vec::new(),
            parsing_call: false,
            current_func_returns: None,
            last_expr: false,
        }
    }

    pub fn run(
        mut self,
        statements: Vec<Spanned<parser::Stmt>>,
    ) -> AshResult<Vec<Spanned<ty::Stmt>>, String> {
        let ast = statements
            .into_iter()
            .map(|stmt| self.type_stmt(stmt, false))
            .collect::<Vec<_>>();

        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        Ok(ast)
    }

    fn type_stmt(
        &mut self,
        (stmt, span): Spanned<parser::Stmt>,
        must_return_value: bool,
    ) -> Spanned<ty::Stmt> {
        let stmt = match stmt {
            parser::Stmt::ProtoFunction(proto) => {
                self.context
                    .new_var(proto.id, proto.name.clone(), proto.ty.clone());
                ty::Stmt::ProtoFunction(proto)
            }
            parser::Stmt::Function(fun) => {
                let proto = &fun.proto.0;
                let ty = &fun.proto.0.ty;
                let prev = self.current_func_returns.replace(ty.fun_return_ty());

                let mut body = self.type_stmt(fun.body, true);
                if ty.fun_return_ty() != Ty::Void {
                    let body_ty = match &mut body.0 {
                        ty::Stmt::Expression(ty::Expr::Block(statements, _), Ty::Void) => {
                            match statements.last() {
                                Some((ty::Stmt::Return(_, ty), _)) => ty.clone(),
                                _ => Ty::Void,
                            }
                        }
                        stmt => stmt.ty(self),
                    };
                    self.check_type(ty.fun_return_ty(), body_ty, span.clone());
                }

                self.context
                    .new_var(proto.id, proto.name.clone(), ty.clone());
                for (id, name, ty) in proto.params.iter() {
                    self.context.new_var(*id, name.clone(), ty.clone());
                }

                let fun = Function {
                    body,
                    proto: fun.proto,
                };

                self.current_func_returns = prev;

                ty::Stmt::Function(Box::new(fun))
            }
            parser::Stmt::VariableDecl {
                id,
                name,
                ty,
                value,
            } => {
                let mut value = self.type_expr(value, span.clone(), false);
                let ty = match ty {
                    Some(ty) => {
                        let value_ty = value.ty(self);
                        self.check_type(ty.clone(), value_ty, span.clone());
                        ty
                    }
                    None => value.ty(self),
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
                let mut value = self.type_expr(value, span.clone(), false);
                let value_ty = value.ty(self);
                self.check_type(ty, value_ty, span.clone());

                ty::Stmt::VariableAssign { id, name, value }
            }
            parser::Stmt::Return(expr) => {
                let expr = expr.map(|e| self.type_expr(e, span.clone(), false));
                let expr = expr.map(|mut e| {
                    let ty = e.ty(self);
                    (e, ty)
                });

                let ty = expr.as_ref().map(|e| e.1.clone()).unwrap_or_default();
                let fun_ty = self.current_func_returns.clone().unwrap();
                if fun_ty == Ty::Void && expr.is_some() {
                    self.new_error("Function returns nothing. The return statement should not contain any expressions", span.clone())
                }
                if ty != fun_ty {
                    self.new_error(format!("return statement returns value of invalid type. Expected {fun_ty}, got: {ty}"), span.clone());
                }

                ty::Stmt::Return(expr.map(|e| e.0), ty)
            }
            parser::Stmt::Expression(expr) => {
                let mut expr = self.type_expr(expr, span.clone(), !must_return_value);
                let ty = expr.ty(self);

                ty::Stmt::Expression(expr, ty)
            }
            parser::Stmt::Annotation(a, stmt) => {
                ty::Stmt::Annotation(a, Box::new(self.type_stmt(*stmt, must_return_value)))
            }
        };

        (stmt, span)
    }

    fn type_expr(&mut self, expr: parser::Expr, span: Span, expr_statement: bool) -> ty::Expr {
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
                let (statements, ty) = self.type_block(statements, expr_statement);
                ty::Expr::Block(statements, ty)
            }
            parser::Expr::If(If {
                then,
                else_ifs,
                otherwise,
            }) => {
                let (mut then, then_ty) = self.type_if(*then, expr_statement);
                let (mut else_ifs, mut else_ifs_ty): (Vec<_>, Vec<_>) = else_ifs
                    .into_iter()
                    .map(|ef| self.type_if(ef, expr_statement))
                    .unzip();
                let (otherwise, otherwise_ty) = self.type_block(otherwise, expr_statement);

                let ty = {
                    let mut conditions = vec![&mut then.condition];
                    for else_if in else_ifs.iter_mut() {
                        conditions.push(&mut else_if.condition);
                    }

                    for (cond_ty, span) in conditions.iter_mut() {
                        let cond_ty = cond_ty.ty(self);
                        self.check_type(Ty::Bool, cond_ty, span.clone());
                    }

                    if expr_statement {
                        Ty::Void
                    } else {
                        let mut body_types = vec![then_ty, otherwise_ty];
                        body_types.append(&mut else_ifs_ty);
                        Ty::DeferTyCheck(body_types, span.clone())
                    }
                };

                ty::Expr::If(
                    If {
                        then: Box::new(then),
                        else_ifs,
                        otherwise,
                    },
                    ty,
                )
            }
            parser::Expr::Group(expr) => self.type_expr(*expr, span, expr_statement),
            parser::Expr::Unary { op, right } => {
                // TODO: Find trait implementation for the operator and operand
                let mut right = Box::new(self.type_expr(*right, span.clone(), expr_statement));
                let right_ty = right.ty(self);
                self.check_type(Ty::Bool, right_ty, span);

                ty::Expr::Unary {
                    op,
                    right,
                    ty: Ty::Bool,
                }
            }
            parser::Expr::Binary { left, op, right } => {
                let mut left = Box::new(self.type_expr(*left, span.clone(), expr_statement));
                let mut right = Box::new(self.type_expr(*right, span.clone(), expr_statement));

                let left_ty = left.ty(self);
                let right_ty = right.ty(self);

                // TODO: Better implementation
                let numeric_ops = &[
                    BinaryOp::Div,
                    BinaryOp::Mod,
                    BinaryOp::Mul,
                    BinaryOp::Sum,
                    BinaryOp::Sub,
                ];
                let other_ops = &[BinaryOp::Equal, BinaryOp::NotEqual];
                let num_string_ops = &[BinaryOp::Sum];

                if self.check_type(left_ty.clone(), right_ty, span.clone()) {
                    let expected_ty =
                        if num_string_ops.contains(&op) || num_string_ops.contains(&op) {
                            vec![Ty::String, Ty::I32, Ty::F64]
                        } else if numeric_ops.contains(&op) {
                            vec![Ty::I32, Ty::F64]
                        } else if other_ops.contains(&op) {
                            vec![Ty::I32, Ty::F64, Ty::String, Ty::Bool]
                        } else {
                            unreachable!()
                        };

                    self.is_one_of(&expected_ty, &left_ty, span);
                }

                let ty = if other_ops.contains(&op) {
                    Ty::Bool
                } else {
                    left.ty(self)
                };

                ty::Expr::Binary {
                    left,
                    op,
                    right,
                    ty,
                }
            }
        }
    }

    fn type_if(
        &mut self,
        r#if: IfInner<parser::Expr, parser::Stmt>,
        expr_statement: bool,
    ) -> (IfInner<ty::Expr, ty::Stmt>, Ty) {
        let (cond_expr, span) = r#if.condition;
        let condition = (self.type_expr(cond_expr, span.clone(), false), span);
        let (body, ty) = self.type_block(r#if.body, expr_statement);
        let r#if = IfInner { condition, body };

        (r#if, ty)
    }

    fn type_block(
        &mut self,
        statements: Vec<Spanned<parser::Stmt>>,
        expr_statement: bool,
    ) -> (Vec<Spanned<ty::Stmt>>, Ty) {
        let stmt_len = statements.len();
        let statements = statements
            .into_iter()
            .enumerate()
            .map(|(i, stmt)| self.type_stmt(stmt, i == stmt_len - 1))
            .collect::<Vec<_>>();

        let ty = if !expr_statement {
            statements
                .last()
                .map(|(stmt, _)| {
                    if let ty::Stmt::Expression(_, ty) = stmt {
                        ty.clone()
                    } else {
                        Ty::Void
                    }
                })
                .unwrap_or_default()
        } else {
            Ty::Void
        };
        (statements, ty)
    }

    fn type_call(&mut self, callee: parser::Expr, args: Vec<parser::Expr>, span: Span) -> ty::Expr {
        self.parsing_call = true;
        let mut callee = self.type_expr(callee, span.clone(), false);
        self.parsing_call = false;

        let fun_ty = callee.ty(self);
        if let Ty::Fun(params, return_ty) = fun_ty {
            // TODO: Build proper checking system
            let mut typed_args = Vec::new();
            if params.len() == args.len() {
                // TODO: Args should have their own spans
                for (i, arg) in args.into_iter().enumerate() {
                    let mut arg = self.type_expr(arg, span.clone(), false);
                    let arg_ty = arg.ty(self);
                    self.check_type(params[i].clone(), arg_ty, span.clone());
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

    pub(super) fn check_type(&mut self, expected: Ty, received: Ty, span: Span) -> bool {
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
