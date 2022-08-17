use crate::core::{AshResult, Context, Source, StringError};
use crate::ir::IR;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::resolver::Resolver;
use crate::ty::TypeSystem;

pub fn build(source: &Source) -> AshResult<(), String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    let mut context = Context::new();

    let resolver = Resolver::new(&mut context);
    resolver.run(&ast)?;
    let type_system = TypeSystem::new(&mut context);
    let typed_ast = type_system.run(ast)?;
    dbg!(&typed_ast);
    let ir = IR::new();
    let ir = ir.run(typed_ast);
    dbg!(&ir);

    Ok(())
}

pub fn run(source: &Source) -> AshResult<(), String> {
    build(source)
}
