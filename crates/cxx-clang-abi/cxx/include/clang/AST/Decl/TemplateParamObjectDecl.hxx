#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::template_param_object_decl {
CXX_MEMORY_ABI_PRELUDE(TemplateParamObjectDecl, ::clang::TemplateParamObjectDecl)
} // namespace cxx_clang::clang::ast::decl::template_param_object_decl

namespace cxx_clang::clang::ast::decl::template_param_object_decl {
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

} // namespace cxx_clang::clang::ast::decl::template_param_object_decl
