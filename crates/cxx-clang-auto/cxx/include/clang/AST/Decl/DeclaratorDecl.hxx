#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::declarator_decl {
CXX_AUTO_PRELUDE(DeclaratorDecl, ::clang::DeclaratorDecl)
} // namespace cxx_clang::clang::ast::decl::declarator_decl

namespace cxx_clang::clang::ast::decl::declarator_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_value_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ValueDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_value_decl(Self& This [[clang::lifetimebound]]) -> ::clang::ValueDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::declarator_decl
