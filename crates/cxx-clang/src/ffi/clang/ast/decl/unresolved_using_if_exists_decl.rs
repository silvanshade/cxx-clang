use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::unresolved_using_if_exists_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl;

impl<'ctx> UnresolvedUsingIfExistsDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        unresolved_using_if_exists_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        unresolved_using_if_exists_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for UnresolvedUsingIfExistsDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UnresolvedUsingIfExistsDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
