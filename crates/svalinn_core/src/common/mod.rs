use chumsky::error::Simple;
pub use source::*;

pub mod source;

pub(crate) type Spanned<T> = (T, Span);

pub type Span = std::ops::Range<usize>;

pub type SvResult<T = ()> = Result<T, Vec<Simple<char>>>;
