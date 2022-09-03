use crate::{opcode::OpCode, prelude::Value};

#[derive(Default)]
pub struct Chunk {
    pub(crate) constants: Vec<Value>,
    pub code: Vec<u8>,
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
        self.add_instr_with_arg(OpCode::Const, OpCode::ConstLong, constant_index);
    }

    pub fn add_instr_with_arg(&mut self, op: OpCode, op_long: OpCode, arg: usize) {
        if arg < 256 {
            self.add_instr(op);
            self.write(arg as u8);
        } else {
            self.add_instr(op_long);
            self.write((arg & 0xff) as u8);
            self.write(((arg >> 8) & 0xff) as u8);
            self.write(((arg >> 16) & 0xff) as u8);
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    #[inline]
    pub fn get_byte(&self, offset: usize) -> u8 {
        self.code[offset]
    }

    #[inline]
    pub fn get_const(&self, index: usize) -> &Value {
        &self.constants[index]
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
