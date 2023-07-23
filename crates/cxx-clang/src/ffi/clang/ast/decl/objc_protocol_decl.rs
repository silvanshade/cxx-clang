use crate::{ffi::clang::ast::decl::objc_container_decl::ObjcContainerDecl, gen::clang::ast::decl::objc_protocol_decl};

pub use crate::abi::clang::ast::decl::objc_protocol_decl::ObjcProtocolDecl;

impl<'ctx> ObjcProtocolDecl<'ctx> {
    #[inline]
    pub fn as_ref_objc_container_decl(&self) -> &ObjcContainerDecl<'ctx> {
        objc_protocol_decl::as_ref_objc_container_decl(self)
    }
}

impl<'ctx> AsRef<ObjcContainerDecl<'ctx>> for ObjcProtocolDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjcContainerDecl<'ctx> {
        self.as_ref_objc_container_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjcProtocolDecl<'ctx> {
    type Target = ObjcContainerDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_objc_container_decl()
    }
}
