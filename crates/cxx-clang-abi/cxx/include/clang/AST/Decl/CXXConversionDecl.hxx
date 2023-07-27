#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::cxx_conversion_decl {
CXX_MEMORY_ABI_PRELUDE(CXXConversionDecl, ::clang::CXXConversionDecl)
} // namespace cxx_clang::clang::ast::decl::cxx_conversion_decl

namespace cxx_clang::clang::ast::decl::cxx_conversion_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_cxx_method_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXMethodDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_cxx_method_decl(Self& This [[clang::lifetimebound]]) -> ::clang::CXXMethodDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::cxx_conversion_decl
