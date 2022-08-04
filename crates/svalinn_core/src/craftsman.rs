use crate::common::{Source, StringError, SvResult};
use crate::lexer::Lexer;
use crate::parser::parser::Parser;

pub fn build(source: &Source) -> SvResult<(), String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    dbg!(&tokens);
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    dbg!(&ast);
    Ok(())
}

pub fn run(source: &Source) -> SvResult<(), String> {
    build(source)
}
