use crate::{
    ffi::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl,
    gen::clang::ast::decl::class_template_partial_specialization_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::class_template_partial_specialization_decl::ClassTemplatePartialSpecializationDecl;

impl<'ctx> ClassTemplatePartialSpecializationDecl<'ctx> {
    #[inline]
    pub fn as_ref_class_template_specialization_decl(&self) -> &ClassTemplateSpecializationDecl<'ctx> {
        class_template_partial_specialization_decl::as_ref_class_template_specialization_decl(self)
    }

    #[inline]
    pub fn as_pin_class_template_specialization_decl(
        self: Pin<&mut Self>,
    ) -> Pin<&mut ClassTemplateSpecializationDecl<'ctx>> {
        class_template_partial_specialization_decl::as_pin_class_template_specialization_decl(self)
    }
}

impl<'ctx> AsRef<ClassTemplateSpecializationDecl<'ctx>> for ClassTemplatePartialSpecializationDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ClassTemplateSpecializationDecl<'ctx> {
        self.as_ref_class_template_specialization_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ClassTemplatePartialSpecializationDecl<'ctx> {
    type Target = ClassTemplateSpecializationDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_class_template_specialization_decl()
    }
}
