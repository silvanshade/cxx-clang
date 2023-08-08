use crate::{
    ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl,
    gen::clang::ast::decl::obj_c_interface_decl,
};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::obj_c_interface_decl::ObjCInterfaceDecl;

impl<'ctx> ObjCInterfaceDecl<'ctx> {
    #[inline]
    pub fn as_ref_obj_c_container_decl(&self) -> &ObjCContainerDecl<'ctx> {
        obj_c_interface_decl::as_ref_obj_c_container_decl(self)
    }

    #[inline]
    pub fn as_pin_obj_c_container_decl(self: Pin<&mut Self>) -> Pin<&mut ObjCContainerDecl<'ctx>> {
        obj_c_interface_decl::as_pin_obj_c_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjCContainerDecl<'ctx>> for ObjCInterfaceDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjCContainerDecl<'ctx> {
        self.as_ref_obj_c_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCInterfaceDecl<'ctx> {
    type Target = ObjCContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_obj_c_container_decl()
    }
}
