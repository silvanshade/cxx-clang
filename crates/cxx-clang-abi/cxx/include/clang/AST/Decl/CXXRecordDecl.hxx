#pragma once

#include "cxx-clang-abi/cxx/include/cxx-clang-abi.hxx"

#include "clang/AST/DeclCXX.h"

namespace cxx_clang::clang::ast::decl::cxx_record_decl {
CXX_MEMORY_ABI_PRELUDE(CXXRecordDecl, ::clang::CXXRecordDecl)
} // namespace cxx_clang::clang::ast::decl::cxx_record_decl

namespace cxx_clang::clang::ast::decl::cxx_record_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_record_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::RecordDecl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_record_decl(Self& This [[clang::lifetimebound]]) -> ::clang::RecordDecl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::cxx_record_decl
