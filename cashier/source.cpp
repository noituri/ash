#include "source.h"

#include <fstream>
#include <iterator>
#include <iostream>
#include <stdexcept>

Source::Source(const char* path) : path_(path) {
    std::fstream file(path);
    if (!file) {
        throw std::runtime_error(std::string("File ") + path + " not found");
    }

    file.seekg(0, std::ios::end);
    size_t size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<uint8_t> bytes;
    bytes.reserve(size);
    bytes.assign(
        std::istreambuf_iterator<char>(file),
        std::istreambuf_iterator<char>()
    ); 

    file.close();

    data_ = Header(bytes);

    std::cout << "File " << path << " loaded. Size: " << size << std::endl;  
}
