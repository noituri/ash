//
// Created by noituri on 2022/08/24.
//

#include "chunk.h"

Chunk::Chunk(std::vector<int32_t> constants, std::vector<uint8_t> code) : code_(code), constants_(constants) {}