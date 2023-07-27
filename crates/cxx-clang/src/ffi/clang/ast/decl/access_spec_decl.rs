use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::access_spec_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::access_spec_decl::AccessSpecDecl;

impl<'ctx> AccessSpecDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        access_spec_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        access_spec_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for AccessSpecDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for AccessSpecDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
