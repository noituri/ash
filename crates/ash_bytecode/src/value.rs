use std::fmt;

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