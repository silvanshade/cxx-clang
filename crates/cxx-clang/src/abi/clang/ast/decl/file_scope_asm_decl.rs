#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct FileScopeAsmDecl<'ctx> {
    _layout: [u8; 56],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for FileScopeAsmDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::decl::file_scope_asm_decl::FileScopeAsmDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for FileScopeAsmDecl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for FileScopeAsmDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FileScopeAsmDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl::file_scope_asm_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/FileScopeAsmDecl.hxx");
        #[cxx_name = "FileScopeAsmDecl"]
        #[allow(unused)]
        type FileScopeAsmDecl<'ctx> = super::FileScopeAsmDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut FileScopeAsmDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<FileScopeAsmDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<FileScopeAsmDecl<'static>>(), 56)
        }
    }
}
