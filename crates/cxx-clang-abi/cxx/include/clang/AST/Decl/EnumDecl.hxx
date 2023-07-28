#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::enum_decl {
CXX_MEMORY_ABI_PRELUDE(EnumDecl, ::clang::EnumDecl)
} // namespace cxx_clang::clang::ast::decl::enum_decl

namespace cxx_clang::clang::ast::decl::enum_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_tag_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TagDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_tag_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TagDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::enum_decl