use crate::{
    core::{Context, Id, Spanned},
    ty::{function::Function, Expr, Stmt, Ty, Value},
};

pub(crate) struct IR<'a> {
    context: &'a mut Context,
}

enum Rest {
    InsertBefore(Vec<Spanned<Stmt>>),
    InsertAfter(Vec<Spanned<Stmt>>),
    None,
}

impl Rest {
    pub fn ignore_direction(self) -> Vec<Spanned<Stmt>> {
        match self {
            Self::InsertAfter(v) => v,
            Self::InsertBefore(v) => v,
            Self::None => Vec::new(),
        }
    }
}

struct DesugaredAst<T> {
    returns: Option<T>,
    rest: Rest,
}

impl<T> DesugaredAst<T> {
    pub fn new(returns: T, rest: Rest) -> Self {
        Self {
            returns: Some(returns),
            rest,
        }
    }

    pub fn returns(returns: T) -> Self {
        Self {
            returns: Some(returns),
            rest: Rest::None,
        }
    }

    pub fn rest(rest: Rest) -> Self {
        Self {
            returns: None,
            rest,
        }
    }

    pub fn none() -> Self {
        Self {
            returns: None,
            rest: Rest::None,
        }
    }
}

impl DesugaredAst<Spanned<Stmt>> {
    pub fn flatten(self) -> Vec<Spanned<Stmt>> {
        let mut out = match self.returns {
            Some(stmt) => vec![stmt],
            None => Vec::new(),
        };

        match self.rest {
            Rest::InsertAfter(mut rest) => out.append(&mut rest),
            Rest::InsertBefore(mut rest) => {
                rest.append(&mut out);
                out = rest;
            }
            Rest::None => {}
        }

        out
    }
}

