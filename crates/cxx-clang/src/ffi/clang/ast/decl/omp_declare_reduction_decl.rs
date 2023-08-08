use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, value_decl::ValueDecl},
    gen::clang::ast::decl::omp_declare_reduction_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::omp_declare_reduction_decl::OmpDeclareReductionDecl;

impl<'ctx> OmpDeclareReductionDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        omp_declare_reduction_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        omp_declare_reduction_decl::as_pin_value_decl(self)
    }

    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        omp_declare_reduction_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        omp_declare_reduction_decl::as_pin_decl_context(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for OmpDeclareReductionDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for OmpDeclareReductionDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> ::core::ops::Deref for OmpDeclareReductionDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
