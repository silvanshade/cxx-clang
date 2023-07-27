use crate::{ffi::clang::ast::decl::var_decl::VarDecl, gen::clang::ast::decl::omp_captured_expr_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::omp_captured_expr_decl::OmpCapturedExprDecl;

impl<'ctx> OmpCapturedExprDecl<'ctx> {
    #[inline]
    pub fn as_ref_var_decl(&self) -> &VarDecl<'ctx> {
        omp_captured_expr_decl::as_ref_var_decl(self)
    }

    #[inline]
    pub fn as_pin_var_decl(self: Pin<&mut Self>) -> Pin<&mut VarDecl<'ctx>> {
        omp_captured_expr_decl::as_pin_var_decl(self)
    }
}

impl<'ctx> AsRef<VarDecl<'ctx>> for OmpCapturedExprDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &VarDecl<'ctx> {
        self.as_ref_var_decl()
    }
}

impl<'ctx> ::core::ops::Deref for OmpCapturedExprDecl<'ctx> {
    type Target = VarDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_var_decl()
    }
}
