use crate::{
    core::{next_id, Context, Id, Spanned},
    parser::{conditional::IfInner, If},
    prelude::Span,
    ty::{self, function::Function, Ty, Value},
};

use crate::ir;

use super::bytecode::Compiler;

struct DesugaredAst<T> {
    returns: Option<T>,
    rest: Vec<Spanned<ir::Stmt>>,
}

impl<T> DesugaredAst<T> {
    pub fn new(returns: T, rest: Vec<Spanned<ir::Stmt>>) -> Self {
        Self {
            returns: Some(returns),
            rest,
        }
    }

    pub fn returns(returns: T) -> Self {
        Self {
            returns: Some(returns),
            rest: Vec::new(),
        }
    }

    pub fn rest(rest: Vec<Spanned<ir::Stmt>>) -> Self {
        Self {
            returns: None,
            rest,
        }
    }

    pub fn none() -> Self {
        Self {
            returns: None,
            rest: Vec::new(),
        }
    }
}

impl DesugaredAst<Spanned<ir::Stmt>> {
    pub fn flatten(mut self) -> Vec<Spanned<ir::Stmt>> {
        if let Some(stmt) = self.returns {
            self.rest.push(stmt);
        }

        self.rest
    }
}

pub(crate) struct IR<'a> {
    context: &'a mut Context,
}

