#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/ASTContext.h"

namespace cxx_clang::clang::ast::ast_context {
using ASTContext = ::clang::ASTContext;
using F = ASTContext;

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

} // namespace cxx_clang::clang::ast::ast_context

namespace cxx_clang::clang::ast::ast_context {
[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_clang::clang::ast::ast_context
