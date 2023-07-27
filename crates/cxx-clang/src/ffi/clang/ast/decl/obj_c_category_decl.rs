use crate::{
    ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl,
    gen::clang::ast::decl::obj_c_category_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_category_decl::ObjCCategoryDecl;

impl<'ctx> ObjCCategoryDecl<'ctx> {
    #[inline]
    pub fn as_ref_obj_c_container_decl(&self) -> &ObjCContainerDecl<'ctx> {
        obj_c_category_decl::as_ref_obj_c_container_decl(self)
    }

    #[inline]
    pub fn as_pin_obj_c_container_decl(self: Pin<&mut Self>) -> Pin<&mut ObjCContainerDecl<'ctx>> {
        obj_c_category_decl::as_pin_obj_c_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjCContainerDecl<'ctx>> for ObjCCategoryDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjCContainerDecl<'ctx> {
        self.as_ref_obj_c_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCCategoryDecl<'ctx> {
    type Target = ObjCContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_obj_c_container_decl()
    }
}
