#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::namespace_alias_decl {
CXX_AUTO_PRELUDE(NamespaceAliasDecl, ::clang::NamespaceAliasDecl)
} // namespace cxx_clang::clang::ast::decl::namespace_alias_decl

namespace cxx_clang::clang::ast::decl::namespace_alias_decl {
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

} // namespace cxx_clang::clang::ast::decl::namespace_alias_decl
