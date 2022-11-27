#include "cli.h"

#include <iostream>
#include <stdexcept>

int main(int argc, char** argv) {
    try {
        ParseArgs(argc, argv);
    } catch (std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
        
    return 0;
}
