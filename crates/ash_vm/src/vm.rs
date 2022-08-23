use ash_bytecode::prelude::*;
use crate::prelude::*;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value> 
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256)
        }
    }

    pub fn run(&mut self) -> VMResult {
        loop {
            let instr: OpCode = self.read_byte().into();
            #[cfg(feature = "debug_info")]
            instr.print(self.chunk, self.offset);

            match instr {
                OpCode::Return => return Ok(()),
                OpCode::Constant => {
                    let constant = self.read_const();
                    println!("{}", constant.to_string());
                }
                OpCode::ConstantLong => {
                    let constant = self.read_const_long();
                    println!("{}", constant.to_string());
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.get_byte(self.ip);
        self.ip += 1;
        b
    }

    fn read_const(&mut self) -> &Value {
       self.chunk.get_const(self.read_byte() as usize) 
    }

    fn read_const_long(&mut self) -> &Value {
        let index = {
            let c1 = self.read_byte() as usize;
            let c2 = self.read_byte() as usize;
            let c3 = self.read_byte() as usize;

            c1 | (c2 << 8) | (c3 << 16)
        };
        
        self.chunk.get_const(index)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}