use crate::{ffi::clang::ast::decl::tag_decl::TagDecl, gen::clang::ast::decl::enum_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::enum_decl::EnumDecl;

impl<'ctx> EnumDecl<'ctx> {
    #[inline]
    pub fn as_ref_tag_decl(&self) -> &TagDecl<'ctx> {
        enum_decl::as_ref_tag_decl(self)
    }

    #[inline]
    pub fn as_pin_tag_decl(self: Pin<&mut Self>) -> Pin<&mut TagDecl<'ctx>> {
        enum_decl::as_pin_tag_decl(self)
    }
}

impl<'ctx> AsRef<TagDecl<'ctx>> for EnumDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TagDecl<'ctx> {
        self.as_ref_tag_decl()
    }
}

impl<'ctx> ::core::ops::Deref for EnumDecl<'ctx> {
    type Target = TagDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_tag_decl()
    }
}
