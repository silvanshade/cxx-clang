use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::obj_c_property_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_property_decl::ObjCPropertyDecl;

impl<'ctx> ObjCPropertyDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        obj_c_property_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        obj_c_property_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for ObjCPropertyDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCPropertyDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
