use crate::{core::{Context, Spanned, Id, next_id, Annotation}, parser::{Stmt, Expr, If, IfInner, operator::{UnaryOp, BinaryOp}}, ty::{function::{Function, ProtoFunction}, Ty, Value}, prelude::Span};

use super::{scope::Scope, hir::{Body, self}, common::sort_root};

pub(crate) struct Desugarer<'a> {
    ctx: &'a mut Context,
    scopes: Scope<Body>,
    tmp_vars: Vec<(Id, String)>,
    mangle_names: bool,
}

impl<'a> Desugarer<'a> {
    pub fn run(ctx: &'a mut Context, ast: Vec<Spanned<Stmt>>) -> Vec<Spanned<hir::Stmt>> {
        let mut desugarer = Self {
            ctx,
            scopes: Scope::new(),
            tmp_vars: Vec::new(),
            mangle_names: true,
        };

        let ast = sort_root(&desugarer.ctx, ast); 
        desugarer.multiple_stmt(ast);
        desugarer.scopes.leave()
    }

    fn multiple_stmt(&mut self, stmts: Vec<Spanned<Stmt>>) {
        for stmt in stmts {
            self.stmt(stmt);
        }
    }

    fn stmt(&mut self, (stmt, span): Spanned<Stmt>) {
        match stmt {
            Stmt::Annotation(data, stmt) => self.annotation(data, *stmt, span),
            Stmt::Function(fun) => self.fun(fun, span),
            Stmt::ProtoFunction(proto) => self.proto(proto, span),
            Stmt::Expression(expr) => self.stmt_expr(expr, span),
            Stmt::VariableDecl { id, ty, value, .. } => self.var_decl(id, ty, value, span),
            Stmt::VariableAssign { id, name, value } => self.var_assign(id, value, name.1, span),
            Stmt::Block(stmts) => self.stmt_block(stmts, span),
            Stmt::If(data) => self.stmt_if(data, span),
            Stmt::While(cond, body) => self.stmt_while(cond, body, span),
            Stmt::Break(expr) => self.br(expr, span),
            Stmt::Return(expr) => self.ret(expr, span),
        }
    }

    fn stmt_expr(&mut self, expr: Expr, span: Span) {
        let expr = self.expr(expr);
        self.scope_add((hir::Stmt::Expr(expr), span));
    }

    fn expr(&mut self, expr: Expr) -> hir::Expr {
        match expr {
            Expr::Variable(id, _) => self.var(id),
            Expr::Block(stmts) => self.expr_block(stmts),
            Expr::If(data) => self.expr_if(data),
            Expr::Literal(v) => self.literal(v),
            Expr::Call { callee, args } => self.call(*callee, args),
            Expr::Group(expr) => self.expr(*expr),
            Expr::Unary { op, right } => self.unary(op, *right),
            Expr::Binary { left, op, right } => self.binary(*left, op, *right),
        }
    }

    fn var(&self, id: Id) -> hir::Expr {
        let name = self.mangled_pointed_name(id);
        hir::Expr::LoadVar(id, name)
    }

    fn expr_block(&mut self, stmts: Vec<Spanned<Stmt>>) -> hir::Expr {
        self.new_tmp_var();
        
        self.scopes.enter();
        {
            self.multiple_stmt(stmts);
        }
        let stmts = self.scopes.leave();
        self.scope_add((hir::Stmt::Block(stmts), Span::default()));
        
        self.tmp_var_load_rem()
    }

    fn literal(&mut self, value: Value) -> hir::Expr {
        hir::Expr::Literal(value)
    }

    fn call(&mut self, callee: Expr, args: Vec<Expr>) -> hir::Expr {
        let callee = Box::new(self.expr(callee));
        let args = args
            .into_iter()
            .map(|a| self.expr(a))
            .collect::<Vec<_>>();
        
        hir::Expr::Call {
            callee,
            args
        }
    }

    fn unary(&mut self, op: UnaryOp, right: Expr) -> hir::Expr {
        let right = Box::new(self.expr(right));
        hir::Expr::Unary {
            op,
            right
        }
    }

    fn binary(&mut self, left: Expr, op: BinaryOp, right: Expr) -> hir::Expr {
        let left = Box::new(self.expr(left));
        let right = Box::new(self.expr(right));

        hir::Expr::Binary {
            left,
            op,
            right
        }
    }    

    fn scoped_stmts(&mut self, stmts: Vec<Spanned<Stmt>>) -> Body {
        self.scopes.enter();
        {
            self.multiple_stmt(stmts);
        }
        self.scopes.leave()
    }

    fn fun(&mut self, mut fun: Box<Function<Stmt>>, span: Span) {
        fun
            .params_mut()
            .iter_mut()
            .for_each(|(id, name, _)| *name = self.mangled_name(*id));
       
        let ret_ty = fun.ret_ty();
        let mut proto = fun.proto.0;
        let proto_span = fun.proto.1;
        let prev = self.mangle_names;
        self.mangle_names = proto.name != "main" && self.mangle_names;
        proto.name = self.mangled_name(proto.id);
        self.mangle_names = prev;

        let body = fun.body.0;
        let body_span = fun.body.1;
        let body = self.fun_body(ret_ty, body, body_span.clone());

        let fun = Function {
            proto: (proto, proto_span),
            body: (body, body_span)
        };

        self.scope_add((hir::Stmt::Fun(Box::new(fun)), span));
    }

