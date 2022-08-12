use crate::common::{AshResult, Source, StringError, Context};
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::resolver::Resolver;

pub fn build(source: &Source) -> AshResult<(), String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    dbg!(&ast);
    let mut context = Context::new();
    let mut resolver = Resolver::new(&mut context);
    resolver.run(&ast)?;

    Ok(())
}

pub fn run(source: &Source) -> AshResult<(), String> {
    build(source)
}
