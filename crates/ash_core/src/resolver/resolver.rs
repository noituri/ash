use std::collections::HashMap;

use chumsky::prelude::Simple;

use crate::{
    common::{Context, Spanned, Id},
    parser::{stmt::Stmt, expr::Expr},
    ty::FunctionType, prelude::{AshResult, Span},
};

pub(crate) type Scope = HashMap<String, bool>;

pub(crate) struct Resolver<'a> {
    context: &'a mut Context,
    scopes: Vec<Scope>,
    current_function: Option<FunctionType>,
    errors: Vec<Simple<String>>
}

impl<'a> Resolver<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        let global_scope = context.get_env().names().into_iter().map(|n| (n, true));

        Self {
            context,
            scopes: vec![Scope::from_iter(global_scope)],
            current_function: None,
            errors: Vec::new()
        }
    }

    pub fn run(&mut self, statements: &'a [Spanned<Stmt>]) -> AshResult<(), String> {
        self.resolve_root(statements);
        self.resolve_statements(statements);
        if !self.errors.is_empty() {
            return Err(self.errors.clone())
        }

        Ok(())
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_root(&mut self, statements: &'a [Spanned<Stmt>]) {
        for (stmt, span) in statements {
            match stmt {
                Stmt::Function { name, .. } => {
                    self.declare(name.clone());
                    self.define(name.clone());
                }
                Stmt::VariableDecl { name, .. } => self.declare(name.clone()),
                _ => {}
            }
        }
    }

    fn resolve_statements(&mut self, statements: &'a [Spanned<Stmt>]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, (stmt, span): &'a Spanned<Stmt>) {
        match stmt {
            Stmt::Expression(expr) => self.resolve_expr(expr, span),
            Stmt::VariableDecl { name, value, .. } => {
                self.declare(name.clone());
                self.resolve_expr(value, span);
                self.define(name.clone())
            }
            Stmt::VariableAssign { id, name, value } => {
                self.resolve_expr(value, span);
                self.resolve_local(*id, name, span.clone());
            }
            Stmt::Function { name, params, body, ty } => {
                self.declare(name.clone());
                self.define(name.clone());

                let prev = self.current_function;
                self.current_function = Some(FunctionType::Function);

                {
                    self.enter_scope();

                    for (param, _ty) in params {
                        self.declare(param.clone());
                        self.define(param.clone());
                    }
                    self.resolve_stmt(body);

                    self.leave_scope();
                }
                self.current_function = prev;
            }
            Stmt::Return(expr) => {
                if self.current_function.is_none() {
                    self.new_error("return can not be used outside of function", span.clone())
                }

                self.resolve_expr(expr, span);
            }
            _ => {}
        }
    }

    fn resolve_expr(&mut self, expr: &'a Expr, span: &'a Span) {
        match expr {
            Expr::Variable(id, name) => {
                if self.scopes.last().unwrap().get(name) == Some(&false) {
                   self.new_error("Use of variable in its own initializer is forbidden", span.clone());
                }

                self.resolve_local(*id, name, span.clone())
            },
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left, span);
                self.resolve_expr(right, span);
            }
            Expr::Unary { right, .. } => self.resolve_expr(right, span),
            Expr::Call { callee, args } => {
                self.resolve_expr(callee, span);
                for arg in args {
                    // TODO: Each arg should have its own span
                    self.resolve_expr(arg, span);
                }
            }
            Expr::Group(expr) => self.resolve_expr(expr, span),
            Expr::Block(stmts) => self.block(stmts),
            _ => {}
        }
    }

    fn resolve_local(&mut self, id: Id, name: &'a str, span: Span) {
        for (depth, scope) in self.scopes.iter().enumerate() {
            if scope.contains_key(name) {
                self.context.resolve(id, depth);
                return;
            }
        }

        self.new_error("Variable does not exist", span);
    }

    fn block(&mut self, statements: &'a [Spanned<Stmt>]) {
        self.enter_scope();
        self.resolve_statements(statements);
        self.leave_scope();
    }

    fn declare(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, false);
        }
    }

    fn define(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, true);
        }
    }

    fn new_error<S: ToString>(&mut self, err_msg: S, span: Span) {
        self.errors.push(Simple::custom(span, err_msg));
    }
}
