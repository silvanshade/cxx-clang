use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, type_decl::TypeDecl},
    gen::clang::ast::decl::tag_decl,
};

pub use crate::abi::clang::ast::decl::tag_decl::TagDecl;

impl<'ctx> TagDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        tag_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_ref_type_decl(&self) -> &TypeDecl<'ctx> {
        tag_decl::as_ref_type_decl(self)
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for TagDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> AsRef<TypeDecl<'ctx>> for TagDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &TypeDecl<'ctx> {
        self.as_ref_type_decl()
    }
}

impl<'ctx> ::core::ops::Deref for TagDecl<'ctx> {
    type Target = TypeDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_type_decl()
    }
}
