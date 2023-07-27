use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, named_decl::NamedDecl},
    gen::clang::ast::decl::obj_c_container_decl,
};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl;

impl<'ctx> ObjCContainerDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        obj_c_container_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_pin_decl_context(self: Pin<&mut Self>) -> Pin<&mut DeclContext<'ctx>> {
        obj_c_container_decl::as_pin_decl_context(self)
    }

    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        obj_c_container_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        obj_c_container_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for ObjCContainerDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for ObjCContainerDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCContainerDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
