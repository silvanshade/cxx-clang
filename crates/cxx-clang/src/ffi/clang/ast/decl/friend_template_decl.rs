use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::friend_template_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::friend_template_decl::FriendTemplateDecl;

impl<'ctx> FriendTemplateDecl<'ctx> {
    #[inline]
    pub fn as_ref_redeclarable_template_decl(&self) -> &Decl<'ctx> {
        friend_template_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_redeclarable_template_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        friend_template_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for FriendTemplateDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_redeclarable_template_decl()
    }
}

impl<'ctx> ::core::ops::Deref for FriendTemplateDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_redeclarable_template_decl()
    }
}
