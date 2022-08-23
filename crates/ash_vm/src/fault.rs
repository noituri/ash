use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Runtime error: {0}")]
    RuntimeError(String)
}

pub type VMResult<T = ()> = Result<T, VMError>;