use crate::common::{AshResult, Source, StringError};
use crate::lexer::Lexer;
use crate::parser::parser::Parser;

pub fn build(source: &Source) -> AshResult<(), String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    dbg!(&ast);
    Ok(())
}

pub fn run(source: &Source) -> AshResult<(), String> {
    build(source)
}
