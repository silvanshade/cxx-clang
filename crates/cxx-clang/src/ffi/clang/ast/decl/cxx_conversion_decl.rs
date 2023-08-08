use crate::{ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl, gen::clang::ast::decl::cxx_conversion_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::cxx_conversion_decl::CxxConversionDecl;

impl<'ctx> CxxConversionDecl<'ctx> {
    #[inline]
    pub fn as_ref_cxx_method_decl(&self) -> &CxxMethodDecl<'ctx> {
        cxx_conversion_decl::as_ref_cxx_method_decl(self)
    }

    #[inline]
    pub fn as_pin_cxx_method_decl(self: Pin<&mut Self>) -> Pin<&mut CxxMethodDecl<'ctx>> {
        cxx_conversion_decl::as_pin_cxx_method_decl(self)
    }
}

impl<'ctx> AsRef<CxxMethodDecl<'ctx>> for CxxConversionDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &CxxMethodDecl<'ctx> {
        self.as_ref_cxx_method_decl()
    }
}

impl<'ctx> ::core::ops::Deref for CxxConversionDecl<'ctx> {
    type Target = CxxMethodDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_cxx_method_decl()
    }
}
