use std::{fmt, hash::Hash};

use chumsky::error::Simple;
pub use context::*;
pub use env::*;
pub use source::*;

pub mod context;
pub mod env;
pub mod source;

pub(crate) type Spanned<T> = (T, Span);

pub type Span = std::ops::Range<usize>;

pub type AshResult<T, E> = Result<T, Vec<Simple<E>>>;

pub(crate) trait StringError<T> {
    fn string_err(self) -> AshResult<T, String>;
}

impl<T, E: fmt::Display + Hash + Eq> StringError<T> for AshResult<T, E> {
    fn string_err(self) -> AshResult<T, String> {
        self.map_err(|err| {
            err.into_iter()
                .map(|e| e.map(|e| e.to_string()))
                .collect::<Vec<_>>()
        })
    }
}
