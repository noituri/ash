#ifndef CASH_CHUNK_H
#define CASH_CHUNK_H

#include <vector>

struct Chunk {
    Chunk(std::vector<int32_t> constants, std::vector<uint8_t> code);

    std::vector<int32_t> constants_;
    std::vector<uint8_t> code_;
};


#endif //CASH_CHUNK_H
