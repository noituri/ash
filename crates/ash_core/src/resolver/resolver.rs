use std::collections::HashMap;

use chumsky::prelude::Simple;

use crate::{
    common::{Context, Id, Spanned},
    parser::{expr::Expr, stmt::Stmt},
    prelude::{AshResult, Span},
    ty::{FunctionType, Ty},
};

pub(crate) type Scope = HashMap<String, VarData>;

#[derive(Debug)]
pub(crate) struct VarData {
    id: Option<Id>,
    is_defined: bool,
    ty: Option<Ty>,
}

pub(crate) struct Resolver<'a> {
    context: &'a mut Context,
    scopes: Vec<Scope>,
    current_function: Option<FunctionType>,
    errors: Vec<Simple<String>>,
}

impl<'a> Resolver<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        let global_scope = context
            .get_env()
            .typed_names()
            .into_iter()
            .map(|(n, t)| (n, VarData {
                id: None, // FIXME: builtin IDs
                is_defined: true,
                ty: Some(t),
            }));

        Self {
            context,
            scopes: vec![Scope::from_iter(global_scope)],
            current_function: None,
            errors: Vec::new(),
        }
    }

    pub fn run(mut self, statements: &'a [Spanned<Stmt>]) -> AshResult<(), String> {
        self.resolve_root(statements);
        self.resolve_statements(statements);
        if !self.errors.is_empty() {
            return Err(self.errors);
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
                Stmt::Function(fun) => {
                    self.declare(fun.name.clone(), Some(fun.id), Some(fun.ty.clone()));
                    self.define(fun.name.clone());
                }
                Stmt::VariableDecl { id, name, ty, value } => self.declare(name.clone(), Some(*id), ty.clone()),
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
            Stmt::VariableDecl { id, name, value, ty } => {
                self.declare(name.clone(), Some(*id), ty.clone());
                self.resolve_expr(value, span);
                self.define(name.clone())
            }
            Stmt::VariableAssign { id, name, value } => {
                self.resolve_expr(value, span);
                let (name, span) = name;
                self.resolve_local(*id, name, span.clone());
            }
            Stmt::Function(fun) => {
                self.declare(fun.name.clone(), Some(fun.id), Some(fun.ty.clone()));
                self.define(fun.name.clone());

                let prev = self.current_function;
                self.current_function = Some(FunctionType::Function);

                {
                    self.enter_scope();

                    for (id, param, ty) in fun.params.iter() {
                        self.declare(param.clone(), Some(*id), Some(ty.clone()));
                        self.define(param.clone());
                    }
                    self.resolve_stmt(&fun.body);

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
                let v = self.scopes.last().unwrap().get(name).map(|v| v.is_defined);
                if v == Some(false) {
                    self.new_error(
                        "Use of variable in its own initializer is forbidden",
                        span.clone(),
                    );
                }

                self.resolve_local(*id, name, span.clone())
            }
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
            if let Some(data) = scope.get(name) {
                dbg!(name);
                dbg!(data);
                self.context.resolve(id, depth, data.ty.clone(), data.id.unwrap());
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

    fn declare(&mut self, name: String, id: Option<Id>, ty: Option<Ty>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, VarData {
                id,
                ty,
                is_defined: false
            });
        }
    }

    fn define(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.get_mut(&name).unwrap().is_defined = true;
        }
    }

    fn new_error<S: ToString>(&mut self, err_msg: S, span: Span) {
        self.errors.push(Simple::custom(span, err_msg));
    }
}
