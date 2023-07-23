#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub(crate) mod decl_context;
pub(crate) mod declarator_decl;
pub(crate) mod field_decl;
pub(crate) mod function_decl;
pub(crate) mod named_decl;
pub(crate) mod objc_container_decl;
pub(crate) mod objc_interface_decl;
pub(crate) mod objc_method_decl;
pub(crate) mod objc_protocol_decl;
pub(crate) mod record_decl;
pub(crate) mod tag_decl;
pub(crate) mod type_decl;
pub(crate) mod typedef_decl;
pub(crate) mod typedef_name_decl;
pub(crate) mod value_decl;
#[repr(C, align(8))]
pub struct Decl<'ctx> {
    _layout: [u8; 40],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for Decl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::decl::Decl");
    type Kind = ::cxx::kind::Opaque;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for Decl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Decl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        #[cxx_name = "Decl"]
        #[allow(unused)]
        type Decl<'ctx> = super::Decl<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<Decl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<Decl<'static>>(), 40)
        }
    }
}
