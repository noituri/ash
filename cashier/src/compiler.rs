use std::{collections::HashMap, ffi::c_char, fmt};

use ash_core::prelude::*;
use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildAdd, LLVMBuildAlloca,
        LLVMBuildFAdd, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildRet,
        LLVMBuildRetVoid, LLVMBuildStore, LLVMBuildSub, LLVMConstReal, LLVMDoubleTypeInContext,
        LLVMFloatTypeInContext, LLVMFunctionType, LLVMGetParam, LLVMGetTypeKind,
        LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMPositionBuilderAtEnd, LLVMSetValueName2,
        LLVMTypeOf, LLVMVoidTypeInContext, LLVMBuildCall2, LLVMGetReturnType, LLVMGetNamedFunction, LLVMConstNull, LLVMGetElementType, LLVMDumpValue, LLVMDumpType, LLVMIsAFunction, LLVMConstInt, LLVMConstStringInContext, LLVMPointerType, LLVMInt8TypeInContext, LLVMBuildGlobalStringPtr, LLVMRunFunctionPassManager,
    },
    prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef, LLVMPassManagerRef},
    LLVMTypeKind, analysis::{LLVMVerifyFunction, LLVMVerifierFailureAction},
};

use crate::scope::Scope;

pub type BinOpFn =
    unsafe extern "C" fn(LLVMBuilderRef, LLVMValueRef, LLVMValueRef, *const i8) -> LLVMValueRef;

pub struct Compiler<'a> {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    fpm: LLVMPassManagerRef,
    src: &'a cash::Header,
    inst_offset: usize,
    str_offset: usize,
    data_offset: usize,
    scope: Scope<'a>,
    funs: HashMap<&'a str, LLVMValueRef>,
}

impl<'a> Compiler<'a> {
    pub fn new(
        src: &'a cash::Header,
        ctx: LLVMContextRef,
        builder: LLVMBuilderRef,
        module: LLVMModuleRef,
        fpm: LLVMPassManagerRef,
    ) -> Self {
        Self {
            ctx,
            src,
            builder,
            module,
            fpm,
            inst_offset: 0,
            str_offset: 0,
            data_offset: 0,
            scope: Scope::new(),
            funs: HashMap::new(),
        }
    }

    pub fn compile(&mut self) {
        while let Some(inst) = self.read_inst() {
            self.compile_inst(inst);
        }
    }

    fn compile_inst(&mut self, inst: cash::Inst) -> LLVMValueRef {
        use cash::Inst;
        match inst {
            Inst::Fun {
                params_len,
                body_len,
            } => self.compile_fun(params_len as usize, body_len as usize),
            Inst::Ret => self.compile_ret(),
            Inst::I32(v) => self.compile_i32(v),
            Inst::F64(v) => self.compile_f64(v),
            Inst::String => self.compile_string(),
            Inst::Sum | Inst::Sub | Inst::Mul | Inst::Div | Inst::Rem => self.compile_bin_op(inst),
            Inst::Var => self.compile_load_var(),
            Inst::Block { len: _ } => todo!(),
            Inst::Call { arg_len } => self.compile_call(arg_len as usize),
            Inst::Eq => todo!(),
            Inst::Neq => todo!(),
            Inst::Gt => todo!(),
            Inst::Lt => todo!(),
            Inst::Gte => todo!(),
            Inst::Lte => todo!(),
            Inst::LogicAnd => todo!(),
            Inst::LogicOr => todo!(),
            Inst::Not => todo!(),
            Inst::Bool(_) => todo!(),
            Inst::Neg => todo!(),
            Inst::VarDecl(_) => todo!(),
            Inst::Assign => todo!(),
            Inst::Loop { len: _ } => todo!(),
            Inst::Repeat => todo!(),
            Inst::Branch(_, _) => todo!(),
            Inst::Break => todo!(),
            Inst::None => unreachable!(),
        }
    }

    // TODO Compile declarations first
    fn compile_fun(&mut self, params_len: usize, body_len: usize) -> LLVMValueRef {
        let name = self.read_string();
        println!("Compiling function: {name}");

        let fun_ty = self.read_type();
        let mut param_names = Vec::with_capacity(params_len as usize);
        let mut param_types = Vec::with_capacity(params_len as usize);

        // Params
        for _ in 0..params_len {
            let (param_name, param_ty) = self.read_typed_field();
            param_names.push(param_name);
            param_types.push(param_ty);
        }

        let fun = unsafe {
            let fun_ty = LLVMFunctionType(fun_ty, param_types.as_mut_ptr(), params_len as u32, 0);
            let fun = LLVMAddFunction(self.module, name.llvm_str(), fun_ty);
            if body_len != 0 {
                let block = LLVMAppendBasicBlockInContext(self.ctx, fun, RawStr(b"entry\0").llvm_str());
                LLVMPositionBuilderAtEnd(self.builder, block);
            }

            self.funs.insert(name.as_str(), fun);

            self.scope.enter();
            for (i, name) in param_names.iter().enumerate() {
                let param = LLVMGetParam(fun, i as u32);
                if body_len != 0 {
                    let ty = param_types[i];
                    let alloca = LLVMBuildAlloca(self.builder, ty, name.llvm_str());
    
                    LLVMBuildStore(self.builder, param, alloca);
                    self.scope.set_var(name.as_str(), (alloca, ty));
                }

                // Names can't contain null bytes hence len - 1
                LLVMSetValueName2(param, name.llvm_str(), name.len() - 1)
            }

            fun
        };

        for _ in 0..body_len {
            let stmt = self.read_inst().expect("expected function statement");
            self.compile_inst(stmt);
        }
        self.scope.leave();
        
        unsafe {
            LLVMVerifyFunction(fun, LLVMVerifierFailureAction::LLVMAbortProcessAction);
            LLVMRunFunctionPassManager(self.fpm, fun);
        }

        fun
    }

