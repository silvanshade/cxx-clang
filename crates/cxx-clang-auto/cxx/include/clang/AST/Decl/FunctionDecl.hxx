#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::function_decl {
CXX_AUTO_PRELUDE(FunctionDecl, ::clang::FunctionDecl)
} // namespace cxx_clang::clang::ast::decl::function_decl

namespace cxx_clang::clang::ast::decl::function_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl_context(Self const& This [[clang::lifetimebound]]) -> ::clang::DeclContext const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl_context(Self& This [[clang::lifetimebound]]) -> ::clang::DeclContext&
{
  return This;
}

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

} // namespace cxx_clang::clang::ast::decl::function_decl
