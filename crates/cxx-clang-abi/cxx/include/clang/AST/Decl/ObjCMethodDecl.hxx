#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_method_decl {
CXX_MEMORY_ABI_PRELUDE(ObjCMethodDecl, ::clang::ObjCMethodDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_method_decl

namespace cxx_clang::clang::ast::decl::obj_c_method_decl {
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

} // namespace cxx_clang::clang::ast::decl::obj_c_method_decl
