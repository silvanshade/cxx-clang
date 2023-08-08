use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, named_decl::NamedDecl},
    gen::clang::ast::decl::namespace_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::namespace_decl::NamespaceDecl;

impl<'ctx> NamespaceDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        namespace_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        namespace_decl::as_pin_named_decl(self)
    }

    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        namespace_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        namespace_decl::as_pin_decl_context(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for NamespaceDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for NamespaceDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> ::core::ops::Deref for NamespaceDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
