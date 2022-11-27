use std::{error::Error, fs, path::PathBuf};

use ash_core::prelude::*;
use inkwell::{context::Context, passes::PassManager};

use crate::compiler::Compiler;

pub struct CashFile {
    src: cash::Header,
}

impl CashFile {
    const MODULE_NAME: &str = "ash_root";

    pub fn new(src: cash::Header) -> Self {
        Self { src }
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path.into())?;
        let src = bincode::deserialize(&bytes[..])?;
        Ok(Self::new(src))
    }

    pub fn compile(&self) {
        let ctx = Context::create();
        let builder = ctx.create_builder();
        let module = ctx.create_module(Self::MODULE_NAME);
        let fpm = PassManager::create(&module);
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();

        fpm.initialize();

        let mut compiler = Compiler::new(&self.src, &ctx, builder, module, fpm);
        compiler.compile();
    }
}
