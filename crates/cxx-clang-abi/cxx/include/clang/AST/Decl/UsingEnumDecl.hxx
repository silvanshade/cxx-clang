#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::using_enum_decl {
CXX_MEMORY_ABI_PRELUDE(UsingEnumDecl, ::clang::UsingEnumDecl)
} // namespace cxx_clang::clang::ast::decl::using_enum_decl

namespace cxx_clang::clang::ast::decl::using_enum_decl {
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

} // namespace cxx_clang::clang::ast::decl::using_enum_decl
