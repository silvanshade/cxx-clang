use crate::{ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl, gen::clang::ast::decl::typedef_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::typedef_decl::TypedefDecl;

impl<'ctx> TypedefDecl<'ctx> {
    #[inline]
    pub fn as_ref_typedef_name_decl(&self) -> &TypedefNameDecl<'ctx> {
        typedef_decl::as_ref_typedef_name_decl(self)
    }

    #[inline]
    pub fn as_pin_typedef_name_decl(self: Pin<&mut Self>) -> Pin<&mut TypedefNameDecl<'ctx>> {
        typedef_decl::as_pin_typedef_name_decl(self)
    }
}

impl<'ctx> AsRef<TypedefNameDecl<'ctx>> for TypedefDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypedefNameDecl<'ctx> {
        self.as_ref_typedef_name_decl()
    }
}

impl<'ctx> ::core::ops::Deref for TypedefDecl<'ctx> {
    type Target = TypedefNameDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_typedef_name_decl()
    }
}
