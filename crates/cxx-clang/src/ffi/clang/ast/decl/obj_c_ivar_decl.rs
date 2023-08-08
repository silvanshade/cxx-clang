use crate::{ffi::clang::ast::decl::field_decl::FieldDecl, gen::clang::ast::decl::obj_c_ivar_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::obj_c_ivar_decl::ObjCIvarDecl;

impl<'ctx> ObjCIvarDecl<'ctx> {
    #[inline]
    pub fn as_ref_field_decl(&self) -> &FieldDecl<'ctx> {
        obj_c_ivar_decl::as_ref_field_decl(self)
    }

    #[inline]
    pub fn as_pin_field_decl(self: Pin<&mut Self>) -> Pin<&mut FieldDecl<'ctx>> {
        obj_c_ivar_decl::as_pin_field_decl(self)
    }
}

impl<'ctx> AsRef<FieldDecl<'ctx>> for ObjCIvarDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &FieldDecl<'ctx> {
        self.as_ref_field_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCIvarDecl<'ctx> {
    type Target = FieldDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_field_decl()
    }
}
