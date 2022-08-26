use ash_bytecode::prelude::*;

use crate::core::{Context, Spanned};
use crate::parser::operator::{UnaryOp, BinaryOp};
use crate::ty;

use super::{Stmt, Expr};

pub(super) struct Compiler<'a> {
    context: &'a Context,
    chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { 
            context,
            chunk: Chunk::default()
        }
    }

    pub fn run(mut self, ast: Vec<Spanned<Stmt>>) -> Chunk {
        // let mut chunk = Chunk::default();
        // chunk.write_const(Value::F64(1.0));
        // chunk.write_const(Value::F64(2.0));
        // chunk.write_const(Value::F64(3.0));
        // chunk.add_instr(OpCode::Mul);
        // chunk.add_instr(OpCode::Sum);
        // chunk.write_const(Value::F64(4.0));
        // chunk.write_const(Value::F64(5.0));
        // chunk.add_instr(OpCode::Neg);
        // chunk.add_instr(OpCode::Div);
        // chunk.add_instr(OpCode::Sub);
        // chunk.add_instr(OpCode::Ret);
        // chunk.print(self.context.location());
        // chunk

        for (stmt, _) in ast {
            self.compile_stmt(stmt);
        }

        self.add_instr(OpCode::Ret);
        self.chunk
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr, _) => self.compile_expr(expr),
            _ => unimplemented!()
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Literal(value) => self.compile_literal(value),
            Expr::Unary { op, right, .. } => {
                self.compile_expr(*right);
                match op {
                    UnaryOp::Neg => self.add_instr(OpCode::Neg),
                    UnaryOp::Not => self.add_instr(OpCode::Not),
                }
            }
            Expr::Binary { left, op, right, .. } => {
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
            _ => unimplemented!()
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
                ty::Value::F64(v) => Value::F64(v),
                _ => unimplemented!()
            };
    
            self.write_const(value);
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn add_instr(&mut self, op: OpCode) {
        self.current_chunk().add_instr(op);
    }

    fn write_const(&mut self, value: Value) {
        self.current_chunk().write_const(value);
    }
}
