use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::binding_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::binding_decl::BindingDecl;

impl<'ctx> BindingDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        binding_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        binding_decl::as_pin_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for BindingDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for BindingDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
