#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclObjC.h"

namespace cxx_clang::clang::ast::decl::obj_c_protocol_decl {
CXX_MEMORY_ABI_PRELUDE(ObjCProtocolDecl, ::clang::ObjCProtocolDecl)
} // namespace cxx_clang::clang::ast::decl::obj_c_protocol_decl

namespace cxx_clang::clang::ast::decl::obj_c_protocol_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_obj_c_container_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCContainerDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_obj_c_container_decl(Self& This [[clang::lifetimebound]]) -> ::clang::ObjCContainerDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::obj_c_protocol_decl
