#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::enum_constant_decl {
CXX_MEMORY_ABI_PRELUDE(EnumConstantDecl, ::clang::EnumConstantDecl)
} // namespace cxx_clang::clang::ast::decl::enum_constant_decl

namespace cxx_clang::clang::ast::decl::enum_constant_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_value_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ValueDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_value_decl(Self& This [[clang::lifetimebound]]) -> ::clang::ValueDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::enum_constant_decl
