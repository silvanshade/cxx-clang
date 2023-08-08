use crate::{ffi::clang::ast::decl::record_decl::RecordDecl, gen::clang::ast::decl::cxx_record_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::cxx_record_decl::CxxRecordDecl;

impl<'ctx> CxxRecordDecl<'ctx> {
    #[inline]
    pub fn as_ref_record_decl(&self) -> &RecordDecl<'ctx> {
        cxx_record_decl::as_ref_record_decl(self)
    }

    #[inline]
    pub fn as_pin_record_decl(self: Pin<&mut Self>) -> Pin<&mut RecordDecl<'ctx>> {
        cxx_record_decl::as_pin_record_decl(self)
    }
}

impl<'ctx> AsRef<RecordDecl<'ctx>> for CxxRecordDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &RecordDecl<'ctx> {
        self.as_ref_record_decl()
    }
}

impl<'ctx> ::core::ops::Deref for CxxRecordDecl<'ctx> {
    type Target = RecordDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_record_decl()
    }
}
