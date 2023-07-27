#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclObjC.h"
#include "llvm/Support/Casting.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using DeclKind = ::clang::Decl::Kind;

namespace cxx_clang::clang::ast::decl {
CXX_MEMORY_ABI_PRELUDE(Decl, ::clang::Decl)
} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_kind(Self const& This [[clang::lifetimebound]]) -> ::clang::Decl::Kind
{
  return This.getKind();
}

} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_field_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FieldDecl const*
{
  return llvm::dyn_cast<::clang::FieldDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_function_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FunctionDecl const*
{
  return llvm::dyn_cast<::clang::FunctionDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_named_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamedDecl const*
{
  return llvm::dyn_cast<::clang::NamedDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_interface_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCInterfaceDecl const*
{
  return llvm::dyn_cast<::clang::ObjCInterfaceDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_method_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCMethodDecl const*
{
  return llvm::dyn_cast<::clang::ObjCMethodDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_protocol_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCProtocolDecl const*
{
  return llvm::dyn_cast<::clang::ObjCProtocolDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_record_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::RecordDecl const*
{
  return llvm::dyn_cast<::clang::RecordDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_type_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeDecl const*
{
  return llvm::dyn_cast<::clang::TypeDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_name_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl const*
{
  return llvm::dyn_cast<::clang::TypedefNameDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefDecl const*
{
  return llvm::dyn_cast<::clang::TypedefDecl const>(&This);
}

} // namespace cxx_clang::clang::ast::decl
