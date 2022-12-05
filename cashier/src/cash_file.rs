use std::{error::Error, fs, path::PathBuf};

use ash_core::prelude::*;
use llvm_sys::{core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder,
    LLVMDisposeModule, LLVMDumpModule, LLVMModuleCreateWithNameInContext, LLVMCreatePassManager, LLVMCreateFunctionPassManagerForModule, LLVMInitializeFunctionPassManager,
}, transforms::{instcombine::LLVMAddInstructionCombiningPass, scalar::{LLVMAddReassociatePass, LLVMAddGVNPass, LLVMAddCFGSimplificationPass}}};

use crate::compiler::{Compiler, RawStr};

pub struct CashFile {
    src: cash::Header,
}

impl CashFile {
    const MODULE_NAME: RawStr<'static> = RawStr(b"ash_root\0");

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
            let module = LLVMModuleCreateWithNameInContext(Self::MODULE_NAME.llvm_str(), ctx);
            let fpm = LLVMCreateFunctionPassManagerForModule(module);
            LLVMAddInstructionCombiningPass(fpm);
            LLVMAddReassociatePass(fpm);
            LLVMAddGVNPass(fpm);
            LLVMAddCFGSimplificationPass(fpm);
            LLVMInitializeFunctionPassManager(fpm);

            let mut compiler = Compiler::new(&self.src, ctx, builder, module, fpm);
            compiler.compile();

            LLVMDumpModule(module);
            LLVMDisposeModule(module);
            LLVMDisposeBuilder(builder);
            LLVMContextDispose(ctx);
        }
    }
}
