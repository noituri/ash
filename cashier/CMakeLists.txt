cmake_minimum_required(VERSION 3.20)
project(cashier)

set(CMAKE_CXX_STANDARD 20)
find_package(LLVM REQUIRED CONFIG)
find_package(ZLIB REQUIRED)

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

include_directories(${LLVM_INCLUDE_DIRS})
separate_arguments(LLVM_DEFINITIONS_LIST NATIVE_COMMAND ${LLVM_DEFINITIONS})
add_definitions(${LLVM_DEFINITIONS_LIST})


add_executable(cashier main.cpp
    source.h source.cpp
    cli.h cli.cpp
    compiler.h compiler.cpp
    cash.h cash.cpp
)

llvm_map_components_to_libnames(llvm_libs support core irreader)
target_link_libraries(cashier ${llvm_libs})
# target_link_libraries(cashier PRIVATE ZLIB::ZLIB)