    fn compile_call(&mut self, arg_len: usize) -> LLVMValueRef {
        let callee = self.read_inst().unwrap();
        let callee = self.compile_inst(callee);
        let mut args = (0..arg_len).map(|_| {
            let inst = self.read_inst().expect("expected arg");
            self.compile_inst(inst)
        }).collect::<Vec<_>>();
        
        unsafe {
            LLVMBuildCall2(
                self.builder,
                LLVMGetElementType(LLVMTypeOf(callee)),
                callee,
                args.as_mut_ptr(),
                args.len() as u32,
                RawStr::null().llvm_str(),
            )
        }
     }

    fn compile_i32(&mut self, v: i32) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.ctx), v as u64, 1) }
    }
    
    fn compile_f64(&mut self, v: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMDoubleTypeInContext(self.ctx), v) }
    }

    fn compile_string(&mut self) -> LLVMValueRef {
        let v = self.read_string();
        unsafe {
            // TODO: Use String struct 
            LLVMBuildGlobalStringPtr(self.builder, v.llvm_str(), RawStr(b"strtmp\0").llvm_str())
        }
    }

    fn compile_load_var(&mut self) -> LLVMValueRef {
        let name = self.read_string();
        if let Some(value) = self.funs.get(name.as_str()) {
            return *value;
        }

        let (value, ty) = self.scope.get_var(name.as_str());
        unsafe { LLVMBuildLoad2(self.builder, ty, value, name.llvm_str()) }
    }

    fn compile_bin_op(&mut self, op: cash::Inst) -> LLVMValueRef {
        let lh = self.read_inst().expect("expected left operand");
        let rh = self.read_inst().expect("expected right operand");
        let lh = self.compile_inst(lh);
        let rh = self.compile_inst(rh);

        let bin_op = |fi: BinOpFn, ff: BinOpFn| unsafe {
            let kind = LLVMGetTypeKind(LLVMTypeOf(lh));
            match kind {
                LLVMTypeKind::LLVMDoubleTypeKind | LLVMTypeKind::LLVMFloatTypeKind => {
                    ff(self.builder, lh, rh, RawStr(b"tmp\0").llvm_str())
                }
                LLVMTypeKind::LLVMIntegerTypeKind => {
                    fi(self.builder, lh, rh, RawStr(b"tmp\0").llvm_str())
                }
                _ => unreachable!(),
            }
        };

        use cash::Inst;
        match op {
            Inst::Sum => bin_op(LLVMBuildAdd, LLVMBuildFAdd),
            Inst::Sub => bin_op(LLVMBuildSub, LLVMBuildFSub),
            Inst::Mul => bin_op(LLVMBuildMul, LLVMBuildFMul),
            Inst::Div => todo!(),
            Inst::Rem => todo!(),
            _ => unreachable!(),
        }
    }

    fn compile_ret(&mut self) -> LLVMValueRef {
        let value = self.read_inst().expect("expected ret value");
        if matches!(value, cash::Inst::None) {
            unsafe {
                return LLVMBuildRetVoid(self.builder);
            }
        }

        let value = self.compile_inst(value);
        unsafe {
            return LLVMBuildRet(self.builder, value);
        }
    }

    fn lower_type(&mut self, ty: cash::Ty) -> LLVMTypeRef {
        unsafe {
            let ty = match ty {
                cash::Ty::String => LLVMPointerType(LLVMInt8TypeInContext(self.ctx), 0),
                cash::Ty::I32 => LLVMInt32TypeInContext(self.ctx),
                cash::Ty::F64 => LLVMDoubleTypeInContext(self.ctx),
                cash::Ty::Bool => LLVMInt1TypeInContext(self.ctx),
                cash::Ty::Void => LLVMVoidTypeInContext(self.ctx),
            };

            ty.into()
        }
    }

    fn read_type(&mut self) -> LLVMTypeRef {
        let cash::Extra::Type(ty) = self.read_data();
        self.lower_type(ty)
    }

    fn read_typed_field(&mut self) -> (RawStr<'a>, LLVMTypeRef) {
        (self.read_string(), self.read_type())
    }

    fn read_inst(&mut self) -> Option<cash::Inst> {
        let inst = self.src.instructions.get(self.inst_offset).cloned();
        self.inst_offset += 1;

        inst
    }

    fn read_string(&mut self) -> RawStr<'a> {
        let mut offset = 0;
        for b in self.src.strings.iter().skip(self.str_offset) {
            offset += 1;
            if *b == '\0' as u8 {
                break;
            }
        }

        let bytes = &self.src.strings[self.str_offset..(self.str_offset + offset)];
        self.str_offset += offset;
        RawStr(bytes)
    }

    fn read_data(&mut self) -> cash::Extra {
        let data = self.src.extra.get(self.data_offset).cloned();
        self.data_offset += 1;
        data.unwrap()
    }
}

pub struct RawStr<'a>(pub &'a [u8]);

impl<'a> RawStr<'a> {
    pub fn null() -> Self {
        Self(b"\0")
    }

    pub fn as_str(&self) -> &'a str {
        std::str::from_utf8(self.0).unwrap()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub unsafe fn llvm_str(&self) -> *const c_char {
        let b = self.0.last().unwrap();
        if *b != '\0' as u8 {
            panic!("Expected null terminated string");
        }
        self.0.as_ptr() as *const _
    }
}

impl<'a> fmt::Display for RawStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
