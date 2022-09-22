#include "cli.h"

#include <stdexcept>
#include <string>

#include "compiler.h"

void Build(const char* path);

void ParseArgs(int argc, char** argv) {
    bool show_help = false;
    if (argc >= 2) {
        if (std::strcmp(argv[1], "build") == 0) {
            if (argc < 3) {
                throw std::runtime_error("Expected path to .cash file");
            }

            Build(argv[2]);
        } else {
            show_help = true;
        }
    } else {
        show_help = true;
    }
}

void Build(const char* path) {
    Compiler compiler(path);
    compiler.Build();
}