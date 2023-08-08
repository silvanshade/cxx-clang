#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::class_template_specialization_decl {
CXX_AUTO_PRELUDE(ClassTemplateSpecializationDecl, ::clang::ClassTemplateSpecializationDecl)
} // namespace cxx_clang::clang::ast::decl::class_template_specialization_decl

namespace cxx_clang::clang::ast::decl::class_template_specialization_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_cxx_record_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXRecordDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_cxx_record_decl(Self& This [[clang::lifetimebound]]) -> ::clang::CXXRecordDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::class_template_specialization_decl
