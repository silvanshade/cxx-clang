use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::import_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::import_decl::ImportDecl;

impl<'ctx> ImportDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        import_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        import_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for ImportDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ImportDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
