#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_property_impl_decl {
CXX_AUTO_PRELUDE(ObjCPropertyImplDecl, ::clang::ObjCPropertyImplDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_property_impl_decl

namespace cxx_clang::clang::ast::decl::obj_c_property_impl_decl {
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

} // namespace cxx_clang::clang::ast::decl::obj_c_property_impl_decl
