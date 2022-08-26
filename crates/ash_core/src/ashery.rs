use ash_bytecode::prelude::Chunk;

use crate::core::{AshResult, Context, Source, StringError};
use crate::ir::IR;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::resolver::Resolver;
use crate::ty::TypeSystem;

pub fn build(source: &Source) -> AshResult<Chunk, String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    let mut context = Context::new(source.location());

    let resolver = Resolver::new(&mut context);
    resolver.run(&ast)?;
    let type_system = TypeSystem::new(&mut context);
    let typed_ast = type_system.run(ast)?;
    let ir = IR::new(&mut context);
    let chunk = ir.run(typed_ast);

    Ok(chunk)
}
