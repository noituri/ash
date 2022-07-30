use crate::common::{Source, SvResult};
use crate::lexer::Lexer;
use chumsky::error::Simple;

pub fn build(source: &Source) -> SvResult {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner())?;
    dbg!(&tokens);
    Ok(())
}

pub fn run(source: &Source) -> SvResult {
    build(source)
}
