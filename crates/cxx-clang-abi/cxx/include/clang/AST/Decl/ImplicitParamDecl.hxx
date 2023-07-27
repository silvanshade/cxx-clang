#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::implicit_param_decl {
CXX_MEMORY_ABI_PRELUDE(ImplicitParamDecl, ::clang::ImplicitParamDecl)
} // namespace cxx_clang::clang::ast::decl::implicit_param_decl

namespace cxx_clang::clang::ast::decl::implicit_param_decl {
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

} // namespace cxx_clang::clang::ast::decl::implicit_param_decl
