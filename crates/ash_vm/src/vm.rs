use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::{memory::Collectable, prelude::*};
use ash_bytecode::prelude::*;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    objects: Vec<&'a dyn Collectable>,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            objects: Vec::new(),
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
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
                    self.push(-v);
                }
                OpCode::Sum => self.bin_op(Add::add),
                OpCode::Sub => self.bin_op(Sub::sub),
                OpCode::Mul => self.bin_op(Mul::mul),
                OpCode::Div => self.bin_op(Div::div),
                OpCode::Rem => self.bin_op(Rem::rem),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Not => {
                    let v = self.pop();
                    self.push(!v);
                }
                OpCode::Eq => self.bin_op(Value::eq),
                OpCode::Neq => self.bin_op(Value::neq),
                OpCode::Gt => self.bin_op(Value::gt),
                OpCode::Lt => self.bin_op(Value::lt),
                OpCode::Gte => self.bin_op(Value::gte),
                OpCode::Lte => self.bin_op(Value::lte),
                OpCode::Pop => {
                    let _ = self.pop();
                }
                OpCode::DefGlobal => {
                    let name = self.read_const();
                    self.def_global(name.string_value());
                }
                OpCode::DefGlobalLong => {
                    let name = self.read_const_long();
                    self.def_global(name.string_value());
                }
                OpCode::LoadGlobal => {
                    let name = self.read_const().string_value();
                    self.load_global(name);
                }
                OpCode::LoadGlobalLong => {
                    let name = self.read_const_long().string_value();
                    self.load_global(name);
                }
                OpCode::StoreGlobal => {
                    let name = self.read_const().string_value();
                    self.store_global(name);
                }
                OpCode::StoreGlobalLong => {
                    let name = self.read_const_long().string_value();
                    self.load_global(name);
                }
                OpCode::LoadLocal => {
                    let slot = self.read_byte() as usize;
                    self.load_local(slot);
                }
                OpCode::LoadLocalLong => {
                    let slot = self.read_long();
                    self.load_local(slot)
                }
                OpCode::StoreLocal => {
                    let slot = self.read_byte() as usize;
                    self.store_local(slot);
                }
                OpCode::StoreLocalLong => {
                    let slot = self.read_long();
                    self.store_local(slot);
                }
                OpCode::JmpIfFalse => {
                    let offset = self.read_short();
                    let cond = self.peek();
                    if !cond.bool_value() {
                        self.ip += offset;
                    }
                }
                OpCode::Jmp => {
                    let offset = self.read_short();
                    self.ip += offset;
                }
            }
        }
    }

    fn def_global(&mut self, name: String) {
        self.globals.insert(name, self.peek().clone());
        let _ = self.pop();
    }

    fn load_global(&mut self, name: String) {
        let value = self.globals.get(&name).unwrap();
        self.push(value.clone());
    }

    fn store_global(&mut self, name: String) {
        let v = self.pop();
        self.globals.insert(name, v);
    }

    fn load_local(&mut self, slot: usize) {
        let v = self.stack[slot].clone();
        self.push(v);
    }

    fn store_local(&mut self, slot: usize) {
        let v = self.pop();
        self.stack[slot] = v;
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

    fn read_short(&mut self) -> usize {
        let c1 = self.read_byte() as usize;
        let c2 = self.read_byte() as usize;

        c1 | (c2 << 8) 
    }

    fn read_long(&mut self) -> usize {
        let c1 = self.read_byte() as usize;
        let c2 = self.read_byte() as usize;
        let c3 = self.read_byte() as usize;

        c1 | (c2 << 8) | (c3 << 16)
    }

    fn read_const(&mut self) -> Value {
        self.chunk.get_const(self.read_byte() as usize).clone()
    }

    fn read_const_long(&mut self) -> Value {
        let index = self.read_long();
        self.chunk.get_const(index).clone()
    }

    fn peek(&self) -> &Value {
        self.stack.last().unwrap()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
