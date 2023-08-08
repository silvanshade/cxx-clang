use crate::{ffi::clang::ast::decl::type_decl::TypeDecl, gen::clang::ast::decl::unresolved_using_typename_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::unresolved_using_typename_decl::UnresolvedUsingTypenameDecl;

impl<'ctx> UnresolvedUsingTypenameDecl<'ctx> {
    #[inline]
    pub fn as_ref_type_decl(&self) -> &TypeDecl<'ctx> {
        unresolved_using_typename_decl::as_ref_type_decl(self)
    }

    #[inline]
    pub fn as_pin_type_decl(self: Pin<&mut Self>) -> Pin<&mut TypeDecl<'ctx>> {
        unresolved_using_typename_decl::as_pin_type_decl(self)
    }
}

impl<'ctx> AsRef<TypeDecl<'ctx>> for UnresolvedUsingTypenameDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypeDecl<'ctx> {
        self.as_ref_type_decl()
    }
}

impl<'ctx> ::core::ops::Deref for UnresolvedUsingTypenameDecl<'ctx> {
    type Target = TypeDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_type_decl()
    }
}
