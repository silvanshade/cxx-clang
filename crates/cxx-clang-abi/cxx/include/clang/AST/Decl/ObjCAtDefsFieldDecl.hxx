#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl {
CXX_MEMORY_ABI_PRELUDE(ObjCAtDefsFieldDecl, ::clang::ObjCAtDefsFieldDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl

namespace cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_field_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FieldDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_field_decl(Self& This [[clang::lifetimebound]]) -> ::clang::FieldDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl
