#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl {
CXX_AUTO_PRELUDE(LifetimeExtendedTemporaryDecl, ::clang::LifetimeExtendedTemporaryDecl)
} // namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl

namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::Decl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl(Self& This [[clang::lifetimebound]]) -> ::clang::Decl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl
