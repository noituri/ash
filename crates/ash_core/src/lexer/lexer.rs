use crate::core::{AshResult, Span, Spanned};
use crate::lexer::basic::basic_lexer;
use crate::lexer::keyword::keyword_lexer;
use crate::lexer::numeric::numeric_lexer;
use crate::lexer::string::string_lexer;
use crate::lexer::token::{Delim, Token, TokenTree};
use chumsky::prelude::*;
use chumsky::text::Character;
use chumsky::{BoxStream, Flat, Stream};

pub(crate) struct Lexer<'a>(BoxedParser<'a, char, Vec<Spanned<TokenTree>>, Simple<char>>);

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        let tt = recursive(|tt| {
            let tt_list = tt.repeated();

            let delim_tree = |delim_l: char, delim_r: char, delim_t: Delim| {
                tt_list
                    .clone()
                    .delimited_by(just(delim_l), just(delim_r))
                    .map(move |tts| TokenTree::Tree(delim_t.clone(), tts))
            };

            keyword_lexer()
                .or(string_lexer())
                .or(numeric_lexer())
                .or(basic_lexer())
                .map(TokenTree::Token)
                .or(delim_tree('(', ')', Delim::Paren))
                .or(delim_tree('{', '}', Delim::Brace))
                .or(delim_tree('[', ']', Delim::Bracket))
                .map_with_span(|tt, span| (tt, span))
                .padded()
        });

        let parser = tt.repeated().then_ignore(end());
        Self(parser.boxed())
    }

    pub fn scan(&self, source: &str) -> AshResult<Vec<Spanned<Token>>, char> {
        let result = self.0.parse(source)?;
        let tokens = Self::flatten_token_trees(result)
            .fetch_tokens()
            .into_iter()
            .collect::<Vec<_>>();

        Ok(tokens)
    }

    fn flatten_token_trees(tts: Vec<Spanned<TokenTree>>) -> BoxStream<'static, Token, Span> {
        use std::iter::once;
        let eoi = if let Some(tok) = tts.last() {
            let span = tok.1.end;
            span..span
        } else {
            0..0
        };

        let span_at = |at| at..at + 1;

        Stream::from_nested(eoi, tts.into_iter(), move |(tt, span)| match tt {
            TokenTree::Token(tok) => Flat::Single((tok, span)),
            TokenTree::Tree(Delim::Brace, tt) => Flat::Many(
                once((
                    Token::LBrace.to_tree(),
                    span_at(span.start.saturating_sub(1)),
                ))
                .chain(tt.into_iter())
                .chain(once((Token::RBrace.to_tree(), span_at(span.end)))),
            ),
            TokenTree::Tree(Delim::Paren, tt) => Flat::Many(
                once((Token::LParen.to_tree(), span_at(span.start)))
                    .chain(tt.into_iter())
                    .chain(once((Token::RParen.to_tree(), span_at(span.end - 1)))),
            ),
            TokenTree::Tree(Delim::Bracket, tt) => Flat::Many(
                once((Token::LBracket.to_tree(), span_at(span.start)))
                    .chain(tt.into_iter())
                    .chain(once((Token::RBracket.to_tree(), span_at(span.end - 1)))),
            ),
        })
    }
}
