use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use ash_bytecode::prelude::Chunk;

use crate::core::{AshResult, Context, Source, StringError};
use crate::hir::Desugarer;
use crate::ir::IR;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::resolver::Resolver;

pub fn build(source: &Source) -> AshResult<(), String> {
    let lexer = Lexer::new();
    let tokens = lexer.scan(source.inner()).string_err()?;
    let parser = Parser::new();
    let ast = parser.parse(tokens).string_err()?;
    let mut context = Context::new(source.location());

    let resolver = Resolver::new(&mut context);
    resolver.run(&ast)?;
    let hir = Desugarer::run(&mut context, ast);
    dbg!(&hir);
    // let type_system = TypeSystem::new(&mut context);
    // let typed_ast = type_system.run(ast)?;
    // let ir = IR::new(&mut context);
    // let bytes = ir.run(typed_ast);

    Ok(())
}


fn write_out(source: &Source, bytes: &[u8]) -> AshResult<(), String> {
    // let mut src_path = PathBuf::from(source.location());
    // let mut path = src_path.parent()
    //     .unwrap()
    //     .to_path_buf();
    // path.push("out");
    // fs::create_dir(path).map_err(|e| vec![chumsky])?; 
    
    // src_path.set_extension("cash");
    // path.push(src_path.file_name().unwrap());
    
    // let file = File::create(path).map_err(|e| e.to_string())?;
    // file.write_all(bytes);

    Ok(())
}
