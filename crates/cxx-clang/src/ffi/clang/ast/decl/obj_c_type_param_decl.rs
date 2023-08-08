use crate::{ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl, gen::clang::ast::decl::obj_c_type_param_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::obj_c_type_param_decl::ObjCTypeParamDecl;

impl<'ctx> ObjCTypeParamDecl<'ctx> {
    #[inline]
    pub fn as_ref_typedef_name_decl(&self) -> &TypedefNameDecl<'ctx> {
        obj_c_type_param_decl::as_ref_typedef_name_decl(self)
    }

    #[inline]
    pub fn as_pin_typedef_name_decl(self: Pin<&mut Self>) -> Pin<&mut TypedefNameDecl<'ctx>> {
        obj_c_type_param_decl::as_pin_typedef_name_decl(self)
    }
}

impl<'ctx> AsRef<TypedefNameDecl<'ctx>> for ObjCTypeParamDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypedefNameDecl<'ctx> {
        self.as_ref_typedef_name_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCTypeParamDecl<'ctx> {
    type Target = TypedefNameDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_typedef_name_decl()
    }
}
