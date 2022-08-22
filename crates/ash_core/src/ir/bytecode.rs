use ash_bytecode::prelude::*;

use crate::core::Context;

pub(super) struct Compiler<'a> {
    context: &'a Context,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub fn run(&self) {
        let mut chunk = Chunk::default();
        chunk.write_const(Value::F64(1.0)); 
        chunk.add_instr(OpCode::Return);
        chunk.print(self.context.location());
    }
}
