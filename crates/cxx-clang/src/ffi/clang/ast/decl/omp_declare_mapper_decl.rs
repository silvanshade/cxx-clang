use crate::{
    ffi::clang::ast::decl::{
        decl_context::DeclContext,
        omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl,
    },
    gen::clang::ast::decl::omp_declare_mapper_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::omp_declare_mapper_decl::OmpDeclareMapperDecl;

impl<'ctx> OmpDeclareMapperDecl<'ctx> {
    #[inline]
    pub fn as_ref_declarative_directive_value_decl(&self) -> &OmpDeclarativeDirectiveValueDecl<'ctx> {
        omp_declare_mapper_decl::as_ref_declarative_directive_value_decl(self)
    }

    #[inline]
    pub fn as_pin_declarative_directive_value_decl(
        self: Pin<&mut Self>,
    ) -> Pin<&mut OmpDeclarativeDirectiveValueDecl<'ctx>> {
        omp_declare_mapper_decl::as_pin_declarative_directive_value_decl(self)
    }

    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        omp_declare_mapper_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        omp_declare_mapper_decl::as_pin_decl_context(self)
    }
}

impl<'ctx> AsRef<OmpDeclarativeDirectiveValueDecl<'ctx>> for OmpDeclareMapperDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &OmpDeclarativeDirectiveValueDecl<'ctx> {
        self.as_ref_declarative_directive_value_decl()
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for OmpDeclareMapperDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> ::core::ops::Deref for OmpDeclareMapperDecl<'ctx> {
    type Target = OmpDeclarativeDirectiveValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarative_directive_value_decl()
    }
}
