#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct UsingDecl<'ctx> {
    _layout: [u8; 88],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for UsingDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::decl::using_decl::UsingDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for UsingDecl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for UsingDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UsingDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl::using_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/UsingDecl.hxx");
        #[cxx_name = "UsingDecl"]
        #[allow(unused)]
        type UsingDecl<'ctx> = super::UsingDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut UsingDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<UsingDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<UsingDecl<'static>>(), 88)
        }
    }
}