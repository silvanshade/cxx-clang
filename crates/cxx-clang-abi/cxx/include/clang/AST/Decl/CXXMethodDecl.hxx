#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::cxx_method_decl {
CXX_MEMORY_ABI_PRELUDE(CXXMethodDecl, ::clang::CXXMethodDecl)
} // namespace cxx_clang::clang::ast::decl::cxx_method_decl

namespace cxx_clang::clang::ast::decl::cxx_method_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_function_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FunctionDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_function_decl(Self& This [[clang::lifetimebound]]) -> ::clang::FunctionDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::cxx_method_decl
