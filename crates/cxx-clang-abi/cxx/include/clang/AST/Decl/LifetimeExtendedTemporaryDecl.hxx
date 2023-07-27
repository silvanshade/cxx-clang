#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl {
CXX_MEMORY_ABI_PRELUDE(LifetimeExtendedTemporaryDecl, ::clang::LifetimeExtendedTemporaryDecl)
} // namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl

namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl {
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

} // namespace cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl
