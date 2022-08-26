use std::{fmt, ops::{Neg, Add, Sub, Mul, Div, Rem, Not}};

#[derive(Clone)]
pub enum Value {
    F64(f64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::F64(v) => format!("{:.2}", v),
            Self::Bool(v) => format!("{v}"),
        };

        f.write_str(&s)
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(v) => Self::Bool(!v),
            _ => unreachable!()
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::F64(v) => Self::F64(-v),
            _ => unreachable!()
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 + v2),
            _ => unreachable!()
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 - v2),
            _ => unreachable!()
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 * v2),
            _ => unreachable!()
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 / v2),
            _ => unreachable!()
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F64(v1), Self::F64(v2)) => Self::F64(v1 % v2),
            _ => unreachable!()
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::F64(v1), Self::F64(v2)) => v1.partial_cmp(v2),
            _ => unreachable!()
        }
    }
}

impl Value {
    pub fn eq(self, other: Self) -> Value {
        Value::Bool(self == other)
    }

    pub fn neq(self, other: Self) -> Value {
        self.eq(other).not()
    }

    pub fn gt(self, other: Self) -> Value {
        Value::Bool(self > other)
    }
    
    pub fn lt(self, other: Self) -> Value {
        Value::Bool(self < other)
    }

    pub fn gte(self, other: Self) -> Value {
        Value::Bool(self >= other)
    }
    
    pub fn lte(self, other: Self) -> Value {
        Value::Bool(self <= other)
    }
}