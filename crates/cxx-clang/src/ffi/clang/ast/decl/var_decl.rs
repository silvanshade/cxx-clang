use crate::{ffi::clang::ast::decl::declarator_decl::DeclaratorDecl, gen::clang::ast::decl::var_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::var_decl::VarDecl;

impl<'ctx> VarDecl<'ctx> {
    #[inline]
    pub fn as_ref_declarator_decl(&self) -> &DeclaratorDecl<'ctx> {
        var_decl::as_ref_declarator_decl(self)
    }

    #[inline]
    pub fn as_pin_declarator_decl(self: Pin<&mut Self>) -> Pin<&mut DeclaratorDecl<'ctx>> {
        var_decl::as_pin_declarator_decl(self)
    }
}

impl<'ctx> AsRef<DeclaratorDecl<'ctx>> for VarDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclaratorDecl<'ctx> {
        self.as_ref_declarator_decl()
    }
}

impl<'ctx> ::core::ops::Deref for VarDecl<'ctx> {
    type Target = DeclaratorDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarator_decl()
    }
}
