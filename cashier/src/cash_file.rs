use std::{error::Error, fs, path::PathBuf};

use ash_core::prelude::*;
use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder,
    LLVMDisposeModule, LLVMDumpModule, LLVMModuleCreateWithNameInContext,
};

use crate::compiler::{Compiler, RawStr};

pub struct CashFile {
    src: cash::Header,
}

impl CashFile {
    const MODULE_NAME: RawStr<'static> = RawStr("ash_root".as_bytes());

    pub fn new(src: cash::Header) -> Self {
        Self { src }
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn Error>> {
        let bytes = fs::read(path.into())?;
        let src = bincode::deserialize(&bytes[..])?;
        Ok(Self::new(src))
    }

    pub fn compile(&self) {
        unsafe {
            let ctx = LLVMContextCreate();
            let builder = LLVMCreateBuilderInContext(ctx);
            let module =
                LLVMModuleCreateWithNameInContext(Self::MODULE_NAME.llvm_str(), ctx);
            // TODO: Optimization pass
            // let fpm = PassManager::create(&module);
            // fpm.add_instruction_combining_pass();
            // fpm.add_reassociate_pass();
            // fpm.add_gvn_pass();
            // fpm.add_cfg_simplification_pass();
            // fpm.add_basic_alias_analysis_pass();
            // fpm.add_promote_memory_to_register_pass();
            // fpm.add_instruction_combining_pass();
            // fpm.add_reassociate_pass();

            // fpm.initialize();

            let mut compiler = Compiler::new(&self.src, ctx, builder, module);
            compiler.compile();

            LLVMDumpModule(module);
            LLVMDisposeModule(module);
            LLVMDisposeBuilder(builder);
            LLVMContextDispose(ctx);
        }
    }
}
