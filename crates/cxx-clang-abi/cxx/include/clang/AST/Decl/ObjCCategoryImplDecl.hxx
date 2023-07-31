#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_category_impl_decl {
CXX_MEMORY_ABI_PRELUDE(ObjCCategoryImplDecl, ::clang::ObjCCategoryImplDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_category_impl_decl

namespace cxx_clang::clang::ast::decl::obj_c_category_impl_decl {
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

} // namespace cxx_clang::clang::ast::decl::obj_c_category_impl_decl
