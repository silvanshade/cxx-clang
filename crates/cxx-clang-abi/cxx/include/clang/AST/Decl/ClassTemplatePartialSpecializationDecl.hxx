#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::class_template_partial_specialization_decl {
CXX_MEMORY_ABI_PRELUDE(ClassTemplatePartialSpecializationDecl, ::clang::ClassTemplatePartialSpecializationDecl)
} // namespace cxx_clang::clang::ast::decl::class_template_partial_specialization_decl

namespace cxx_clang::clang::ast::decl::class_template_partial_specialization_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_class_template_specialization_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ClassTemplateSpecializationDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_class_template_specialization_decl(Self& This [[clang::lifetimebound]])
  -> ::clang::ClassTemplateSpecializationDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::class_template_partial_specialization_decl
