#include "cash.h"

#include <stdexcept>

Header::Header(const std::vector<uint8_t>& bytes) {
    CashReader reader(bytes);
    Header reader = reader.Read();
}

CashReader::CashReader(const std::vector<uint8_t>& bytes) : bytes_(bytes), offset_(0) {}

Header CashReader::Read() {
    auto version = ReadVersion();
    auto instructions = ReadInstructions();
}

std::array<uint8_t, 3> CashReader::ReadVersion() {
    std::array<uint8_t, 3> version;
    version[0] = ReadU8();
    version[1] = ReadU8();
    version[2] = ReadU8();
}

std::vector<Inst> CashReader::ReadInstructions() {
    size_t len = static_cast<size_t>(ReadU64());
    std::vector<Inst> instructions;
    instructions.reserve(len);

    for (size_t i = 0; i < len; ++i) {
       instructions.push_back(ReadInst());
    }
    
    return instructions;
}

Inst CashReader::ReadInst() {
    switch (ReadU32()) {
    case Inst::FUN:
        return ReadFun();
    case Inst::CALL:
        return ReadCall();
    case Inst::BLOCK:
        return ReadBlock();
    case Inst::VAR:
        return ReadSimple(Inst::VAR);
    case Inst::SUM:
        return ReadSimple(Inst::SUM);
    case Inst::SUB:
        return ReadSimple(Inst::SUB);
    case Inst::MUL:
        return ReadSimple(Inst::MUL);
    case Inst::DIV:
        return ReadSimple(Inst::DIV);
    case Inst::REM:
        return ReadSimple(Inst::REM);
    case Inst::EQ:
        return ReadSimple(Inst::EQ);
    case Inst::NEQ:
        return ReadSimple(Inst::NEQ);
    case Inst::GT:
        return ReadSimple(Inst::GT);
    case Inst::LT:
        return ReadSimple(Inst::LT);
    case Inst::GTE:
        return ReadSimple(Inst::GTE);
    case Inst::LTE:
        return ReadSimple(Inst::LTE);
    case Inst::LOGIC_AND:
        return ReadSimple(Inst::LOGIC_AND);
    case Inst::LOGIC_OR:
        return ReadSimple(Inst::LOGIC_OR);
    case Inst::NOT:
        return ReadSimple(Inst::NOT);
    case Inst::NEG:
        return ReadSimple(Inst::NEG);
    default:
        throw std::runtime_error("Not implemented or invalid instruction");
    }
}

Inst CashReader::ReadFun() {
    Inst::Fun fun;
    fun.params_len = ReadU8();
    fun.body_len = ReadU32();

    Inst inst;
    inst.ty_ = Inst::FUN;
    inst.data_.fun_data = fun;

    return inst;
}

Inst CashReader::ReadCall() {
    Inst::Call call;
    call.args_len = ReadU8();

    Inst inst;
    inst.ty_ = Inst::CALL;
    inst.data_.call_data = call;

    return inst;
}

Inst CashReader::ReadBlock() {
    Inst::Block block;
    block.len = ReadU32();

    Inst inst;
    inst.ty_ = Inst::BLOCK;
    inst.data_.block_data = block;

    return inst;
}

Inst CashReader::ReadSimple(Inst::InstTy ty) {
    Inst inst;
    inst.ty_ = ty;

    return inst;
}

uint8_t CashReader::ReadU8() {
    return bytes_[offset_++];
}

uint32_t CashReader::ReadU32() {
    uint8_t b1 = ReadU8();
    uint8_t b2 = ReadU8();
    uint8_t b3 = ReadU8();
    uint8_t b4 = ReadU8();
    return b1 | (b2 << 8) | (b3 << 16) | (b4 << 24);
}

uint64_t CashReader::ReadU64() {
    uint8_t b1 = ReadU8();
    uint8_t b2 = ReadU8();
    uint8_t b3 = ReadU8();
    uint8_t b4 = ReadU8();
    uint8_t b5 = ReadU8();
    uint8_t b6 = ReadU8();
    uint8_t b7 = ReadU8();
    uint8_t b8 = ReadU8();
    return b1 | (b2 << 8) | (b3 << 16) | (b4 << 24) | (b5 << 32) || (b6 << 40) || (b7 << 48) || (b8 << 56);
}