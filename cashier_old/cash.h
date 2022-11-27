#pragma once

#include <cstdint>
#include <vector>
#include <array>

enum Ty {
    String,
    I32,
    F64,
    BOOL,
    VOID,
};

class Inst {
public:
    // 4 bytes INST
    enum InstTy {
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
        Branch branch_data;
    };

    InstTy ty_;
    Data data_;
};

class Extra {
public:
    enum ExtraTy {
        TYPED_FIELD,
        TYPE,
    };

    union Data {
        Ty ty_data;
    };

    ExtraTy ty_;
    Data data_;
};

class Header {
public:
    Header(const std::vector<uint8_t>& bytes);

    std::array<uint8_t, 3> version_;
    std::vector<Inst> instructions_;
    std::vector<char> strings_;
    std::vector<Extra> extra_; 
};

class CashReader {
public:
    CashReader(const std::vector<uint8_t>& bytes);
    
    Header Read();

private:
    size_t offset_;
    const std::vector<uint8_t>& bytes_;

    std::array<uint8_t, 3> ReadVersion();

    std::vector<Inst> ReadInstructions();

    Inst ReadInst();

    Inst ReadFun();

    Inst ReadCall();

    Inst ReadBlock();

    Inst ReadSimple(Inst::InstTy ty);

    uint8_t ReadU8();

    uint32_t ReadU32();

    uint64_t ReadU64();
};