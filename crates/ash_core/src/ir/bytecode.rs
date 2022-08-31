use std::collections::HashMap;

use ash_bytecode::prelude::*;

use crate::core::{Context, Spanned};
use crate::parser::operator::{BinaryOp, UnaryOp};
use crate::ty;

use super::{Expr, Stmt};

pub(super) struct Compiler<'a> {
    context: &'a Context,
    chunk: Chunk,
    str_constants: HashMap<String, usize>,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            chunk: Chunk::default(),
            str_constants: HashMap::new(),
        }
    }

    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Chunk {
        for (stmt, _) in ast {
            self.compile_stmt(stmt, true);
        }

        self.add_instr(OpCode::Ret);
        self.chunk
    }

    fn compile_stmt(&mut self, stmt: Stmt, _is_global: bool) {
        match stmt {
            Stmt::Expression(expr, _) => {
                self.compile_expr(expr);
                self.add_instr(OpCode::Pop);
            }
            Stmt::VariableDecl { name, value, .. } => {
                let name_index = self.compile_identifier(name);
                self.compile_expr(value);
                self.def_global(name_index);
            }
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
        let name_index = self.compile_identifier(name);
        self.chunk
            .add_instr_with_arg(OpCode::LoadGlobal, OpCode::LoadGlobalLong, name_index);
    }

    fn compile_identifier(&mut self, name: String) -> usize {
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

    fn def_global(&mut self, global_index: usize) {
        self.chunk
            .add_instr_with_arg(OpCode::DefGlobal, OpCode::DefGlobalLong, global_index);
    }

    fn add_instr(&mut self, op: OpCode) {
        self.current_chunk().add_instr(op);
    }

    fn write_const(&mut self, value: Value) {
        self.current_chunk().write_const(value);
    }
}
