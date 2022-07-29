use crate::common::Source;
use crate::lexer::Lexer;

pub fn build(mut source: Source) {
    let lexer = Lexer::new();
    match lexer.scan(&source.to_string()) {
        Ok(tts) => println!("{:#?}", tts),
        Err(errs) => errs.into_iter().for_each(|e| println!("{:#?}", e)),
    }
}

pub fn run(source: Source) {
    build(source);
}
