use crate::{opcode::OpCode, prelude::Value};

#[derive(Default)]
pub struct Chunk {
    pub(crate) constants: Vec<Value>,
    pub(crate) code: Vec<u8>,
}

impl Chunk {
    pub fn add_instr(&mut self, op: OpCode) {
        self.write(op as u8);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_const(&mut self, value: Value) {
        let constant_index = self.add_const(value);
        if constant_index < 256 {
            self.add_instr(OpCode::Constant);
            self.write(constant_index as u8);
        } else {
            self.add_instr(OpCode::ConstantLong);
            // Little-edian
            self.write((constant_index & 0xff) as u8);
            self.write(((constant_index >> 8) & 0xff) as u8);
            self.write(((constant_index >> 16) & 0xff) as u8);
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn print(&self, name: &str) {
        println!("-= {name} =-");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.print_instruction(offset);
        }
    }

    fn print_instruction(&self, offset: usize) -> usize {
        print!("{:0>5} ", offset);
        let op: OpCode = self.code[offset].into();
        op.print(self, offset)
    }
}
