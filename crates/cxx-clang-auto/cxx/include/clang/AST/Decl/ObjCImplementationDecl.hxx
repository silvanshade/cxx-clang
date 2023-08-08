#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_implementation_decl {
CXX_AUTO_PRELUDE(ObjCImplementationDecl, ::clang::ObjCImplementationDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_implementation_decl

namespace cxx_clang::clang::ast::decl::obj_c_implementation_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_obj_c_impl_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCImplDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_obj_c_impl_decl(Self& This [[clang::lifetimebound]]) -> ::clang::ObjCImplDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::obj_c_implementation_decl
