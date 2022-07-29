pub(crate) use lexer::*;

use crate::lexer::token::TokenTree;

mod basic;
mod indent;
mod keyword;
pub mod lexer;
mod numeric;
mod string;
pub mod token;

pub(crate) type Span = std::ops::Range<usize>;

pub(crate) type SpanTokenTree = (TokenTree, Span);
