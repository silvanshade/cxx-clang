#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct UnresolvedUsingIfExistsDecl<'ctx> {
    _layout: [u8; 48],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for UnresolvedUsingIfExistsDecl<'ctx> {
    type Id =
        ::cxx::type_id!("cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for UnresolvedUsingIfExistsDecl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for UnresolvedUsingIfExistsDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UnresolvedUsingIfExistsDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/UnresolvedUsingIfExistsDecl.hxx");
        #[cxx_name = "UnresolvedUsingIfExistsDecl"]
        #[allow(unused)]
        type UnresolvedUsingIfExistsDecl<'ctx> = super::UnresolvedUsingIfExistsDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut UnresolvedUsingIfExistsDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<UnresolvedUsingIfExistsDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<UnresolvedUsingIfExistsDecl<'static>>(), 48)
        }
    }
}
