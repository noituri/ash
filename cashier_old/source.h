#pragma once

#include <vector>
#include <string>

#include "cash.h"

class Source {
public:
    Source(const char* path);

private:
    std::string path_;
    Header data_;
};