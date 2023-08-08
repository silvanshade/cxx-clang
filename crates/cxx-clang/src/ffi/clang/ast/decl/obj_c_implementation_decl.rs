use crate::{ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl, gen::clang::ast::decl::obj_c_implementation_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::obj_c_implementation_decl::ObjCImplementationDecl;

impl<'ctx> ObjCImplementationDecl<'ctx> {
    #[inline]
    pub fn as_ref_obj_c_impl_decl(&self) -> &ObjCImplDecl<'ctx> {
        obj_c_implementation_decl::as_ref_obj_c_impl_decl(self)
    }

    #[inline]
    pub fn as_pin_obj_c_impl_decl(self: Pin<&mut Self>) -> Pin<&mut ObjCImplDecl<'ctx>> {
        obj_c_implementation_decl::as_pin_obj_c_impl_decl(self)
    }
}

impl<'ctx> AsRef<ObjCImplDecl<'ctx>> for ObjCImplementationDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ObjCImplDecl<'ctx> {
        self.as_ref_obj_c_impl_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCImplementationDecl<'ctx> {
    type Target = ObjCImplDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_obj_c_impl_decl()
    }
}
