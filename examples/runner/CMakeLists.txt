cmake_minimum_required(VERSION 3.29)
project(runner)

set(CMAKE_CXX_STANDARD 20)

# Add parent directory to include paths
include_directories(${CMAKE_SOURCE_DIR}/../..)

# Specify the library path (where your Rust library is)
link_directories(${CMAKE_SOURCE_DIR}/../../target/release)

add_executable(runner main.cpp)

target_link_libraries(runner tiny_skia_c)
