#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::constructor_using_shadow_decl {
CXX_MEMORY_ABI_PRELUDE(ConstructorUsingShadowDecl, ::clang::ConstructorUsingShadowDecl)
} // namespace cxx_clang::clang::ast::decl::constructor_using_shadow_decl

namespace cxx_clang::clang::ast::decl::constructor_using_shadow_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_using_shadow_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingShadowDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_using_shadow_decl(Self& This [[clang::lifetimebound]]) -> ::clang::UsingShadowDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::constructor_using_shadow_decl
