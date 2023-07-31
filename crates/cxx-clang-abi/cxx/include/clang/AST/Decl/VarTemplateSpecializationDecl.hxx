#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::var_template_specialization_decl {
CXX_MEMORY_ABI_PRELUDE(VarTemplateSpecializationDecl, ::clang::VarTemplateSpecializationDecl)
} // namespace cxx_clang::clang::ast::decl::var_template_specialization_decl

namespace cxx_clang::clang::ast::decl::var_template_specialization_decl {
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

} // namespace cxx_clang::clang::ast::decl::var_template_specialization_decl
