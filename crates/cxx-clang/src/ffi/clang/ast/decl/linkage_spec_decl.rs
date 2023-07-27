use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, Decl},
    gen::clang::ast::decl::linkage_spec_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::linkage_spec_decl::LinkageSpecDecl;

impl<'ctx> LinkageSpecDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        linkage_spec_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        linkage_spec_decl::as_pin_decl(self)
    }

    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        linkage_spec_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        linkage_spec_decl::as_pin_decl_context(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for LinkageSpecDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for LinkageSpecDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> ::core::ops::Deref for LinkageSpecDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
