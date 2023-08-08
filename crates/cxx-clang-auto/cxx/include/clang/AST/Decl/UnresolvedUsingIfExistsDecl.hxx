#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl {
CXX_AUTO_PRELUDE(UnresolvedUsingIfExistsDecl, ::clang::UnresolvedUsingIfExistsDecl)
} // namespace cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl

namespace cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl {
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

} // namespace cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl
