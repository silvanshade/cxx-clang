use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::template_param_object_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::template_param_object_decl::TemplateParamObjectDecl;

impl<'ctx> TemplateParamObjectDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        template_param_object_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        template_param_object_decl::as_pin_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for TemplateParamObjectDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for TemplateParamObjectDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
