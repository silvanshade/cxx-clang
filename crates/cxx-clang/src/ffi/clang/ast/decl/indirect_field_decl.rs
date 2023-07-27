use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::indirect_field_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::indirect_field_decl::IndirectFieldDecl;

impl<'ctx> IndirectFieldDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        indirect_field_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        indirect_field_decl::as_pin_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for IndirectFieldDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for IndirectFieldDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
