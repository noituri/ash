use crate::common::Spanned;
use chumsky::{prelude::*, text::Character};

use crate::lexer::token::TokenTree;

use super::token::Token;

pub(super) fn indentation_lexer<'a, T, F>(
    token: T,
    make_group: F,
) -> impl Parser<char, Vec<Spanned<TokenTree>>, Error = Simple<char>> + Clone + 'a
where
    T: Parser<char, Spanned<TokenTree>, Error = Simple<char>> + Clone + 'a,
    F: Fn(Vec<Spanned<TokenTree>>) -> Option<Spanned<TokenTree>> + Clone + 'a,
{
    let line_ws = filter(|c: &char| c.is_inline_whitespace());

    let line = token.padded_by(line_ws.ignored().repeated()).repeated();

    let lines = line_ws
        .repeated()
        .then(line)
        .separated_by(text::newline())
        .padded();

    lines.map(move |lines| {
        fn collapse<F>(
            mut tree: Vec<(Vec<char>, Vec<Spanned<TokenTree>>)>,
            make_group: &F,
        ) -> Option<Spanned<TokenTree>>
        where
            F: Fn(Vec<Spanned<TokenTree>>) -> Option<Spanned<TokenTree>>,
        {
            while let Some((_, tts)) = tree.pop() {
                if let Some(tt) = make_group(tts) {
                    if let Some(last) = tree.last_mut() {
                        last.1.push(tt);
                    } else {
                        return Some(tt);
                    }
                }
            }
            None
        }

        let mut nesting = vec![(Vec::new(), Vec::new())];
        for (indent, mut line) in lines {
            let mut indent = indent.as_slice();
            let mut i = 0;
            if let Some((_, span)) = line.last() {
                let span = (span.end() - 1)..span.end();
                line.push((Token::NewLine.to_tree(), span));
            }
            while let Some(tail) = nesting
                .get(i)
                .and_then(|(n, _)| indent.strip_prefix(n.as_slice()))
            {
                indent = tail;
                i += 1;
            }
            if let Some(mut tail) = collapse(nesting.split_off(i), &make_group) {
                nesting.last_mut().unwrap().1.push(tail);
            }
            if indent.len() > 0 {
                nesting.push((indent.to_vec(), line));
            } else {
                nesting.last_mut().unwrap().1.append(&mut line);
            }
        }

        nesting.remove(0).1
    })
}
