#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct ObjCPropertyDecl<'ctx> {
    _layout: [u8; 128],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for ObjCPropertyDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::decl::obj_c_property_decl::ObjCPropertyDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for ObjCPropertyDecl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for ObjCPropertyDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ObjCPropertyDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCPropertyDecl.hxx");
        #[cxx_name = "ObjCPropertyDecl"]
        #[allow(unused)]
        type ObjCPropertyDecl<'ctx> = super::ObjCPropertyDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut ObjCPropertyDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ObjCPropertyDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ObjCPropertyDecl<'static>>(), 128)
        }
    }
}
