use crate::{
    ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl,
    gen::clang::ast::decl::function_template_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::function_template_decl::FunctionTemplateDecl;

impl<'ctx> FunctionTemplateDecl<'ctx> {
    #[inline]
    pub fn as_ref_redeclarable_template_decl(&self) -> &RedeclarableTemplateDecl<'ctx> {
        function_template_decl::as_ref_redeclarable_template_decl(self)
    }

    #[inline]
    pub fn as_pin_redeclarable_template_decl(self: Pin<&mut Self>) -> Pin<&mut RedeclarableTemplateDecl<'ctx>> {
        function_template_decl::as_pin_redeclarable_template_decl(self)
    }
}

impl<'ctx> AsRef<RedeclarableTemplateDecl<'ctx>> for FunctionTemplateDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &RedeclarableTemplateDecl<'ctx> {
        self.as_ref_redeclarable_template_decl()
    }
}

impl<'ctx> ::core::ops::Deref for FunctionTemplateDecl<'ctx> {
    type Target = RedeclarableTemplateDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_redeclarable_template_decl()
    }
}
