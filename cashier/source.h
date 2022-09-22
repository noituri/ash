#pragma once

#include <vector>
#include <string>

class Source {
public:
    Source(const char* path);

private:
    std::string path_;
    std::vector<uint8_t> data_;    
};