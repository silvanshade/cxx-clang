#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_ivar_decl {
CXX_AUTO_PRELUDE(ObjCIvarDecl, ::clang::ObjCIvarDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_ivar_decl

namespace cxx_clang::clang::ast::decl::obj_c_ivar_decl {
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

} // namespace cxx_clang::clang::ast::decl::obj_c_ivar_decl
