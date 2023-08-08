use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::namespace_alias_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::namespace_alias_decl::NamespaceAliasDecl;

impl<'ctx> NamespaceAliasDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        namespace_alias_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        namespace_alias_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for NamespaceAliasDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for NamespaceAliasDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
