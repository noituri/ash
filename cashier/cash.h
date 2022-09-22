#pragma once

#include <cstdint>

class Cash {
public:
    enum Inst {
        NONE,
        FUN,
        CALL,
        BLOCK,
        VAR,
        SUM,
        SUB,
        MUL,
        DIV,
        REM,
        EQ,
        NEQ,
        GT,
        LT,
        GTE,
        LTE,
        LOGIC_AND,
        LOGIC_OR,
        NOT,
        NEG,
        I32,
        F64,
        BOOL,
        STRING,
        RET,
        VAR_DECL,
        ASSIGN,
        LOOP,
        REPEAT,
        BRANCH,
        BREAK
    };

    enum Ty {
        String,
        I32,
        F64,
        BOOL,
        VOID,
    };

    struct Fun {
        uint8_t params_len;
        uint32_t body_len;
    };

    struct Call {
        uint8_t args_len;
    };

    struct Block {
        uint32_t len;
    };

    struct VarDecl {
        Ty ty;
    };

    struct Loop {
        uint32_t len;
    };

    struct Branch {
        uint32_t then_len;
        uint32_t else_len;
    };
    
    union Data {
        Fun fun_data;
        Call call_data;
        Block block_data;
        int32_t i32_data;
        double f64_data;
        bool bool_data;
        VarDecl var_decl_data;        
        Loop loop_data;
        Branch branch_data;ł
    };
};
