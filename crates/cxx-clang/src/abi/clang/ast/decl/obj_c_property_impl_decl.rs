#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct ObjCPropertyImplDecl<'ctx> {
    _layout: [u8; 96],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for ObjCPropertyImplDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_clang::clang::ast::decl::obj_c_property_impl_decl::ObjCPropertyImplDecl");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for ObjCPropertyImplDecl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for ObjCPropertyImplDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ObjCPropertyImplDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_impl_decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCPropertyImplDecl.hxx");
        #[cxx_name = "ObjCPropertyImplDecl"]
        #[allow(unused)]
        type ObjCPropertyImplDecl<'ctx> = super::ObjCPropertyImplDecl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut ObjCPropertyImplDecl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ObjCPropertyImplDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ObjCPropertyImplDecl<'static>>(), 96)
        }
    }
}
