#include <iostream>
#include <memory>

#include "chunk.h"
#include "compiler.h"

int main() {
    // TEST CODE
    std::vector<int32_t> constants{3, 4, 5};
    std::vector<uint8_t> code{
        1, 0,
        1, 1,
        4,
        1, 2,
        4,
        0,
    };
    Chunk chunk(constants, code);
    Compiler compiler(chunk);
    compiler.run();

    return 0;
}
