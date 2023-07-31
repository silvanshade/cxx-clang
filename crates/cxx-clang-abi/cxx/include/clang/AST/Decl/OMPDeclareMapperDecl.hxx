#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclOpenMP.h"

namespace cxx_memory::abi {
template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
cxx_is_move_constructible<::clang::OMPDeclareMapperDecl>() noexcept -> bool
{
  return false;
}
} // namespace cxx_memory::abi

namespace cxx_clang::clang::ast::decl::omp_declare_mapper_decl {
CXX_MEMORY_ABI_PRELUDE(OMPDeclareMapperDecl, ::clang::OMPDeclareMapperDecl)
} // namespace cxx_clang::clang::ast::decl::omp_declare_mapper_decl

namespace cxx_clang::clang::ast::decl::omp_declare_mapper_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_declarative_directive_value_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::OMPDeclarativeDirective<::clang::ValueDecl> const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_declarative_directive_value_decl(Self& This [[clang::lifetimebound]])
  -> ::clang::OMPDeclarativeDirective<::clang::ValueDecl>&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl_context(Self const& This [[clang::lifetimebound]]) -> ::clang::DeclContext const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl_context(Self& This [[clang::lifetimebound]]) -> ::clang::DeclContext&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::omp_declare_mapper_decl
