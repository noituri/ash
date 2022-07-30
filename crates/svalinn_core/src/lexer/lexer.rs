use crate::common::{Span, Spanned, SvResult};
use crate::lexer::basic::basic_lexer;
use crate::lexer::indent::indentation_lexer;
use crate::lexer::keyword::keyword_lexer;
use crate::lexer::numeric::numeric_lexer;
use crate::lexer::string::string_lexer;
use crate::lexer::token::{Delim, Token, TokenTree, TokenType};
use chumsky::prelude::*;
use chumsky::{BoxStream, Flat, Stream};

pub(crate) struct Lexer<'a>(BoxedParser<'a, char, Vec<Spanned<TokenTree>>, Simple<char>>);

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        let keywords = keyword_lexer();
        let num = numeric_lexer();
        let basic = basic_lexer();
        let string = string_lexer();

        let tt = recursive(|tt| {
            let tt_list = tt.padded().repeated();

            let delim_tree = |delim_l: char, delim_r: char, delim_t: Delim| {
                tt_list
                    .clone()
                    .delimited_by(just(delim_l), just(delim_r))
                    .map(move |tts| TokenTree::Tree(delim_t.clone(), tts))
            };

            keywords
                .or(string)
                .or(num)
                .or(basic)
                .map(TokenTree::Token)
                .or(delim_tree('(', ')', Delim::Paren))
                .or(delim_tree('{', '}', Delim::Brace))
                .or(delim_tree('[', ']', Delim::Bracket))
                .map_with_span(|tt, span| (tt, span))
        });

        let parser = indentation_lexer(tt, |tts| {
            let span = if tts.is_empty() {
                0..0usize
            } else {
                let start = tts.first().unwrap().1.start();
                let end = tts.last().unwrap().1.end();
                start..end
            };

            (TokenTree::Tree(Delim::Block, tts), span)
        })
        .then_ignore(end());

        Self(parser.boxed())
    }

    pub fn scan(&self, source: &str) -> SvResult<Vec<Spanned<Token>>> {
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
            TokenTree::Tree(Delim::Block, tt) => Flat::Many(
                once((TokenType::StartBlock.to_tree(), span_at(span.start)))
                    .chain(tt.into_iter())
                    .chain(once((TokenType::EndBlock.to_tree(), span_at(span.end)))),
            ),
            TokenTree::Tree(Delim::Paren, tt) => Flat::Many(
                once((TokenType::LParen.to_tree(), span_at(span.start)))
                    .chain(tt.into_iter())
                    .chain(once((TokenType::RParen.to_tree(), span_at(span.end - 1)))),
            ),
            TokenTree::Tree(_, _) => unimplemented!(),
        })
    }
}
