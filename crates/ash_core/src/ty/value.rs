#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
}
