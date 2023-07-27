use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::unnamed_global_constant_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::unnamed_global_constant_decl::UnnamedGlobalConstantDecl;

impl<'ctx> UnnamedGlobalConstantDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        unnamed_global_constant_decl::as_ref_value_decl(self)
    }

    #[inline]
    pub fn as_pin_value_decl(self: Pin<&mut Self>) -> Pin<&mut ValueDecl<'ctx>> {
        unnamed_global_constant_decl::as_pin_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for UnnamedGlobalConstantDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UnnamedGlobalConstantDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
