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
    Rem = 8,
    True = 9,
    False = 10,
    Not = 11,
    Eq = 12,
    Neq = 13,
    Gt = 14,
    Lt = 15,
    Gte = 16,
    Lte = 17,
    Pop = 18,
    DefGlobal = 19,
    DefGlobalLong = 20,
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
            Self::Rem => "OP_REM",
            Self::True => "OP_TRUE",
            Self::False => "OP_FALSE",
            Self::Not => "OP_NOT",
            Self::Eq => "OP_EQ",
            Self::Neq => "OP_NEQ",
            Self::Gt => "OP_GT",
            Self::Lt => "OP_LT",
            Self::Gte => "OP_GTE",
            Self::Lte => "OP_LTE",
            Self::Pop => "OP_POP",
            Self::DefGlobal => "OP_DEF_GLOBAL",
            Self::DefGlobalLong => "OP_DEF_GLOBAL_LONG",
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
            8 => Self::Rem,
            9 => Self::True,
            10 => Self::False,
            11 => Self::Not,
            12 => Self::Eq,
            13 => Self::Neq,
            14 => Self::Gt,
            15 => Self::Lt,
            16 => Self::Gte,
            17 => Self::Lte,
            18 => Self::Pop,
            19 => Self::DefGlobal,
            20 => Self::DefGlobalLong,
            _ => unreachable!("Operation does not exist: {b}"),
        }
    }
}

impl OpCode {
    pub fn print(&self, chunk: &Chunk, offset: usize) -> usize {
        match self {
            Self::Ret
            | Self::Neg
            | Self::Sum
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Rem
            | Self::True
            | Self::False
            | Self::Not
            | Self::Eq
            | Self::Neq
            | Self::Gt
            | Self::Lt
            | Self::Gte
            | Self::Lte
            | Self::Pop => {
                println!("{}", self.to_string());
                offset + 1
            }
            Self::Const | Self::DefGlobal => {
                let constant = chunk.code[offset + 1];
                let value = &chunk.constants[constant as usize];
                println!(
                    "{} `{}` at {}",
                    self.to_string(),
                    value.to_string(),
                    constant
                );
                offset + 2
            }
            Self::ConstLong | Self::DefGlobalLong => {
                let constant = {
                    let c1 = chunk.code[offset + 1] as usize;
                    let c2 = chunk.code[offset + 2] as usize;
                    let c3 = chunk.code[offset + 3] as usize;

                    // Little-edian
                    c1 | (c2 << 8) | (c3 << 16)
                };

                let value = &chunk.constants[constant];
                println!(
                    "{} `{}` at {}",
                    self.to_string(),
                    value.to_string(),
                    constant
                );
                offset + 4
            }
        }
    }
}
