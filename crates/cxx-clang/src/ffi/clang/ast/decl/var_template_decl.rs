use crate::{
    ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl,
    gen::clang::ast::decl::var_template_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::var_template_decl::VarTemplateDecl;

impl<'ctx> VarTemplateDecl<'ctx> {
    #[inline]
    pub fn as_ref_redeclarable_template_decl(&self) -> &RedeclarableTemplateDecl<'ctx> {
        var_template_decl::as_ref_redeclarable_template_decl(self)
    }

    #[inline]
    pub fn as_pin_redeclarable_template_decl(self: Pin<&mut Self>) -> Pin<&mut RedeclarableTemplateDecl<'ctx>> {
        var_template_decl::as_pin_redeclarable_template_decl(self)
    }
}

impl<'ctx> AsRef<RedeclarableTemplateDecl<'ctx>> for VarTemplateDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &RedeclarableTemplateDecl<'ctx> {
        self.as_ref_redeclarable_template_decl()
    }
}

impl<'ctx> ::core::ops::Deref for VarTemplateDecl<'ctx> {
    type Target = RedeclarableTemplateDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_redeclarable_template_decl()
    }
}