    fn fun_body(&mut self, ret_ty: Ty, stmt: Stmt, span: Span) -> Body {
        self.scopes.enter();
        {
            match stmt {
                Stmt::Block(stmts) => self.multiple_stmt(stmts),
                Stmt::Expression(expr) => self.ret(Some(expr), span),
                _ => unreachable!("Invalid function body")
            }
        }

        // Implicit return;
        if ret_ty == Ty::Void {
            let add_ret = match self.scopes.current().last() {
                Some((last, _)) => !last.is_ret(),
                None => true
            };

            if add_ret {
                self.scope_add((hir::Stmt::Ret(None), Span::default()));
            }
        }
        self.scopes.leave()
    }

    fn br(&mut self, expr: Option<Expr>, span: Span) {
        // Block expression
        if let Some(expr) = expr {
            let value = self.expr(expr);
            self.tmp_var_store(value, span.clone());
        }
        self.scope_add((hir::Stmt::Break, span))
    }

    fn ret(&mut self, expr: Option<Expr>, span: Span) {
        let expr = expr.map(|v| self.expr(v));
        let ret = hir::Stmt::Ret(expr);
        self.scope_add((ret, span))
    }
    
    fn var_decl(&mut self, id: Id, ty: Option<Ty>, value: Expr, span: Span) {
        let name = self.mangled_name(id);
        let value = Some(self.expr(value));
        let decl = hir::Stmt::DeclVar { id, name, ty, value };
        self.scope_add((decl, span))
    }

    fn var_assign(&mut self, id: Id, value: Expr, name_span: Span, span: Span) {
        let name = (self.mangled_pointed_name(id), name_span);
        let value = self.expr(value);
        let assign = hir::Stmt::StoreVar { id, name, value };
        self.scope_add((assign, span))
    }

    fn annotation(&mut self, data: Spanned<Annotation>, stmt: Spanned<Stmt>, _span: Span) {
        if data.0.is_builtin() {
            let prev = self.mangle_names;
            self.mangle_names = false;
            self.stmt(stmt);
            self.mangle_names = prev;
        } else {
            unimplemented!()
        }
    }

    fn proto(&mut self, mut proto: ProtoFunction, span: Span) {
        proto.name = self.mangled_name(proto.id);
        self.scope_add((hir::Stmt::Proto(proto), span));
    }

    fn stmt_while(&mut self, (cond, cond_span): Spanned<Expr>, body: Vec<Spanned<Stmt>>, span: Span) {
        let cond = self.expr(cond);
        let body = self.scoped_stmts(body);
        let r#while = hir::Stmt::While((cond, cond_span), body);
        self.scope_add((r#while, span));
    }

    fn stmt_if(&mut self, data: If<Expr, Stmt>, span: Span) {
        let then = Box::new(self.convert_inner_if(*data.then));
        let else_ifs = data.else_ifs
            .into_iter()
            .map(|inner| self.convert_inner_if(inner))
            .collect::<Vec<_>>();
        let otherwise = self.scoped_stmts(data.otherwise);

        let r#if = hir::Stmt::If(If {
            then,
            else_ifs,
            otherwise
        });
        self.scope_add((r#if, span));
    }


    fn expr_if(&mut self, data: If<Expr, Stmt>) -> hir::Expr {
        self.new_tmp_var();
        self.stmt_if(data, Span::default());
        self.tmp_var_load_rem()
    }

    fn convert_inner_if(&mut self, inner: IfInner<Expr, Stmt>) -> IfInner<hir::Expr, hir::Stmt> {
        let cond = self.expr(inner.condition.0);
        let cond_span = inner.condition.1;
        let body = self.scoped_stmts(inner.body);
        
        IfInner { condition: (cond, cond_span), body }
    }

    fn stmt_block(&mut self, stmts: Vec<Spanned<Stmt>>, span: Span) {
        self.scopes.enter();
        {
            self.multiple_stmt(stmts);
        }
        let stmts = self.scopes.leave();
        self.scope_add((hir::Stmt::Block(stmts), span));
    }
    
    fn mangled_name(&mut self, id: Id) -> String {
        let local = self.ctx.get_local_mut(id);
        if !self.mangle_names {
            local.mangle_name = local.name.clone();
        }
        local.mangle_name.clone().unwrap()
    }

    fn mangled_pointed_name(&self, id: Id) -> String {
        let local = self.ctx.get_pointed_local(id);
        local.mangle_name.clone().unwrap()
    }

    fn new_tmp_var(&mut self) {
        let id = next_id();
        self.ctx.new_var(id, "tmp_".to_string(), None);
        let name = self.mangled_name(id);
        let decl = hir::Stmt::DeclVar { 
            id,
            name: name.clone(),
            ty: None,
            value: None,
        };

        self.scope_add((decl, Span::default()));
        self.tmp_vars.push((id, name));
    }

    fn tmp_var_store(&mut self, value: hir::Expr, span: Span) {
        let (id, name) = self.cur_tmp_var().clone();
        let name = (name, span);
        self.scope_add((hir::Stmt::StoreVar { id, name, value }, Span::default()));
    }

    fn tmp_var_load_rem(&mut self) -> hir::Expr {
        let (id, name) = self.rem_tmp_var();
        hir::Expr::LoadVar(id, name)
    }

    fn cur_tmp_var(&self) -> &(Id, String) {
        self.tmp_vars.last().unwrap()
    }

    fn rem_tmp_var(&mut self) -> (Id, String) {
        self.tmp_vars.remove(self.tmp_vars.len()-1)
    }

    fn scope_add(&mut self, stmt: Spanned<hir::Stmt>) {
        self.scopes.current_mut().push(stmt)
    }
}

