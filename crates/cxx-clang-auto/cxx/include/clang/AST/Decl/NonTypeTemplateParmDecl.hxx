#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::non_type_template_parm_decl {
CXX_AUTO_PRELUDE(NonTypeTemplateParmDecl, ::clang::NonTypeTemplateParmDecl)
} // namespace cxx_clang::clang::ast::decl::non_type_template_parm_decl

namespace cxx_clang::clang::ast::decl::non_type_template_parm_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_declarator_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::DeclaratorDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_declarator_decl(Self& This [[clang::lifetimebound]]) -> ::clang::DeclaratorDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::non_type_template_parm_decl
