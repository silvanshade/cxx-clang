use crate::{ffi::clang::ast::decl::template_decl::TemplateDecl, gen::clang::ast::decl::redeclarable_template_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl;

impl<'ctx> RedeclarableTemplateDecl<'ctx> {
    #[inline]
    pub fn as_ref_template_decl(&self) -> &TemplateDecl<'ctx> {
        redeclarable_template_decl::as_ref_template_decl(self)
    }

    #[inline]
    pub fn as_pin_template_decl(self: Pin<&mut Self>) -> Pin<&mut TemplateDecl<'ctx>> {
        redeclarable_template_decl::as_pin_template_decl(self)
    }
}

impl<'ctx> AsRef<TemplateDecl<'ctx>> for RedeclarableTemplateDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TemplateDecl<'ctx> {
        self.as_ref_template_decl()
    }
}

impl<'ctx> ::core::ops::Deref for RedeclarableTemplateDecl<'ctx> {
    type Target = TemplateDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_template_decl()
    }
}
