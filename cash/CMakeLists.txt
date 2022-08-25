cmake_minimum_required(VERSION 3.23)
project(cash)

set(CMAKE_CXX_STANDARD 20)
find_package(LLVM REQUIRED CONFIG)

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

include_directories(${LLVM_INCLUDE_DIRS})
separate_arguments(LLVM_DEFINITIONS_LIST NATIVE_COMMAND ${LLVM_DEFINITIONS})
add_definitions(${LLVM_DEFINITIONS_LIST})


add_executable(cash main.cpp compiler.h compiler.cpp chunk.h chunk.cpp)

llvm_map_components_to_libnames(llvm_libs support core irreader)
target_link_libraries(cash ${llvm_libs})
