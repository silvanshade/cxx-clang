use crate::{
    ffi::clang::ast::decl::objc_container_decl::ObjcContainerDecl,
    gen::clang::ast::decl::objc_interface_decl,
};

pub use crate::abi::clang::ast::decl::objc_interface_decl::ObjcInterfaceDecl;

impl<'ctx> ObjcInterfaceDecl<'ctx> {
    #[inline]
    pub fn as_ref_objc_container_decl(&self) -> &ObjcContainerDecl<'ctx> {
        objc_interface_decl::as_ref_objc_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjcContainerDecl<'ctx>> for ObjcInterfaceDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjcContainerDecl<'ctx> {
        self.as_ref_objc_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjcInterfaceDecl<'ctx> {
    type Target = ObjcContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_objc_container_decl()
    }
}
