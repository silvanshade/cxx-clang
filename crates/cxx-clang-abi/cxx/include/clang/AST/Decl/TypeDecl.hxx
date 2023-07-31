#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::type_decl {
CXX_MEMORY_ABI_PRELUDE(TypeDecl, ::clang::TypeDecl)
} // namespace cxx_clang::clang::ast::decl::type_decl

namespace cxx_clang::clang::ast::decl::type_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_named_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamedDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_named_decl(Self& This [[clang::lifetimebound]]) -> ::clang::NamedDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::type_decl
