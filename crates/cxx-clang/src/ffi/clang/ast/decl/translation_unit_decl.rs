use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, Decl},
    gen::clang::ast::decl::translation_unit_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::translation_unit_decl::TranslationUnitDecl;

impl<'ctx> TranslationUnitDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        translation_unit_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        translation_unit_decl::as_pin_decl(self)
    }

    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        translation_unit_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        translation_unit_decl::as_pin_decl_context(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for TranslationUnitDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for TranslationUnitDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> ::core::ops::Deref for TranslationUnitDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
