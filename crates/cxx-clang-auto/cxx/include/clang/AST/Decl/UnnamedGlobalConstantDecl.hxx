#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::unnamed_global_constant_decl {
CXX_AUTO_PRELUDE(UnnamedGlobalConstantDecl, ::clang::UnnamedGlobalConstantDecl)
} // namespace cxx_clang::clang::ast::decl::unnamed_global_constant_decl

namespace cxx_clang::clang::ast::decl::unnamed_global_constant_decl {
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

} // namespace cxx_clang::clang::ast::decl::unnamed_global_constant_decl
