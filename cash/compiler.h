#ifndef CASH_COMPILER_H
#define CASH_COMPILER_H

#include "chunk.h"
#include <llvm/IR/Value.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Verifier.h>

static llvm::LLVMContext llvm_context;
static llvm::IRBuilder<> builder(llvm_context);

class Compiler {
public:
    Compiler(const Chunk& chunk);
    void run();

private:
    uint8_t read_byte();
    void push(llvm::Value* value);
    llvm::Value* pop();
    int32_t read_const();
    void compile();

    const Chunk& chunk_;
    size_t offset_ = 0;
    std::vector<llvm::Value*> stack_;
};


#endif //CASH_COMPILER_H
