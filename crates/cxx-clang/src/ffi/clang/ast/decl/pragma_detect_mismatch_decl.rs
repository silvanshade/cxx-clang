use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::pragma_detect_mismatch_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::pragma_detect_mismatch_decl::PragmaDetectMismatchDecl;

impl<'ctx> PragmaDetectMismatchDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        pragma_detect_mismatch_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        pragma_detect_mismatch_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for PragmaDetectMismatchDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for PragmaDetectMismatchDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
