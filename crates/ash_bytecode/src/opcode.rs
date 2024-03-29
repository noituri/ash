use std::fmt;

use crate::prelude::Chunk;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
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
    LoadGlobal = 21,
    LoadGlobalLong = 22,
    StoreGlobal = 23,
    StoreGlobalLong = 24,
    LoadLocal = 25,
    LoadLocalLong = 26,
    StoreLocal = 27,
    StoreLocalLong = 28,
    JmpIfFalse = 29,
    Jmp = 30,
    Loop = 31,
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
            Self::LoadGlobal => "OP_LOAD_GLOBAL",
            Self::LoadGlobalLong => "OP_LOAD_GLOBAL_LONG",
            Self::StoreGlobal => "OP_STORE_GLOBAL",
            Self::StoreGlobalLong => "OP_STORE_GLOBAL_LONG",
            Self::LoadLocal => "OP_LOAD_LOCAL",
            Self::LoadLocalLong => "OP_LOAD_LOCAL_LONG",
            Self::StoreLocal => "OP_STORE_LOCAL",
            Self::StoreLocalLong => "OP_STORE_LOCAL_LONG",
            Self::JmpIfFalse => "OP_JMP_IF_FALSE",
            Self::Jmp => "OP_JMP",
            Self::Loop => "OP_LOOP",
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
            21 => Self::LoadGlobal,
            22 => Self::LoadGlobalLong,
            23 => Self::StoreGlobal,
            24 => Self::StoreGlobalLong,
            25 => Self::LoadLocal,
            26 => Self::LoadLocalLong,
            27 => Self::StoreLocal,
            28 => Self::StoreLocalLong,
            29 => Self::JmpIfFalse,
            30 => Self::Jmp,
            31 => Self::Loop,
            _ => unreachable!("Operation does not exist: {b}"),
        }
    }
}

impl OpCode {
    pub fn print(&self, chunk: &Chunk, offset: usize) -> usize {
        let read_long = || {
            let c1 = chunk.code[offset + 1] as usize;
            let c2 = chunk.code[offset + 2] as usize;
            let c3 = chunk.code[offset + 3] as usize;

            // Little-edian
            c1 | (c2 << 8) | (c3 << 16)
        };

        let read_short = || {
                    let c1 = chunk.code[offset + 1] as usize;
                    let c2 = chunk.code[offset + 2] as usize;

                    c1 | (c2 << 8) 
        };

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
            Self::Const | Self::DefGlobal | Self::LoadGlobal | Self::StoreGlobal => {
                let index = chunk.code[offset + 1];
                let value = &chunk.constants[index as usize];
                println!("{} `{}` at {}", self.to_string(), value.to_string(), index);
                offset + 2
            }
            Self::ConstLong
            | Self::DefGlobalLong
            | Self::LoadGlobalLong
            | Self::StoreGlobalLong => {
                let index = read_long();
                let value = &chunk.constants[index];
                println!("{} `{}` at {}", self.to_string(), value.to_string(), index,);
                offset + 4
            }
            Self::LoadLocal | Self::StoreLocal => {
                let slot = chunk.code[offset + 1];
                println!("{} slot {}", self.to_string(), slot);
                offset + 2
            }
            Self::LoadLocalLong | Self::StoreLocalLong => {
                let slot = read_long();
                println!("{} slot {}", self.to_string(), slot);
                offset + 2
            }
            Self::JmpIfFalse | Self::Jmp | Self::Loop => {
                let jmp = read_short() as i64;
                let sign = if *self == Self::Loop {
                    -1
                } else {
                    1
                };
                println!("{} {} -> {}", self.to_string(), offset, (offset as i64) + 3 + sign * jmp);
                offset + 3
            }
        }
    }
}
