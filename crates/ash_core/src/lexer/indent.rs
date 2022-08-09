use crate::common::Spanned;
use chumsky::{prelude::*, text::Character};

use crate::lexer::token::TokenTree;

use super::token::Token;

pub(super) fn handle_newlines<'a, T>(
    token: T,
) -> impl Parser<char, Vec<Spanned<TokenTree>>, Error = Simple<char>> + Clone + 'a
where
    T: Parser<char, Spanned<TokenTree>, Error = Simple<char>> + Clone + 'a,
{
    let line_ws = filter(|c: &char| c.is_inline_whitespace());
    let line = token.padded_by(line_ws.ignored().repeated()).repeated();
    let lines = line.separated_by(text::newline()).padded();

    lines.map(move |mut lines| {
        for line in lines.iter_mut() {
            if let Some((_, span)) = line.last() {
                let span = (span.end() - 1)..span.end();
                line.push((Token::NewLine.to_tree(), span));
            }
        }

        lines.into_iter().flatten().collect()
    })
}
