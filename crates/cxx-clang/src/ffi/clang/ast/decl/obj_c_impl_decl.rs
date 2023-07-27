use crate::{ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl, gen::clang::ast::decl::obj_c_impl_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl;

impl<'ctx> ObjCImplDecl<'ctx> {
    #[inline]
    pub fn as_ref_obj_c_container_decl(&self) -> &ObjCContainerDecl<'ctx> {
        obj_c_impl_decl::as_ref_obj_c_container_decl(self)
    }

    #[inline]
    pub fn as_pin_obj_c_container_decl(self: Pin<&mut Self>) -> Pin<&mut ObjCContainerDecl<'ctx>> {
        obj_c_impl_decl::as_pin_obj_c_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjCContainerDecl<'ctx>> for ObjCImplDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjCContainerDecl<'ctx> {
        self.as_ref_obj_c_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCImplDecl<'ctx> {
    type Target = ObjCContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_obj_c_container_decl()
    }
}
