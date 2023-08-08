use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::unresolved_using_value_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::unresolved_using_value_decl::UnresolvedUsingValueDecl;

impl<'ctx> UnresolvedUsingValueDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        unresolved_using_value_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        unresolved_using_value_decl::as_pin_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for UnresolvedUsingValueDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UnresolvedUsingValueDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
