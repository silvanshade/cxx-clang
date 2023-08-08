use crate::{ffi::clang::ast::decl::var_decl::VarDecl, gen::clang::ast::decl::implicit_param_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::implicit_param_decl::ImplicitParamDecl;

impl<'ctx> ImplicitParamDecl<'ctx> {
    #[inline]
    pub fn as_ref_var_decl(&self) -> &VarDecl<'ctx> {
        implicit_param_decl::as_ref_var_decl(self)
    }

    #[inline]
    pub fn as_pin_var_decl(self: Pin<&mut Self>) -> Pin<&mut VarDecl<'ctx>> {
        implicit_param_decl::as_pin_var_decl(self)
    }
}

impl<'ctx> AsRef<VarDecl<'ctx>> for ImplicitParamDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &VarDecl<'ctx> {
        self.as_ref_var_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ImplicitParamDecl<'ctx> {
    type Target = VarDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_var_decl()
    }
}
