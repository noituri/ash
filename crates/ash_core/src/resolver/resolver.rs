use std::collections::HashMap;

use chumsky::prelude::Simple;

use crate::{
    core::{Context, Id, Spanned},
    parser::{expr::Expr, stmt::Stmt, If},
    prelude::{AshResult, Span},
    ty::{function::{MAX_FUNCTION_PARAMS, ProtoFunction}, FunctionType, Ty},
};

#[derive(Default)]
pub(crate) struct Scope {
    vars: HashMap<String, VarData>,
    early_exit: bool
}

#[derive(Debug)]
pub(crate) struct VarData {
    id: Id,
    is_defined: bool,
    is_mutable: bool,
    ty: Option<Ty>,
}

pub(crate) struct Resolver<'a> {
    context: &'a mut Context,
    scopes: Vec<Scope>,
    current_function: Option<FunctionType>,
    is_expr_block: bool,
    errors: Vec<Simple<String>>,
    deps: Option<(Id, String, Vec<Id>)>,
}

impl<'a> Resolver<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self {
            context,
            scopes: vec![Scope::default()],
            current_function: None,
            is_expr_block: false,
            errors: Vec::new(),
            deps: None,
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
        self.scopes.push(Scope::default());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop().unwrap();
    }

    fn resolve_root(&mut self, statements: &'a [Spanned<Stmt>]) {
        for stmt in statements {
            self.resolve_root_stmt(stmt)
        }
    }

    fn resolve_root_stmt(&mut self, (stmt, span): &'a Spanned<Stmt>) {
        match stmt {
            Stmt::Annotation(_, stmt) => {
                self.resolve_root_stmt(stmt);
            }
            Stmt::ProtoFunction(proto) => {
                self.declare(proto.name.clone(), proto.id, false, Some(proto.ty.clone()));
                self.define(proto.name.clone());
            }
            Stmt::Function(fun) => {
                let (proto, _) = &fun.proto;
                self.declare(proto.name.clone(), proto.id, false, Some(proto.ty.clone()));
                self.define(proto.name.clone());
            }
            Stmt::VariableDecl {
                id,
                name,
                ty,
                value: _,
                mutable,
            } => {
                self.declare(name.clone(), *id, *mutable, ty.clone());
                self.define(name.clone());
            }
            _ => self.new_error(
                "This statement can not be used in the root scope",
                span.clone(),
            ),
        };
    }

    fn resolve_statements(&mut self, statements: &'a [Spanned<Stmt>]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, (stmt, span): &'a Spanned<Stmt>) {
        match stmt {
            Stmt::Annotation((a, span), stmt) => {
                if !a.is_builtin() {
                    self.new_error("Unknown annotation", span.clone())
                }
                self.resolve_stmt(stmt);
            }
            Stmt::Expression(expr) => self.resolve_expr(expr, span),
            Stmt::VariableDecl {
                id,
                name,
                value,
                ty,
                mutable,
            } => {
                let prev_deps = self.deps.clone();
                self.deps = Some((*id, name.clone(), Vec::new()));

                self.declare(name.clone(), *id, *mutable, ty.clone());
                self.resolve_expr(value, span);
                self.define(name.clone());

                let (_, _, deps) = self.deps.clone().unwrap();
                self.context
                    .resolve_new_var(*id, name.clone(), value.clone(), deps);
                self.deps = prev_deps;
            }
            Stmt::VariableAssign { id, name, value } => {
                self.resolve_expr(value, span);
                let (name, span) = name;
                self.resolve_local(*id, name, span.clone());
            }
            Stmt::ProtoFunction(proto) => {
                if proto.params.len() > MAX_FUNCTION_PARAMS {
                    self.new_error(
                        "Functions can not have more than {MAX_FUNCTION_PARAMS} arguments",
                        span.clone(),
                    );
                }
                self.declare(proto.name.clone(), proto.id, false, Some(proto.ty.clone()));
                self.define(proto.name.clone());

                self.context.new_var(proto.id, proto.name.clone(), None);
            }
            Stmt::Function(fun) => {
                let (proto, _) = &fun.proto;
                self.declare(proto.name.clone(), proto.id, false, Some(proto.ty.clone()));
                self.define(proto.name.clone());
                let prev = self.current_function;
                self.current_function = Some(FunctionType::Function);
                {
                    self.enter_scope();

                    if proto.params.len() > MAX_FUNCTION_PARAMS {
                        self.new_error(
                            "Functions can not have more than {MAX_FUNCTION_PARAMS} arguments",
                            span.clone(),
                        );
                    }
                    for (id, param, ty) in proto.params.iter() {
                        self.declare(param.clone(), *id, false, Some(ty.clone()));
                        self.define(param.clone());
                        self.context.new_var(*id, param.clone(), Some(ty.clone()));
                    }
                    self.resolve_stmt(&fun.body);

                    self.leave_scope();
                }
                self.context.new_var(proto.id, proto.name.clone(), None);
                self.current_function = prev;
            }
            Stmt::While((cond, span), body) => {
                self.resolve_expr(cond, span);
                self.resolve_statements(body);
            }
            Stmt::Return(expr) => {
                if self.current_function.is_none() {
                    self.new_error("return can not be used outside of function", span.clone())
                }
                if let Some(expr) = expr {
                    self.resolve_expr(expr, span);
                }
            }
            Stmt::Block(stmts) => {
                self.block(stmts, false);
            },
            Stmt::If(If {
                then,
                else_ifs,
                otherwise,
            }) => {
                let (cond, cond_span) = &then.condition;
                self.resolve_expr(cond, cond_span);
                self.block(&then.body, false);

                for else_if in else_ifs {
                    let (cond, cond_span) = &else_if.condition;
                    self.resolve_expr(cond, cond_span);
                    self.block(&else_if.body, false);
                }

                self.block(&otherwise, false);
            }
            Stmt::Break(expr) => {
                if !self.is_expr_block {
                    self.new_error("break can not be used outside of expression block or loop", span.clone())
                }
            
                self.mark_scope_exhaustive();

                match expr {
                    Some(expr) => self.resolve_expr(expr, span),
                    None if self.is_expr_block => self.new_error("break inside a block expression needs to pass a value", span.clone()),
                    None => {}
                }
            }
        }
    }

    fn resolve_expr(&mut self, expr: &'a Expr, span: &'a Span) {
        match expr {
            Expr::Variable(id, name) => {
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
            Expr::Block(stmts) => {
                let exhaustive = self.block(stmts, true);
                if exhaustive {
                    self.mark_scope_exhaustive();
                } else {
                    self.new_error("Block expression not exhaustive", span.clone())
                }
            },
            Expr::If(If {
                then,
                else_ifs,
                otherwise,
            }) => {
                let (cond, cond_span) = &then.condition;
                self.resolve_expr(cond, cond_span);
                let mut if_exhaustive = self.block(&then.body, true);

                for else_if in else_ifs {
                    let (cond, cond_span) = &else_if.condition;
                    self.resolve_expr(cond, cond_span);
                    let else_if_exhaustive = self.block(&else_if.body, true);
                    if if_exhaustive {
                        if_exhaustive = else_if_exhaustive;
                    }
                }

                let else_exhaustive = self.block(&otherwise, true);
                if if_exhaustive && else_exhaustive {
                    self.mark_scope_exhaustive();
                } else {
                    self.new_error("If expression not exhaustive", span.clone())
                }
            }
            _ => {}
        }
    }

    fn resolve_local(&mut self, id: Id, name: &'a str, span: Span) {
        for Scope { vars, .. } in self.scopes.iter_mut().rev() {
            if let Some(data) = vars.get(name) {
                let points_to = data.id;
                if !data.is_defined {
                    continue;
                }
                self.context.resolve(id, data.is_mutable, data.ty.clone(), points_to);
                self.detect_deps(points_to, span.clone());
                return;
            }
        }

        self.new_error("Variable does not exist", span);
    }


    fn block(&mut self, statements: &'a [Spanned<Stmt>], is_expr: bool) -> bool {
        let prev = self.is_expr_block;
        if is_expr {
            self.is_expr_block = true;
        }
        
        self.enter_scope();
        self.resolve_statements(statements);
        let exhaustive = self.has_scope_early_exit();
        self.leave_scope();
        
        if is_expr {
            self.is_expr_block = prev;
        }

        exhaustive
    }

    fn declare(&mut self, name: String, id: Id, is_mutable: bool, ty: Option<Ty>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.vars.insert(
                name,
                VarData {
                    id,
                    ty,
                    is_mutable,
                    is_defined: false,
                },
            );
        }
    }

    fn define(&mut self, name: String) {
        if let Some(Scope { vars, .. }) = self.scopes.last_mut() {
            vars.get_mut(&name).unwrap().is_defined = true;
        }
    }

    fn detect_deps(&mut self, checked_id: Id, span: Span) {
        if let Some(deps) = &mut self.deps {
            let path = self.context.check_circular_dep(deps.0, checked_id);
            if !path.is_empty() {
                let path = path
                    .into_iter()
                    .map(|v| v.name)
                    .collect::<Vec<_>>()
                    .join(" -> ");
                let var_name = deps.1.clone();
                self.new_error(
                    format!("Found initialization loop: {var_name} -> {path} -> {var_name}"),
                    span,
                );
                return;
            }
            deps.2.push(checked_id);
        }
    }

    fn has_scope_early_exit(&self) -> bool {
        self.scopes.last().unwrap().early_exit
    }

    fn mark_scope_exhaustive(&mut self) {
        self.scopes.last_mut().unwrap().early_exit = true;
    }

    fn new_error<S: ToString>(&mut self, err_msg: S, span: Span) {
        self.errors.push(Simple::custom(span, err_msg));
    }
}
