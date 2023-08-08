use crate::{ffi::clang::ast::decl::declarator_decl::DeclaratorDecl, gen::clang::ast::decl::field_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::field_decl::FieldDecl;

impl<'ctx> FieldDecl<'ctx> {
    #[inline]
    pub fn as_ref_declarator_decl(&self) -> &DeclaratorDecl<'ctx> {
        field_decl::as_ref_declarator_decl(self)
    }

    #[inline]
    pub fn as_pin_declarator_decl(self: Pin<&mut Self>) -> Pin<&mut DeclaratorDecl<'ctx>> {
        field_decl::as_pin_declarator_decl(self)
    }
}

impl<'ctx> AsRef<DeclaratorDecl<'ctx>> for FieldDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclaratorDecl<'ctx> {
        self.as_ref_declarator_decl()
    }
}

impl<'ctx> ::core::ops::Deref for FieldDecl<'ctx> {
    type Target = DeclaratorDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarator_decl()
    }
}
