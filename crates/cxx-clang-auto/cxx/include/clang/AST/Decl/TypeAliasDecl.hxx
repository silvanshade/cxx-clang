#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::type_alias_decl {
CXX_AUTO_PRELUDE(TypeAliasDecl, ::clang::TypeAliasDecl)
} // namespace cxx_clang::clang::ast::decl::type_alias_decl

namespace cxx_clang::clang::ast::decl::type_alias_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_typedef_name_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_typedef_name_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::type_alias_decl
