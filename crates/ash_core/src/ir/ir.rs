use crate::{
    core::Spanned,
    ty::{function::Function, Expr, Stmt, Ty},
};

pub(crate) struct IR;

impl IR {
    pub fn new() -> Self {
        Self
    }

    // TODO: Return Bytecode IR
    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Vec<Spanned<Stmt>> {
        self.desugar_statements(ast)
    }

    fn desugar_statements(&mut self, statements: Vec<Spanned<Stmt>>) -> Vec<Spanned<Stmt>> {
        statements
            .into_iter()
            .map(|stmt| self.desugar_stmt(stmt))
            .flatten()
            .collect()
    }

    // TODO: Split the code into smaller & more managable functions
    fn desugar_stmt(&mut self, (stmt, span): Spanned<Stmt>) -> Vec<Spanned<Stmt>> {
        match stmt {
            Stmt::Function(mut fun) => {
                fun = self.desugar_fun(fun);
                vec![(Stmt::Function(fun), span)]
            }
            _ => vec![(stmt, span)],
        }
    }

    fn desugar_fun(&mut self, mut fun: Box<Function<Stmt>>) -> Box<Function<Stmt>> {
        let ty = fun.body.0.ty();
        let span = fun.body.1.clone();
        let body = match fun.body.0 {
            Stmt::Expression(Expr::Block(mut statements, _), ty) => {
                statements = self.desugar_statements(statements);
                self.desugar_fun_return_expr(statements, ty.clone())
            }
            stmt @ Stmt::Expression(_, _) => {
                let ty = stmt.ty();
                let statements = self.desugar_stmt((stmt, fun.body.1));
                self.desugar_fun_return_expr(statements, ty.clone())
            }
            _ => unreachable!("Invalid function body") 
        };

        fun.body = (
            Stmt::Expression(Expr::Block(body, ty.clone()), ty),
            span.clone(),
        );

        fun
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
            (Stmt::Return(last.0.to_expr(), ty), last.1)
        };

        body.push(return_stmt);
        body
    }
}
