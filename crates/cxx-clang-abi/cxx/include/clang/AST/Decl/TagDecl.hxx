#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::tag_decl {
CXX_MEMORY_ABI_PRELUDE(TagDecl, ::clang::TagDecl)
} // namespace cxx_clang::clang::ast::decl::tag_decl

namespace cxx_clang::clang::ast::decl::tag_decl {
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

} // namespace cxx_clang::clang::ast::decl::tag_decl
