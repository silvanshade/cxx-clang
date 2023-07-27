use crate::{ffi::clang::ast::decl::tag_decl::TagDecl, gen::clang::ast::decl::record_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::record_decl::RecordDecl;

impl<'ctx> RecordDecl<'ctx> {
    #[inline]
    pub fn as_ref_tag_decl(&self) -> &TagDecl<'ctx> {
        record_decl::as_ref_tag_decl(self)
    }

    #[inline]
    pub fn as_pin_tag_decl(self: Pin<&mut Self>) -> Pin<&mut TagDecl<'ctx>> {
        record_decl::as_pin_tag_decl(self)
    }
}

impl<'ctx> AsRef<TagDecl<'ctx>> for RecordDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TagDecl<'ctx> {
        self.as_ref_tag_decl()
    }
}

impl<'ctx> ::core::ops::Deref for RecordDecl<'ctx> {
    type Target = TagDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_tag_decl()
    }
}
