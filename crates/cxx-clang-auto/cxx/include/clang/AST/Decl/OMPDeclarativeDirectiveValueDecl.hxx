#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl {
CXX_AUTO_PRELUDE(OMPDeclarativeDirectiveValueDecl, ::clang::OMPDeclarativeDirective, ::clang::ValueDecl)
} // namespace cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl

namespace cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl {
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

} // namespace cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl
