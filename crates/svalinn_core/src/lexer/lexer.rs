use crate::lexer::basic::basic_lexer;
use crate::lexer::indent::indentation_lexer;
use crate::lexer::keyword::keyword_lexer;
use crate::lexer::numeric::numeric_lexer;
use crate::lexer::string::string_lexer;
use crate::lexer::token::{Delim, TokenTree};
use crate::lexer::{SpanTokenTree};
use chumsky::prelude::*;

pub type LexResult<T = ()> = Result<T, Vec<Simple<char>>>;

pub(crate) struct Lexer<'a>(BoxedParser<'a, char, Vec<SpanTokenTree>, Simple<char>>);

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
                .or(delim_tree('(', ')', Delim::Paren))
                .or(delim_tree('{', '}', Delim::Brace))
                .or(delim_tree('[', ']', Delim::Bracket))
                // .padded_by(comment_lexer().padded().repeated())
                .map_with_span(|tt, span| (tt, span))
        });

        let parser = indentation_lexer(tt, |tts| {
            let span = if tts.is_empty() {
                0..0
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

    pub fn scan(&self, source: &str) -> LexResult<Vec<SpanTokenTree>> {
        self.0.parse(source)
    }
}
