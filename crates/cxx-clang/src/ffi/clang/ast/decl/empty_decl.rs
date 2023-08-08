use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::empty_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::empty_decl::EmptyDecl;

impl<'ctx> EmptyDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        empty_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        empty_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for EmptyDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for EmptyDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
