use crate::{
    ffi::clang::ast::decl::using_shadow_decl::UsingShadowDecl,
    gen::clang::ast::decl::constructor_using_shadow_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::constructor_using_shadow_decl::ConstructorUsingShadowDecl;

impl<'ctx> ConstructorUsingShadowDecl<'ctx> {
    #[inline]
    pub fn as_ref_using_shadow_decl(&self) -> &UsingShadowDecl<'ctx> {
        constructor_using_shadow_decl::as_ref_using_shadow_decl(self)
    }

    #[inline]
    pub fn as_pin_using_shadow_decl(self: Pin<&mut Self>) -> Pin<&mut UsingShadowDecl<'ctx>> {
        constructor_using_shadow_decl::as_pin_using_shadow_decl(self)
    }
}

impl<'ctx> AsRef<UsingShadowDecl<'ctx>> for ConstructorUsingShadowDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &UsingShadowDecl<'ctx> {
        self.as_ref_using_shadow_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ConstructorUsingShadowDecl<'ctx> {
    type Target = UsingShadowDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_using_shadow_decl()
    }
}