impl<'a> IR<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Self { context }
    }

    // TODO: Return Bytecode IR
    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Vec<Spanned<Stmt>> {
        self.desugar_statements(ast)
    }

    fn desugar_statements(&mut self, statements: Vec<Spanned<Stmt>>) -> Vec<Spanned<Stmt>> {
        statements
            .into_iter()
            .map(|stmt| self.desugar_stmt(stmt).flatten())
            .flatten()
            .collect()
    }

    fn desugar_stmt(&mut self, (stmt, span): Spanned<Stmt>) -> DesugaredAst<Spanned<Stmt>> {
        match stmt {
            Stmt::Function(mut fun) => {
                fun = self.desugar_fun(fun);
                DesugaredAst::returns((Stmt::Function(fun), span))
            }
            Stmt::VariableDecl {
                id,
                mut name,
                ty,
                value,
            } => {
                self.mangle_var_decl_name(id, &mut name);
                let ast = self.desugar_expr(value);
                let value = ast
                    .returns
                    .unwrap_or(Expr::Literal(Value::default_for_ty(ty.clone())));
                let decl = Stmt::VariableDecl {
                    id,
                    name,
                    ty,
                    value,
                };

                DesugaredAst::new((decl, span), ast.rest)
            }
            Stmt::Expression(expr, ty) => {
                let expr = self.desugar_expr(expr);
                let expr_stmt = Stmt::Expression(expr.returns.unwrap(), ty);

                DesugaredAst::new((expr_stmt, span), expr.rest)
            }
            Stmt::VariableAssign {
                id,
                mut name,
                value,
            } => {
                self.mangle_var_expr_name(id, &mut name.0);
                let ty = self.context.var_type_at(id);
                let expr = self.desugar_expr(value);
                let value = expr
                    .returns
                    .unwrap_or(Expr::Literal(Value::default_for_ty(ty)));
                let assign = Stmt::VariableAssign { id, name, value };

                DesugaredAst::new((assign, span), expr.rest)
            }
            Stmt::Annotation(name, stmt) => {
                let stmt = self.desugar_stmt(*stmt);
                let annotation = Stmt::Annotation(name, Box::new(stmt.returns.unwrap()));

                DesugaredAst::new((annotation, span), stmt.rest)
            }
            Stmt::ProtoFunction(_) => DesugaredAst::returns((stmt, span)),
            Stmt::Return(expr, ty) => match expr {
                Some(expr) => {
                    let DesugaredAst { returns, rest } = self.desugar_expr(expr);
                    let expr = returns.unwrap_or(Expr::Literal(Value::default_for_ty(ty.clone())));
                    let stmt = Stmt::Return(Some(expr), ty);

                    DesugaredAst::new((stmt, span), rest)
                }
                None => DesugaredAst::returns((Stmt::Return(None, ty), span)),
            },
        }
    }

    fn desugar_expr(&mut self, expr: Expr) -> DesugaredAst<Expr> {
        match expr {
            Expr::Variable(id, mut name, ty) => {
                self.mangle_var_expr_name(id, &mut name);
                let var = Expr::Variable(id, name, ty);

                DesugaredAst::returns(var)
            }
            Expr::Block(statements, ty) => {
                let mut statements = self.desugar_statements(statements);
                if ty == Ty::Void {
                    DesugaredAst::rest(Rest::InsertBefore(statements))
                } else {
                    let (last, _) = statements.remove(statements.len() - 1);
                    DesugaredAst::new(last.to_expr(), Rest::InsertBefore(statements))
                }
            }
            Expr::Unary { op, right, ty } => {
                let expr = self.desugar_expr(*right);
                let right = Box::new(expr.returns.unwrap());
                let unary = Expr::Unary { op, right, ty };

                DesugaredAst::new(unary, expr.rest)
            }
            Expr::Binary {
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
                    let mut left = l_expr.rest.ignore_direction();
                    let mut right = r_expr.rest.ignore_direction();
                    left.append(&mut right);

                    Rest::InsertBefore(left)
                };
                let binary = Expr::Binary {
                    left,
                    op,
                    right,
                    ty,
                };

                DesugaredAst::new(binary, rest)
            }
            Expr::Literal(_) => DesugaredAst::returns(expr),
            Expr::Call { callee, args, ty } => {
                let callee_expr = self.desugar_expr(*callee);
                let callee = Box::new(callee_expr.returns.unwrap());

                let mut rest = callee_expr.rest.ignore_direction();
                let args = args
                    .into_iter()
                    .map(|a| {
                        let a = self.desugar_expr(a);
                        let mut arg_rest = a.rest.ignore_direction();
                        rest.append(&mut arg_rest);

                        a.returns.unwrap()
                    })
                    .collect::<Vec<_>>();

                // TODO: Needs testing
                let rest = Rest::InsertBefore(rest);
                let call = Expr::Call { callee, args, ty };

                DesugaredAst::new(call, rest)
            }
            _ => DesugaredAst::returns(expr),
        }
    }

    fn desugar_fun(&mut self, mut fun: Box<Function<Stmt>>) -> Box<Function<Stmt>> {
        // Mangle param names
        fun.proto
            .0
            .params
            .iter_mut()
            .for_each(|(id, name, _)| self.mangle_var_decl_name(*id, name));

        let fun_ty = fun.proto.0.ty.fun_return_ty();
        let span = fun.body.1.clone();
        let body = match fun.body.0 {
            Stmt::Expression(Expr::Block(mut statements, _), _) => {
                statements = self.desugar_statements(statements);
                self.desugar_fun_return_expr(statements, fun_ty.clone())
            }
            stmt @ Stmt::Expression(_, _) => {
                let statements = self.desugar_stmt((stmt, fun.body.1)).flatten();
                self.desugar_fun_return_expr(statements, fun_ty.clone())
            }
            _ => unreachable!("Invalid function body"),
        };

        fun.body = (
            Stmt::Expression(Expr::Block(body, fun_ty.clone()), fun_ty),
            span.clone(),
        );

        // TODO: Mangle function name
        fun
    }

    fn mangle_var_decl_name(&mut self, id: Id, name: &mut String) {
        *name = format!("__{}__{}", name, id);

        dbg!(&name);
        let local = self.context.get_local_mut(id);
        local.name = Some(name.clone())
    }

    fn mangle_var_expr_name(&mut self, id: Id, name: &mut String) {
        let local = self.context.get_pointed_local(id);
        *name = local.name.clone().unwrap();
        dbg!(name);
    }

    fn desugar_fun_return_expr(
        &mut self,
        mut body: Vec<Spanned<Stmt>>,
        ty: Ty,
    ) -> Vec<Spanned<Stmt>> {
        if ty == Ty::Void {
            return body;
        }

        let last = body.remove(body.len() - 1);
        let return_stmt = if matches!(last.0, Stmt::Return(_, _)) {
            last
        } else {
            (Stmt::Return(Some(last.0.to_expr()), ty), last.1)
        };

        body.push(return_stmt);
        body
    }
}
