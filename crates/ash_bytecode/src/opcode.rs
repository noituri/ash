use std::fmt;

use crate::prelude::Chunk;

#[repr(u8)]
pub enum OpCode {
    Ret = 0,
    Const = 1,
    ConstLong = 2,
    Neg = 3,
    Sum = 4,
    Sub = 5,
    Mul = 6,
    Div = 7,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ret => "OP_RET",
            Self::Const => "OP_CONST",
            Self::ConstLong => "OP_CONST_LONG",
            Self::Neg => "OP_NEG",
            Self::Sum => "OP_SUM",
            Self::Sub => "OP_SUB",
            Self::Mul => "OP_MUL",
            Self::Div => "OP_DIV",
        };

        f.write_str(s)
    }
}

impl From<u8> for OpCode {
    fn from(b: u8) -> Self {
       match b {
            0 => Self::Ret,
            1 => Self::Const,
            2 => Self::ConstLong,
            3 => Self::Neg,
            4 => Self::Sum,
            5 => Self::Sub,
            6 => Self::Mul,
            7 => Self::Div,
            _ => unreachable!("Operation does not exist: {b}")
       } 
    }
}

impl OpCode {
    pub fn print(&self, chunk: &Chunk, offset: usize) -> usize {
        match self {
           Self::Ret | Self::Neg | Self::Sum | Self::Sub | Self::Mul | Self::Div => {
                println!("{}", self.to_string());
                offset + 1
           } 
           Self::Const => {
                let constant = chunk.code[offset+1];
                let value = &chunk.constants[constant as usize];
                println!("{} `{}` at {}", self.to_string(), value.to_string(), constant);
                offset + 2
           }
           Self::ConstLong => {
                let constant = {
                    let c1 = chunk.code[offset+1] as usize;
                    let c2 = chunk.code[offset+2] as usize;
                    let c3 = chunk.code[offset+3] as usize;

                    // Little-edian
                    c1 | (c2 << 8) | (c3 << 16)
                };

                let value = &chunk.constants[constant];
                println!("{} `{}` at {}", self.to_string(), value.to_string(), constant);
                offset + 4
           }
        }
    } 
}