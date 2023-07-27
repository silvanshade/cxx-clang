#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::redeclarable_template_decl {
CXX_MEMORY_ABI_PRELUDE(RedeclarableTemplateDecl, ::clang::RedeclarableTemplateDecl)
} // namespace cxx_clang::clang::ast::decl::redeclarable_template_decl

namespace cxx_clang::clang::ast::decl::redeclarable_template_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TemplateDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_template_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TemplateDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::redeclarable_template_decl
