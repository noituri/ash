use std::{
    error::Error,
    ffi::{CStr, CString},
    fs,
    mem::MaybeUninit,
    path::PathBuf,
    process::Command,
};

use ash_core::prelude::*;
use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
        LLVMCreateFunctionPassManagerForModule, LLVMCreatePassManager, LLVMDisposeBuilder,
        LLVMDisposeMessage, LLVMDisposeModule, LLVMDumpModule, LLVMInitializeFunctionPassManager,
        LLVMModuleCreateWithNameInContext,
    },
    target::{
        LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos,
        LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
    },
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine,
        LLVMGetDefaultTargetTriple, LLVMGetFirstTarget, LLVMGetTargetFromTriple, LLVMRelocMode,
        LLVMTarget, LLVMTargetMachineEmitToFile, LLVMTargetRef,
    },
    transforms::{
        instcombine::LLVMAddInstructionCombiningPass,
        scalar::{LLVMAddCFGSimplificationPass, LLVMAddGVNPass, LLVMAddReassociatePass},
    },
};

use crate::compiler::{Compiler, RawStr};

pub struct CashFile {
    name: String,
    src: cash::Header,
}

// TODO: Remake it properly
impl CashFile {
    const MODULE_NAME: RawStr<'static> = RawStr(b"ash_root\0");

    pub fn new(name: String, src: cash::Header) -> Self {
        Self { name, src }
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn Error>> {
        let path: PathBuf = path.into();
        let name = path
            .file_name()
            .map(|f| f.to_str().unwrap())
            .unwrap_or("unknown")
            .trim_end_matches(".cash")
            .to_owned();
        let bytes = fs::read(path)?;
        let src = bincode::deserialize(&bytes[..])?;
        Ok(Self::new(name, src))
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
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmParsers();
            LLVM_InitializeAllAsmPrinters();

            let mut compiler = Compiler::new(&self.src, ctx, builder, module, fpm);
            compiler.compile();
            // LLVMDumpModule(module);

            // FIXME: Temporary hacks below!
            let triple = LLVMGetDefaultTargetTriple();
            println!("Using {} target", CStr::from_ptr(triple).to_str().unwrap());
            let mut target = MaybeUninit::<LLVMTargetRef>::uninit();
            let mut error = CString::default().into_raw();
            LLVMGetTargetFromTriple(triple, target.as_mut_ptr(), &mut error);

            let target_machine = LLVMCreateTargetMachine(
                target.assume_init(),
                triple,
                "generic\0".as_ptr() as *const i8,
                "\0".as_ptr() as *const i8,
                LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
                LLVMRelocMode::LLVMRelocDefault,
                LLVMCodeModel::LLVMCodeModelDefault,
            );

            let file_type = LLVMCodeGenFileType::LLVMObjectFile;
            LLVMTargetMachineEmitToFile(
                target_machine,
                module,
                format!("./{}.o\0", self.name).as_ptr() as *mut i8,
                file_type,
                &mut error,
            );
            

            Command::new("clang")
                .args([
                    &format!("./{}.o", self.name),
                    "-o",
                    &format!("{}.exe", self.name),
                ])
                .output()
                .expect("failed to execute process");

            fs::remove_file(format!("./{}.o", self.name));

            LLVMDisposeBuilder(builder);
            LLVMContextDispose(ctx);
        }
    }
}
