#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_memory::abi {
template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
cxx_is_move_constructible<::clang::OMPAllocateDecl>() noexcept -> bool
{
  return false;
}
} // namespace cxx_memory::abi

namespace cxx_clang::clang::ast::decl::omp_allocate_decl {
CXX_MEMORY_ABI_PRELUDE(OMPAllocateDecl, ::clang::OMPAllocateDecl)
} // namespace cxx_clang::clang::ast::decl::omp_allocate_decl

namespace cxx_clang::clang::ast::decl::omp_allocate_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_omp_declarative_directive_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::OMPDeclarativeDirective<::clang::Decl> const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_omp_declarative_directive_decl(Self& This [[clang::lifetimebound]])
  -> ::clang::OMPDeclarativeDirective<::clang::Decl>&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::omp_allocate_decl
