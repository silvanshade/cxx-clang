use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::file_scope_asm_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::file_scope_asm_decl::FileScopeAsmDecl;

impl<'ctx> FileScopeAsmDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        file_scope_asm_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        file_scope_asm_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for FileScopeAsmDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for FileScopeAsmDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
