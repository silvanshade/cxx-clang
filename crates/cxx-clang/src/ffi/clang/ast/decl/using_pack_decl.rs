use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::using_pack_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::using_pack_decl::UsingPackDecl;

impl<'ctx> UsingPackDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        using_pack_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        using_pack_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for UsingPackDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UsingPackDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
