#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::using_directive_decl {
CXX_MEMORY_ABI_PRELUDE(UsingDirectiveDecl, ::clang::UsingDirectiveDecl)
} // namespace cxx_clang::clang::ast::decl::using_directive_decl

namespace cxx_clang::clang::ast::decl::using_directive_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_named_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamedDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_named_decl(Self& This [[clang::lifetimebound]]) -> ::clang::NamedDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::using_directive_decl
