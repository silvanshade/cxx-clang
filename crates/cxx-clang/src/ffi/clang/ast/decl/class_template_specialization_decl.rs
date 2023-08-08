use crate::{
    ffi::clang::ast::decl::cxx_record_decl::CxxRecordDecl,
    gen::clang::ast::decl::class_template_specialization_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl;

impl<'ctx> ClassTemplateSpecializationDecl<'ctx> {
    #[inline]
    pub fn as_ref_cxx_record_decl(&self) -> &CxxRecordDecl<'ctx> {
        class_template_specialization_decl::as_ref_cxx_record_decl(self)
    }

    #[inline]
    pub fn as_pin_cxx_record_decl(self: Pin<&mut Self>) -> Pin<&mut CxxRecordDecl<'ctx>> {
        class_template_specialization_decl::as_pin_cxx_record_decl(self)
    }
}

impl<'ctx> AsRef<CxxRecordDecl<'ctx>> for ClassTemplateSpecializationDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &CxxRecordDecl<'ctx> {
        self.as_ref_cxx_record_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ClassTemplateSpecializationDecl<'ctx> {
    type Target = CxxRecordDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_cxx_record_decl()
    }
}
