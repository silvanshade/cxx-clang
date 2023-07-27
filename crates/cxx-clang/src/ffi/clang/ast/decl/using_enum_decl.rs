use crate::{ffi::clang::ast::decl::base_using_decl::BaseUsingDecl, gen::clang::ast::decl::using_enum_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::using_enum_decl::UsingEnumDecl;

impl<'ctx> UsingEnumDecl<'ctx> {
    #[inline]
    pub fn as_ref_base_using_decl(&self) -> &BaseUsingDecl<'ctx> {
        using_enum_decl::as_ref_base_using_decl(self)
    }

    #[inline]
    pub fn as_pin_base_using_decl(self: Pin<&mut Self>) -> Pin<&mut BaseUsingDecl<'ctx>> {
        using_enum_decl::as_pin_base_using_decl(self)
    }
}

impl<'ctx> AsRef<BaseUsingDecl<'ctx>> for UsingEnumDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &BaseUsingDecl<'ctx> {
        self.as_ref_base_using_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UsingEnumDecl<'ctx> {
    type Target = BaseUsingDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_base_using_decl()
    }
}
