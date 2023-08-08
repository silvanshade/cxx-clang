use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, declarator_decl::DeclaratorDecl},
    gen::clang::ast::decl::function_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::function_decl::FunctionDecl;

impl<'ctx> FunctionDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        function_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        function_decl::as_pin_decl_context(self)
    }

    #[inline]
    pub fn as_ref_declarator_decl(&self) -> &DeclaratorDecl<'ctx> {
        function_decl::as_ref_declarator_decl(self)
    }

    #[inline]
    pub fn as_pin_declarator_decl(self: Pin<&mut Self>) -> Pin<&mut DeclaratorDecl<'ctx>> {
        function_decl::as_pin_declarator_decl(self)
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for FunctionDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> AsRef<DeclaratorDecl<'ctx>> for FunctionDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclaratorDecl<'ctx> {
        self.as_ref_declarator_decl()
    }
}

impl<'ctx> ::core::ops::Deref for FunctionDecl<'ctx> {
    type Target = DeclaratorDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_declarator_decl()
    }
}
