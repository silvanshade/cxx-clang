use crate::{
    ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl,
    gen::clang::ast::decl::omp_requires_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::omp_requires_decl::OmpRequiresDecl;

impl<'ctx> OmpRequiresDecl<'ctx> {
    #[inline]
    pub fn as_ref_declarative_directive_decl(&self) -> &OmpDeclarativeDirectiveDecl<'ctx> {
        omp_requires_decl::as_ref_declarative_directive_decl(self)
    }

    #[inline]
    pub fn as_pin_declarative_directive_decl(self: Pin<&mut Self>) -> Pin<&mut OmpDeclarativeDirectiveDecl<'ctx>> {
        omp_requires_decl::as_pin_declarative_directive_decl(self)
    }
}

impl<'ctx> AsRef<OmpDeclarativeDirectiveDecl<'ctx>> for OmpRequiresDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &OmpDeclarativeDirectiveDecl<'ctx> {
        self.as_ref_declarative_directive_decl()
    }
}

impl<'ctx> ::core::ops::Deref for OmpRequiresDecl<'ctx> {
    type Target = OmpDeclarativeDirectiveDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarative_directive_decl()
    }
}
