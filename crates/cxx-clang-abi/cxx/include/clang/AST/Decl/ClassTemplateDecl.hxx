#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::class_template_decl {
CXX_MEMORY_ABI_PRELUDE(ClassTemplateDecl, ::clang::ClassTemplateDecl)
} // namespace cxx_clang::clang::ast::decl::class_template_decl

namespace cxx_clang::clang::ast::decl::class_template_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_redeclarable_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::RedeclarableTemplateDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_redeclarable_template_decl(Self& This [[clang::lifetimebound]]) -> ::clang::RedeclarableTemplateDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::class_template_decl
