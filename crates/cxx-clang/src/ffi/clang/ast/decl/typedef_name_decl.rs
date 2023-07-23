use crate::{ffi::clang::ast::decl::type_decl::TypeDecl, gen::clang::ast::decl::typedef_name_decl};

pub use crate::abi::clang::ast::decl::typedef_name_decl::TypedefNameDecl;

impl<'ctx> TypedefNameDecl<'ctx> {
    #[inline]
    pub fn as_ref_type_decl(&self) -> &TypeDecl<'ctx> {
        typedef_name_decl::as_ref_type_decl(self)
    }
}

impl<'ctx> AsRef<TypeDecl<'ctx>> for TypedefNameDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypeDecl<'ctx> {
        self.as_ref_type_decl()
    }
}

impl<'ctx> ::core::ops::Deref for TypedefNameDecl<'ctx> {
    type Target = TypeDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_type_decl()
    }
}
