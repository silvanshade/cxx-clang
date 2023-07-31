#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_clang::clang::ast::decl::omp_declarative_directive_decl {
CXX_MEMORY_ABI_PRELUDE(OMPDeclarativeDirectiveDecl, ::clang::OMPDeclarativeDirective, ::clang::Decl)
} // namespace cxx_clang::clang::ast::decl::omp_declarative_directive_decl

namespace cxx_clang::clang::ast::decl::omp_declarative_directive_decl {
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

} // namespace cxx_clang::clang::ast::decl::omp_declarative_directive_decl
