use std::fmt;

use crate::prelude::Chunk;

#[repr(u8)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    ConstantLong = 2,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Return => "OP_RETURN",
            Self::Constant => "OP_CONSTANT",
            Self::ConstantLong => "OP_CONSTANT_LONG"
        };

        f.write_str(s)
    }
}

impl From<u8> for OpCode {
    fn from(b: u8) -> Self {
       match b {
            0 => Self::Return,
            1 => Self::Constant,
            2 => Self::ConstantLong,
            _ => unreachable!("Operation does not exist: {b}")
       } 
    }
}

impl OpCode {
    pub(crate) fn print(&self, chunk: &Chunk, offset: usize) -> usize {
        match self {
           Self::Return => {
                println!("{}", self.to_string());
                offset + 1
           } 
           Self::Constant => {
                let constant = chunk.code[offset+1];
                let value = &chunk.constants[constant as usize];
                println!("{} {:>4} {}", self.to_string(), constant, value.to_string());
                offset + 2
           }
           Self::ConstantLong => {
                let constant = {
                    let c1 = chunk.code[offset+1] as usize;
                    let c2 = chunk.code[offset+2] as usize;
                    let c3 = chunk.code[offset+3] as usize;

                    // Little-edian
                    c1 | (c2 << 8) | (c3 << 16)
                };

                let value = &chunk.constants[constant];
                println!("{} {:>4} {}", self.to_string(), constant, value.to_string());
                offset + 4
           }
        }
    } 
}