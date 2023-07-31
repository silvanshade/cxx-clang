#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_clang::clang::ast::decl::omp_declare_reduction_decl {
CXX_MEMORY_ABI_PRELUDE(OMPDeclareReductionDecl, ::clang::OMPDeclareReductionDecl)
} // namespace cxx_clang::clang::ast::decl::omp_declare_reduction_decl

namespace cxx_clang::clang::ast::decl::omp_declare_reduction_decl {
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

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl_context(Self const& This [[clang::lifetimebound]]) -> ::clang::DeclContext const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl_context(Self& This [[clang::lifetimebound]]) -> ::clang::DeclContext&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::omp_declare_reduction_decl
