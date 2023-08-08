use crate::{ffi::clang::ast::decl::base_using_decl::BaseUsingDecl, gen::clang::ast::decl::using_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::using_decl::UsingDecl;

impl<'ctx> UsingDecl<'ctx> {
    #[inline]
    pub fn as_ref_base_using_decl(&self) -> &BaseUsingDecl<'ctx> {
        using_decl::as_ref_base_using_decl(self)
    }

    #[inline]
    pub fn as_pin_base_using_decl(self: Pin<&mut Self>) -> Pin<&mut BaseUsingDecl<'ctx>> {
        using_decl::as_pin_base_using_decl(self)
    }
}

impl<'ctx> AsRef<BaseUsingDecl<'ctx>> for UsingDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &BaseUsingDecl<'ctx> {
        self.as_ref_base_using_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UsingDecl<'ctx> {
    type Target = BaseUsingDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_base_using_decl()
    }
}
