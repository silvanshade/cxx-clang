use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::static_assert_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::static_assert_decl::StaticAssertDecl;

impl<'ctx> StaticAssertDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        static_assert_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        static_assert_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for StaticAssertDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for StaticAssertDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
