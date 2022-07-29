mod basic;
mod indent;
mod keyword;
pub mod lexer;
mod numeric;
mod string;
pub mod token;

use chumsky::prelude::*;

use crate::lexer::token::TokenTree;
pub(crate) use lexer::*;

pub(crate) type Span = std::ops::Range<usize>;

pub(crate) type SpanTokenTree = (TokenTree, Span);
