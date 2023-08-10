# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-clang LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# ensure that the C++ compiler is `clang++`
if(APPLE)
  set(CLANG_CXX_COMPILER "/opt/homebrew/opt/llvm@16/bin/clang++")
  if(NOT EXISTS ${CLANG_CXX_COMPILER})
    message(FATAL_ERROR "Could not find clang++-16`")
  endif()
  set(CMAKE_CXX_COMPILER ${CLANG_CXX_COMPILER})
elseif(LINUX)
  find_program(CLANG_CXX_COMPILER clang++-16)
  if(NOT CLANG_CXX_COMPILER)
    message(FATAL_ERROR "Could not find ${CLANG_CXX_COMPILER}`")
  endif()
  set(CMAKE_CXX_COMPILER ${CLANG_CXX_COMPILER})
else()
  message(FATAL_ERROR "unsupported platform")
endif()

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
