use crate::{ffi::clang::ast::decl::type_decl::TypeDecl, gen::clang::ast::decl::template_type_parm_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::template_type_parm_decl::TemplateTypeParmDecl;

impl<'ctx> TemplateTypeParmDecl<'ctx> {
    #[inline]
    pub fn as_ref_type_decl(&self) -> &TypeDecl<'ctx> {
        template_type_parm_decl::as_ref_type_decl(self)
    }

    #[inline]
    pub fn as_pin_type_decl(self: Pin<&mut Self>) -> Pin<&mut TypeDecl<'ctx>> {
        template_type_parm_decl::as_pin_type_decl(self)
    }
}

impl<'ctx> AsRef<TypeDecl<'ctx>> for TemplateTypeParmDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypeDecl<'ctx> {
        self.as_ref_type_decl()
    }
}

impl<'ctx> ::core::ops::Deref for TemplateTypeParmDecl<'ctx> {
    type Target = TypeDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_type_decl()
    }
}