impl<'a> IR<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self { context }
    }

    // TODO: Return Bytecode IR
    pub fn run(mut self, ast: Vec<Spanned<ty::Stmt>>) -> Vec<Spanned<ir::Stmt>> {
        let ast = self.desugar_statements(ast);
        let compiler = Compiler::new(self.context);
        compiler.run();
        ast
    }

    fn desugar_statements(&mut self, statements: Vec<Spanned<ty::Stmt>>) -> Vec<Spanned<ir::Stmt>> {
        statements
            .into_iter()
            .map(|stmt| self.desugar_stmt(stmt).flatten())
            .flatten()
            .collect()
    }

    fn desugar_stmt(&mut self, (stmt, span): Spanned<ty::Stmt>) -> DesugaredAst<Spanned<ir::Stmt>> {
        match stmt {
            ty::Stmt::Function(fun) => {
                let fun = self.desugar_fun(fun);
                DesugaredAst::returns((ir::Stmt::Function(fun), span))
            }
            ty::Stmt::VariableDecl {
                id,
                mut name,
                ty,
                value,
            } => {
                self.mangle_var_decl_name(id, &mut name);
                let ast = self.desugar_expr(value);
                let value = ast
                    .returns
                    .unwrap_or(ir::Expr::Literal(Value::default_for_ty(ty.clone())));
                let decl = ir::Stmt::VariableDecl {
                    id,
                    name,
                    ty,
                    value,
                };

                DesugaredAst::new((decl, span), ast.rest)
            }
            ty::Stmt::Expression(expr, ty) => {
                let expr = self.desugar_expr(expr);
                match expr.returns {
                    Some(returns) => {
                        let expr_stmt = ir::Stmt::Expression(returns, ty);
                        DesugaredAst::new((expr_stmt, span), expr.rest)
                    }
                    None => DesugaredAst::rest(expr.rest),
                }
            }
            ty::Stmt::VariableAssign {
                id,
                mut name,
                value,
            } => {
                self.mangle_var_expr_name(id, &mut name.0);
                let ty = self.context.var_type_at(id);
                let expr = self.desugar_expr(value);
                let value = expr
                    .returns
                    .unwrap_or(ir::Expr::Literal(Value::default_for_ty(ty)));
                let assign = ir::Stmt::VariableAssign { id, name, value };

                DesugaredAst::new((assign, span), expr.rest)
            }
            ty::Stmt::Annotation(name, stmt) => {
                let stmt = self.desugar_stmt(*stmt);
                let annotation = ir::Stmt::Annotation(name, Box::new(stmt.returns.unwrap()));

                DesugaredAst::new((annotation, span), stmt.rest)
            }
            ty::Stmt::ProtoFunction(proto) => {
                let proto_stmt = ir::Stmt::ProtoFunction(proto);
                DesugaredAst::returns((proto_stmt, span))
            }
            ty::Stmt::While(cond, body) => {
                let cond_expr = self.desugar_expr(cond.0);
                let body = self.desugar_statements(body);
                let r#while = ir::Stmt::While((cond_expr.returns.unwrap(), cond.1), body);

                DesugaredAst::new((r#while, span), cond_expr.rest)
            }
            ty::Stmt::Return(expr, ty) => match expr {
                Some(expr) => {
                    let DesugaredAst { returns, rest } = self.desugar_expr(expr);
                    let expr =
                        returns.unwrap_or(ir::Expr::Literal(Value::default_for_ty(ty.clone())));
                    let stmt = ir::Stmt::Return(Some(expr), ty);

                    DesugaredAst::new((stmt, span), rest)
                }
                None => DesugaredAst::returns((ir::Stmt::Return(None, ty), span)),
            },
        }
    }

    fn desugar_expr(&mut self, expr: ty::Expr) -> DesugaredAst<ir::Expr> {
        match expr {
            ty::Expr::Variable(id, mut name, ty) => {
                self.mangle_var_expr_name(id, &mut name);
                let var = ir::Expr::Variable(id, name, ty);

                DesugaredAst::returns(var)
            }
            ty::Expr::Block(statements, ty) => self.desugar_block(statements, ty),
            ty::Expr::Unary { op, right, ty } => {
                let expr = self.desugar_expr(*right);
                let right = Box::new(expr.returns.unwrap());
                let unary = ir::Expr::Unary { op, right, ty };

                DesugaredAst::new(unary, expr.rest)
            }
            ty::Expr::Binary {
                left,
                op,
                right,
                ty,
            } => {
                let l_expr = self.desugar_expr(*left);
                let r_expr = self.desugar_expr(*right);
                let left = Box::new(l_expr.returns.unwrap());
                let right = Box::new(r_expr.returns.unwrap());
                // TODO: Needs testing
                let rest = {
                    let mut left = l_expr.rest;
                    let mut right = r_expr.rest;
                    left.append(&mut right);

                    left
                };
                let binary = ir::Expr::Binary {
                    left,
                    op,
                    right,
                    ty,
                };

                DesugaredAst::new(binary, rest)
            }
            ty::Expr::Literal(value) => DesugaredAst::returns(ir::Expr::Literal(value)),
            ty::Expr::Call { callee, args, ty } => {
                let callee_expr = self.desugar_expr(*callee);
                let callee = Box::new(callee_expr.returns.unwrap());

                let mut rest = callee_expr.rest;
                let args = args
                    .into_iter()
                    .map(|a| {
                        let a = self.desugar_expr(a);
                        let mut arg_rest = a.rest;
                        rest.append(&mut arg_rest);

                        a.returns.unwrap()
                    })
                    .collect::<Vec<_>>();

                // TODO: Needs testing
                let call = ir::Expr::Call { callee, args, ty };
                DesugaredAst::new(call, rest)
            }
            ty::Expr::If(r#if, ty) => {
                // TODO: Span
                if ty == Ty::Void {
                    let tmp = self.desugar_if_stmt(r#if);
                    DesugaredAst::rest(tmp.flatten())
                } else {
                    let tmp = self.desugar_if_expr(r#if, ty);
                    DesugaredAst::new(tmp.returns.unwrap(), tmp.rest)
                }
            }
        }
    }

    fn desugar_block(
        &mut self,
        statements: Vec<Spanned<ty::Stmt>>,
        ty: Ty,
    ) -> DesugaredAst<ir::Expr> {
        let mut statements = self.desugar_statements(statements);
        if ty == Ty::Void {
            DesugaredAst::rest(statements)
        } else {
            let (last, _) = statements.remove(statements.len() - 1);
            let expr = if let ir::Stmt::Expression(expr, _) = last {
                expr
            } else {
                unreachable!()
            };
            DesugaredAst::new(expr, statements)
        }
    }

    fn init_new_var(&mut self, statements: &mut Vec<Spanned<ir::Stmt>>, ty: Ty) -> (Id, String) {
        let id = next_id();
        let mut name = "_tmp_".to_owned();
        self.context.new_var(id, name.clone(), ty.clone());
        self.mangle_var_decl_name(id, &mut name);
        let tmp_var = ir::Stmt::VariableDecl {
            id,
            name: name.clone(),
            ty: ty.clone(),
            value: ir::Expr::Literal(Value::default_for_ty(ty.clone())),
        };
        statements.push((tmp_var, Span::default()));

        (id, name)
    }

    fn new_var_read(&mut self, name: String, ty: Ty, points_to: Id) -> ir::Expr {
        let read_id = next_id();
        let read = ir::Expr::Variable(read_id, name, ty.clone());
        self.context.resolve(read_id, 0, Some(ty), points_to);
        read
    }

    fn new_var_assign(&mut self, name: String, value: ir::Expr, ty: Ty, points_to: Id) -> ir::Stmt {
        let id = next_id();
        let name = (name, Span::default());
        let assign = ir::Stmt::VariableAssign { id, name, value };
        self.context.resolve(id, 0, Some(ty), points_to);
        assign
    }

    fn desugar_if_stmt(&mut self, r#if: If<ty::Expr, ty::Stmt>) -> DesugaredAst<Spanned<ir::Stmt>> {
        let mut insert_before = Vec::new();
        let mut desugar_inner =
            |inner: IfInner<ty::Expr, ty::Stmt>| -> IfInner<ir::Expr, ir::Stmt> {
                let mut condition_desugar = self.desugar_expr(inner.condition.0);
                insert_before.append(&mut condition_desugar.rest);
                let condition = (condition_desugar.returns.unwrap(), inner.condition.1);

                IfInner {
                    condition,
                    body: self.desugar_statements(inner.body),
                }
            };

        let r#if = If {
            then: Box::new(desugar_inner(*r#if.then)),
            else_ifs: r#if
                .else_ifs
                .into_iter()
                .map(|ef| desugar_inner(ef))
                .collect(),
            otherwise: self.desugar_statements(r#if.otherwise),
        };
        let if_stmt = ir::Stmt::If(r#if);
        // TODO: Span
        DesugaredAst::new((if_stmt, Span::default()), insert_before)
    }

    fn desugar_if_expr(&mut self, r#if: If<ty::Expr, ty::Stmt>, ty: Ty) -> DesugaredAst<ir::Expr> {
        let mut insert_before = Vec::new();
        let (final_var_id, final_var_name) = self.init_new_var(&mut insert_before, ty.clone());

        fn desugar_inner_if_body(
            ir: &mut IR,
            body: Vec<Spanned<ty::Stmt>>,
            id: Id,
            name: String,
            ty: Ty,
        ) -> Vec<Spanned<ir::Stmt>> {
            let body = ir.desugar_block(body, ty.clone());
            let assign = body
                .returns
                .map(|expr| ir.new_var_assign(name.clone(), expr, ty.clone(), id));
            let mut body = body.rest;
            body.push((assign.unwrap(), Span::default()));
            body
        }

        let mut desugar_inner_if =
            |inner: IfInner<ty::Expr, ty::Stmt>| -> IfInner<ir::Expr, ir::Stmt> {
                let mut cond = self.desugar_expr(inner.condition.0);
                insert_before.append(&mut cond.rest);

                IfInner {
                    condition: (cond.returns.unwrap(), inner.condition.1),
                    body: desugar_inner_if_body(
                        self,
                        inner.body,
                        final_var_id,
                        final_var_name.clone(),
                        ty.clone(),
                    ),
                }
            };

        let r#if = If {
            then: Box::new(desugar_inner_if(*r#if.then)),
            else_ifs: r#if
                .else_ifs
                .into_iter()
                .map(|ef| desugar_inner_if(ef))
                .collect::<Vec<_>>(),
            otherwise: desugar_inner_if_body(
                self,
                r#if.otherwise,
                final_var_id,
                final_var_name.clone(),
                ty.clone(),
            ),
        };

        insert_before.push((ir::Stmt::If(r#if), Span::default()));
        DesugaredAst::new(
            self.new_var_read(final_var_name, ty, final_var_id),
            insert_before,
        )
    }

    fn desugar_fun(
        &mut self,
        fun: Box<Function<ty::Stmt>>,
    ) -> Box<Function<Vec<Spanned<ir::Stmt>>>> {
        let mut ir_fun = Function {
            proto: fun.proto,
            body: (Vec::new(), Span::default()),
        };

        // Mangle param names
        ir_fun
            .proto
            .0
            .params
            .iter_mut()
            .for_each(|(id, name, _)| self.mangle_var_decl_name(*id, name));

        let fun_ty = ir_fun.proto.0.ty.fun_return_ty();
        let span = fun.body.1.clone();
        let body = match fun.body.0 {
            ty::Stmt::Expression(ty::Expr::Block(statements, _), _) => {
                let statements = self.desugar_statements(statements);
                self.desugar_fun_return_expr(statements, fun_ty.clone())
            }
            stmt @ ty::Stmt::Expression(_, _) => {
                let statements = self.desugar_stmt((stmt, fun.body.1)).flatten();
                self.desugar_fun_return_expr(statements, fun_ty.clone())
            }
            _ => unreachable!("Invalid function body"),
        };

        ir_fun.body = (body, span.clone());

        // TODO: Mangle function name
        Box::new(ir_fun)
    }

    fn mangle_var_decl_name(&mut self, id: Id, name: &mut String) {
        *name = format!("__{}__{}", name, id);

        let local = self.context.get_local_mut(id);
        local.name = Some(name.clone())
    }

    fn mangle_var_expr_name(&mut self, id: Id, name: &mut String) {
        let local = self.context.get_pointed_local(id);
        *name = local.name.clone().unwrap();
    }

    fn desugar_fun_return_expr(
        &mut self,
        mut body: Vec<Spanned<ir::Stmt>>,
        ty: Ty,
    ) -> Vec<Spanned<ir::Stmt>> {
        if ty == Ty::Void {
            return body;
        }

        let last = body.remove(body.len() - 1);
        let return_stmt = if matches!(last.0, ir::Stmt::Return(_, _)) {
            last
        } else {
            let expr = if let ir::Stmt::Expression(expr, _) = last.0 {
                expr
            } else {
                unreachable!()
            };
            (ir::Stmt::Return(Some(expr), ty), last.1)
        };

        body.push(return_stmt);
        body
    }
}
