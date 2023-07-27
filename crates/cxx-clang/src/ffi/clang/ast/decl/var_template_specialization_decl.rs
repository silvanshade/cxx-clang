use crate::{ffi::clang::ast::decl::var_decl::VarDecl, gen::clang::ast::decl::var_template_specialization_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::var_template_specialization_decl::VarTemplateSpecializationDecl;

impl<'ctx> VarTemplateSpecializationDecl<'ctx> {
    #[inline]
    pub fn as_ref_var_decl(&self) -> &VarDecl<'ctx> {
        var_template_specialization_decl::as_ref_var_decl(self)
    }

    #[inline]
    pub fn as_pin_var_decl(self: Pin<&mut Self>) -> Pin<&mut VarDecl<'ctx>> {
        var_template_specialization_decl::as_pin_var_decl(self)
    }
}

impl<'ctx> AsRef<VarDecl<'ctx>> for VarTemplateSpecializationDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &VarDecl<'ctx> {
        self.as_ref_var_decl()
    }
}

impl<'ctx> ::core::ops::Deref for VarTemplateSpecializationDecl<'ctx> {
    type Target = VarDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_var_decl()
    }
}
