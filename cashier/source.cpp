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
    
    data_.reserve(size);
    data_.assign(
        std::istreambuf_iterator<char>(file),
        std::istreambuf_iterator<char>()
    ); 

    file.close();

    std::cout << "File " << path << " loaded. Size: " << size << std::endl;  
}