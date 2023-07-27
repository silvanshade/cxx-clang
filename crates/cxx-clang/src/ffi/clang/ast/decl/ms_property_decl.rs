use crate::{ffi::clang::ast::decl::declarator_decl::DeclaratorDecl, gen::clang::ast::decl::ms_property_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::ms_property_decl::MsPropertyDecl;

impl<'ctx> MsPropertyDecl<'ctx> {
    #[inline]
    pub fn as_ref_declarator_decl(&self) -> &DeclaratorDecl<'ctx> {
        ms_property_decl::as_ref_declarator_decl(self)
    }

    #[inline]
    pub fn as_pin_declarator_decl(self: Pin<&mut Self>) -> Pin<&mut DeclaratorDecl<'ctx>> {
        ms_property_decl::as_pin_declarator_decl(self)
    }
}

impl<'ctx> AsRef<DeclaratorDecl<'ctx>> for MsPropertyDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclaratorDecl<'ctx> {
        self.as_ref_declarator_decl()
    }
}

impl<'ctx> ::core::ops::Deref for MsPropertyDecl<'ctx> {
    type Target = DeclaratorDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarator_decl()
    }
}
