#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::template_type_parm_decl {
CXX_MEMORY_ABI_PRELUDE(TemplateTypeParmDecl, ::clang::TemplateTypeParmDecl)
} // namespace cxx_clang::clang::ast::decl::template_type_parm_decl

namespace cxx_clang::clang::ast::decl::template_type_parm_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_type_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_type_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TypeDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::template_type_parm_decl
