use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::obj_c_compatible_alias_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::obj_c_compatible_alias_decl::ObjCCompatibleAliasDecl;

impl<'ctx> ObjCCompatibleAliasDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        obj_c_compatible_alias_decl::as_ref_named_decl(self)
    }

    #[inline]
    pub fn as_pin_named_decl(self: Pin<&mut Self>) -> Pin<&mut NamedDecl<'ctx>> {
        obj_c_compatible_alias_decl::as_pin_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for ObjCCompatibleAliasDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjCCompatibleAliasDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
