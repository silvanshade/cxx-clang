#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_clang::clang::ast::decl::omp_captured_expr_decl {
CXX_MEMORY_ABI_PRELUDE(OMPCapturedExprDecl, ::clang::OMPCapturedExprDecl)
} // namespace cxx_clang::clang::ast::decl::omp_captured_expr_decl

namespace cxx_clang::clang::ast::decl::omp_captured_expr_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_var_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::VarDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_var_decl(Self& This [[clang::lifetimebound]]) -> ::clang::VarDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::omp_captured_expr_decl
