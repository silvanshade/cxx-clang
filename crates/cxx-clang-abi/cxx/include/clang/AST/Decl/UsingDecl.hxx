#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::using_decl {
CXX_MEMORY_ABI_PRELUDE(UsingDecl, ::clang::UsingDecl)
} // namespace cxx_clang::clang::ast::decl::using_decl

namespace cxx_clang::clang::ast::decl::using_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_base_using_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::BaseUsingDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_base_using_decl(Self& This [[clang::lifetimebound]]) -> ::clang::BaseUsingDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::using_decl
