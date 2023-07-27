use crate::{
    ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl,
    gen::clang::ast::decl::obj_c_protocol_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_protocol_decl::ObjCProtocolDecl;

impl<'ctx> ObjCProtocolDecl<'ctx> {
    #[inline]
    pub fn as_ref_obj_c_container_decl(&self) -> &ObjCContainerDecl<'ctx> {
        obj_c_protocol_decl::as_ref_obj_c_container_decl(self)
    }

    #[inline]
    pub fn as_pin_obj_c_container_decl(self: Pin<&mut Self>) -> Pin<&mut ObjCContainerDecl<'ctx>> {
        obj_c_protocol_decl::as_pin_obj_c_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjCContainerDecl<'ctx>> for ObjCProtocolDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjCContainerDecl<'ctx> {
        self.as_ref_obj_c_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCProtocolDecl<'ctx> {
    type Target = ObjCContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_obj_c_container_decl()
    }
}
