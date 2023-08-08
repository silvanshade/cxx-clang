#pragma once

#include "cxx-clang-auto/cxx/include/cxx-clang-auto.hxx"

#include "clang/AST/Decl.h"

namespace cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl {
CXX_AUTO_PRELUDE(PragmaDetectMismatchDecl, ::clang::PragmaDetectMismatchDecl)
} // namespace cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl

namespace cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::Decl const&
{
  return This;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_decl(Self& This [[clang::lifetimebound]]) -> ::clang::Decl&
{
  return This;
}

} // namespace cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl
