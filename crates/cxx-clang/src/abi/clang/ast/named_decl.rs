#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct NamedDecl<'ctx> {
    _layout: [u8; 48],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for NamedDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::named_decl::NamedDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for NamedDecl<'ctx> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for NamedDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NamedDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::named_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/NamedDecl.hxx");
        #[cxx_name = "NamedDecl"]
        #[allow(unused)]
        type NamedDecl<'ctx> = super::NamedDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut NamedDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<NamedDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<NamedDecl<'static>>(), 48)
        }
    }
}
