use ash_bytecode::prelude::*;

use crate::core::{Context, Spanned};

use super::Stmt;

pub(super) struct Compiler<'a> {
    context: &'a Context,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub fn run(&self, ast: Vec<Spanned<Stmt>>) -> Chunk {
        let mut chunk = Chunk::default();
        chunk.write_const(Value::F64(1.0)); 
        chunk.write_const(Value::F64(1.5)); 
        chunk.add_instr(OpCode::Negate);
        chunk.add_instr(OpCode::Sum);
        chunk.add_instr(OpCode::Return);
        chunk.print(self.context.location());

        chunk
    }
}
