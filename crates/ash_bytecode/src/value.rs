use std::fmt;

#[derive(Clone)]
pub enum Value {
    F64(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::F64(v) => format!("{:.2}", v),
        };

        f.write_str(&s)
    }
}


impl Value {
    pub fn neg(self) -> Self {
        match self {
            Self::F64(v) => Self::F64(-v)
        }
    }

    pub fn sum(self, other: Self) -> Self {
        match (self, other) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 + v2),
        }
    }

    pub fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 - v2),
        }
    }

    pub fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 * v2),
        }
    }

    pub fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 / v2),
        }
    }
}