use std::collections::HashMap;

use crate::{
    common::{Context, Spanned},
    parser::stmt::Stmt,
    ty::FunctionType,
};

pub(crate) type Scope = HashMap<String, bool>;

pub(crate) struct Resolver<'a> {
    context: &'a mut Context,
    scopes: Vec<Scope>,
    current_function: Option<FunctionType>,
}

impl<'a> Resolver<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self {
            context,
            scopes: Vec::new(),
            current_function: None,
        }
    }

    pub fn run(&mut self, statements: &'a [Spanned<Stmt>]) {
        self.resolve_root(statements);
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

    fn resolve_stmt(&mut self, stmt: &'a Spanned<Stmt>) {}

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
}
