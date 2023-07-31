#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::extern_c_context_decl {
CXX_MEMORY_ABI_PRELUDE(ExternCContextDecl, ::clang::ExternCContextDecl)
} // namespace cxx_clang::clang::ast::decl::extern_c_context_decl

namespace cxx_clang::clang::ast::decl::extern_c_context_decl {
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

} // namespace cxx_clang::clang::ast::decl::extern_c_context_decl
