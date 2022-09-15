#include "compiler.h"

#include <iostream>
#include <stdexcept>

Compiler::Compiler(const Chunk &chunk) : chunk_(chunk) {}

void Compiler::run() {
    std::unique_ptr<llvm::Module> main_module = std::make_unique<llvm::Module>("ash_main", llvm_context);
    std::vector<llvm::Type*> params;
    llvm::FunctionType* function_ty = llvm::FunctionType::get(llvm::Type::getInt32Ty(llvm_context), params, false);
    llvm::Function* function = llvm::Function::Create(function_ty, llvm::GlobalValue::ExternalLinkage, "main", main_module.get());
    llvm::BasicBlock* block = llvm::BasicBlock::Create(llvm_context, "entry", function);
    builder.SetInsertPoint(block);

    compile();

    llvm::verifyFunction(*function, &llvm::errs());
    main_module->print(llvm::errs(), nullptr);
}

void Compiler::compile() {
    while (true) {
        const uint8_t instr = read_byte();
        switch (instr) {
            case 0: {
                llvm::Value* v = pop();
                builder.CreateRet(v);
                return;
            }
            case 1: {
                int32_t c = read_const();
                push(llvm::ConstantInt::getSigned(llvm::Type::getInt32Ty(llvm_context), c));
                break;
            }
            case 4: {
                llvm::Value* b = pop();
                llvm::Value* a = pop();
                push(builder.CreateAdd(a, b, "tmp_add"));
                break;
            }
            default:
                throw std::runtime_error("unknown instruction");
        }
    }
}

uint8_t Compiler::read_byte() {
    return chunk_.code_[offset_++];
}

int32_t Compiler::read_const() {
    return chunk_.constants_[read_byte()];
}

void Compiler::push(llvm::Value* value) {
    stack_.push_back(value);
}

llvm::Value* Compiler::pop() {
    llvm::Value* value = stack_.back();
    stack_.pop_back();
    return value;
}