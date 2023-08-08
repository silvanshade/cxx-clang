#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::unresolved_using_typename_decl {
CXX_AUTO_PRELUDE(UnresolvedUsingTypenameDecl, ::clang::UnresolvedUsingTypenameDecl)
} // namespace cxx_clang::clang::ast::decl::unresolved_using_typename_decl

namespace cxx_clang::clang::ast::decl::unresolved_using_typename_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_type_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_type_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TypeDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::unresolved_using_typename_decl
