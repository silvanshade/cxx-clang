use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::named_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::named_decl::NamedDecl;

impl<'ctx> NamedDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        named_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        named_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for NamedDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for NamedDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
