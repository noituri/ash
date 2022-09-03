use std::collections::HashMap;

use ash_bytecode::prelude::*;

use crate::core::{Context, Spanned};
use crate::parser::operator::{BinaryOp, UnaryOp};
use crate::ty;

use super::{Expr, Stmt};

pub(super) struct Compiler<'a> {
    context: &'a Context,
    chunk: Chunk,
    locals: Vec<(String, usize)>,
    scope_depth: usize,
    str_constants: HashMap<String, usize>,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            chunk: Chunk::default(),
            locals: Vec::new(),
            scope_depth: 0,
            str_constants: HashMap::new(),
        }
    }

    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Chunk {
        for (stmt, _) in ast {
            self.compile_stmt(stmt);
        }

        self.add_instr(OpCode::Ret);
        self.chunk
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr, _) => {
                self.compile_expr(expr);
                self.add_instr(OpCode::Pop);
            }
            Stmt::VariableDecl { name, value, .. } => {
                let name_index = self.compile_identifier(name);
                self.compile_expr(value);
                self.def_var(name_index);
            }
            Stmt::VariableAssign { name, value, .. } => {
                let (store, store_long, arg) = match self.resolve_local(&name.0) {
                    Some(arg) => (OpCode::StoreLocalLong, OpCode::StoreLocalLong, arg),
                    None => (
                        OpCode::StoreGlobal,
                        OpCode::StoreGlobalLong,
                        self.compile_identifier(name.0),
                    ),
                };

                self.compile_expr(value);

                self.chunk.add_instr_with_arg(store, store_long, arg);
            }
            Stmt::Block(statements) => self.compile_block(statements),
            _ => unimplemented!(),
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Variable(_, name, _) => self.compile_var_load(name),
            Expr::Literal(value) => self.compile_literal(value),
            Expr::Unary { op, right, .. } => {
                self.compile_expr(*right);
                match op {
                    UnaryOp::Neg => self.add_instr(OpCode::Neg),
                    UnaryOp::Not => self.add_instr(OpCode::Not),
                }
            }
            Expr::Binary {
                left, op, right, ..
            } => {
                self.compile_expr(*left);
                self.compile_expr(*right);

                let instr = match op {
                    BinaryOp::Sum => OpCode::Sum,
                    BinaryOp::Sub => OpCode::Sub,
                    BinaryOp::Mul => OpCode::Mul,
                    BinaryOp::Div => OpCode::Div,
                    BinaryOp::Rem => OpCode::Rem,
                    BinaryOp::Equal => OpCode::Eq,
                    BinaryOp::NotEqual => OpCode::Neq,
                    BinaryOp::Gt => OpCode::Gt,
                    BinaryOp::Lt => OpCode::Lt,
                    BinaryOp::Gte => OpCode::Gte,
                    BinaryOp::Lte => OpCode::Lte,
                };

                self.add_instr(instr);
            }
            _ => unimplemented!(),
        }
    }

    fn compile_block(&mut self, statements: Vec<Spanned<Stmt>>) {
        self.begin_scope();
        for (stmt, _) in statements {
            self.compile_stmt(stmt);
        }
        self.end_scope();
    }

    fn compile_literal(&mut self, value: ty::Value) {
        if let ty::Value::Bool(v) = value {
            if v {
                self.add_instr(OpCode::True);
            } else {
                self.add_instr(OpCode::False);
            }
        } else {
            let value = match value {
                ty::Value::I32(v) => Value::I32(v),
                ty::Value::F64(v) => Value::F64(v),
                _ => unimplemented!(),
            };

            self.write_const(value);
        }
    }

    fn compile_var_load(&mut self, name: String) {
        let (load, load_long, arg) = match self.resolve_local(&name) {
            Some(arg) => (OpCode::LoadLocalLong, OpCode::LoadLocalLong, arg),
            None => (
                OpCode::LoadGlobal,
                OpCode::LoadGlobalLong,
                self.compile_identifier(name),
            ),
        };
        self.chunk.add_instr_with_arg(load, load_long, arg);
    }

    fn compile_identifier(&mut self, name: String) -> usize {
        self.decl_var(name.clone());
        if self.scope_depth > 0 {
            return 0;
        }
        match self.str_constants.get(&name) {
            Some(index) => *index,
            None => {
                let index = self.chunk.add_const(Value::String(name.clone()));
                self.str_constants.insert(name, index);
                index
            }
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn def_var(&mut self, global_index: usize) {
        if self.scope_depth > 0 {
            return;
        }
        self.chunk
            .add_instr_with_arg(OpCode::DefGlobal, OpCode::DefGlobalLong, global_index);
    }

    fn decl_var(&mut self, name: String) {
        if self.scope_depth == 0 {
            return;
        }

        self.add_local(name);
    }

    fn add_local(&mut self, name: String) {
        self.locals.push((name, self.scope_depth));
    }

    fn add_instr(&mut self, op: OpCode) {
        self.current_chunk().add_instr(op);
    }

    fn write_const(&mut self, value: Value) {
        self.current_chunk().write_const(value);
    }

    fn resolve_local(&mut self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if name == local.0 {
                return Some(i);
            }
        }

        None
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some((_, depth)) = self.locals.last() {
            if *depth <= self.scope_depth {
                break;
            }

            self.add_instr(OpCode::Pop);
            self.locals.remove(self.locals.len() - 1);
        }
    }
}
