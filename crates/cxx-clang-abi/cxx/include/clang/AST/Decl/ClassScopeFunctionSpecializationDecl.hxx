#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclTemplate.h"

namespace cxx_clang::clang::ast::decl::class_scope_function_specialization_decl {
CXX_MEMORY_ABI_PRELUDE(ClassScopeFunctionSpecializationDecl, ::clang::ClassScopeFunctionSpecializationDecl)
} // namespace cxx_clang::clang::ast::decl::class_scope_function_specialization_decl

namespace cxx_clang::clang::ast::decl::class_scope_function_specialization_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::Decl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl(Self& This [[clang::lifetimebound]]) -> ::clang::Decl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::class_scope_function_specialization_decl
