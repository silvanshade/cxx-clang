#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclObjC.h"
#include "llvm/Support/Casting.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using DeclKind = ::clang::Decl::Kind;

namespace cxx_clang::clang::ast::decl {
using Decl = ::clang::Decl;
using F = Decl;

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_align<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_size<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_default_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_copy_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_move_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_destructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_copyable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_movable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_destructible<F>();
}

} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_kind(F const& This [[clang::lifetimebound]]) -> ::clang::Decl::Kind
{
  return This.getKind();
}

} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_field_decl(F const& This [[clang::lifetimebound]]) -> ::clang::FieldDecl const*
{
  return llvm::dyn_cast<::clang::FieldDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_function_decl(F const& This [[clang::lifetimebound]]) -> ::clang::FunctionDecl const*
{
  return llvm::dyn_cast<::clang::FunctionDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_named_decl(F const& This [[clang::lifetimebound]]) -> ::clang::NamedDecl const*
{
  return llvm::dyn_cast<::clang::NamedDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_objc_interface_decl(F const& This [[clang::lifetimebound]]) -> ::clang::ObjCInterfaceDecl const*
{
  return llvm::dyn_cast<::clang::ObjCInterfaceDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_objc_method_decl(F const& This [[clang::lifetimebound]]) -> ::clang::ObjCMethodDecl const*
{
  return llvm::dyn_cast<::clang::ObjCMethodDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_objc_protocol_decl(F const& This [[clang::lifetimebound]]) -> ::clang::ObjCProtocolDecl const*
{
  return llvm::dyn_cast<::clang::ObjCProtocolDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_record_decl(F const& This [[clang::lifetimebound]]) -> ::clang::RecordDecl const*
{
  return llvm::dyn_cast<::clang::RecordDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_type_decl(F const& This [[clang::lifetimebound]]) -> ::clang::TypeDecl const*
{
  return llvm::dyn_cast<::clang::TypeDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_name_decl(F const& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl const*
{
  return llvm::dyn_cast<::clang::TypedefNameDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_decl(F const& This [[clang::lifetimebound]]) -> ::clang::TypedefDecl const*
{
  return llvm::dyn_cast<::clang::TypedefDecl const>(&This);
}

} // namespace cxx_clang::clang::ast::decl
