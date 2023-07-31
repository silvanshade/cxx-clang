#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_type_param_decl {
CXX_MEMORY_ABI_PRELUDE(ObjCTypeParamDecl, ::clang::ObjCTypeParamDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_type_param_decl

namespace cxx_clang::clang::ast::decl::obj_c_type_param_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_typedef_name_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_typedef_name_decl(Self& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::obj_c_type_param_decl
