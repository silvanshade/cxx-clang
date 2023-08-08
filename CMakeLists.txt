# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-clang LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# ensure that the C++ compiler is `clang++`
set(CMAKE_CXX_COMPILER "clang++")

add_library(cxx-clang-auto STATIC
  crates/cxx-clang-auto/cxx/lib/cmake.cxx
)
target_include_directories(cxx-clang-auto PUBLIC
  ../cxx-clang/crates/cxx-clang-auto/..
  ../cxx-llvm/crates/cxx-llvm-auto/..
  ../cxx-auto/..
  target/cxxbridge
  ../swift-project/build/Ninja-MinSizeRelAssert/llvm-linux-x86_64/include
  ../swift-project/build/Ninja-MinSizeRelAssert/llvm-linux-x86_64/tools/clang/include
  ../swift-project/llvm-project/clang/include
  ../swift-project/llvm-project/llvm/include
)
target_compile_definitions(cxx-clang-auto PUBLIC _LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS)
target_compile_options(cxx-clang-auto PUBLIC
  -std=gnu++20
  # -stdlib=libc++
  -Werror
  -Wall
  -Wextra
  -Wthread-safety
  -Wthread-safety-beta
  -pedantic
  -Wno-ambiguous-reversed-operator
  -Wno-deprecated-anon-enum-enum-conversion
  -Wno-deprecated-builtins
  -Wno-dollar-in-identifier-extension
  -Wno-nested-anon-types
  -Wno-unused-parameter
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
)
