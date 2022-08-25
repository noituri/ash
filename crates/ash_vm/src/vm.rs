use crate::prelude::*;
use ash_bytecode::prelude::*;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn run(&mut self) -> VMResult {
        loop {
            let instr: OpCode = self.read_byte().into();
            #[cfg(feature = "debug_info")]
            {
                print!("Stack: ");
                if self.stack.is_empty() {
                    print!("| ");
                }
                for v in self.stack.iter() {
                    print!("| {} ", v.to_string())
                }
                println!("|");
                instr.print(self.chunk, self.ip - 1);
            }

            match instr {
                OpCode::Ret => {
                    println!("{}", self.pop().to_string());
                    return Ok(());
                }
                OpCode::Const => {
                    let constant = self.read_const();
                    self.push(constant);
                }
                OpCode::ConstLong => {
                    let constant = self.read_const_long();
                    self.push(constant);
                }
                OpCode::Neg => {
                    let v = self.pop();
                    self.push(v.neg());
                }
                OpCode::Sum => self.bin_op(Value::sum),
                OpCode::Sub => self.bin_op(Value::sub),
                OpCode::Mul => self.bin_op(Value::mul),
                OpCode::Div => self.bin_op(Value::div),
            }
        }
    }

    fn bin_op<F>(&mut self, op_f: F)
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let b = self.pop();
        let a = self.pop();
        self.push(op_f(a, b))
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.get_byte(self.ip);
        self.ip += 1;
        b
    }

    fn read_const(&mut self) -> Value {
        self.chunk.get_const(self.read_byte() as usize).clone()
    }

    fn read_const_long(&mut self) -> Value {
        let index = {
            let c1 = self.read_byte() as usize;
            let c2 = self.read_byte() as usize;
            let c3 = self.read_byte() as usize;

            c1 | (c2 << 8) | (c3 << 16)
        };

        self.chunk.get_const(index).clone()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
