use crate::{
    ffi::clang::ast::decl::{decl_context::DeclContext, named_decl::NamedDecl},
    gen::clang::ast::decl::objc_method_decl,
};

pub use crate::abi::clang::ast::decl::objc_method_decl::ObjcMethodDecl;

impl<'ctx> ObjcMethodDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl_context(&self) -> &DeclContext<'ctx> {
        objc_method_decl::as_ref_decl_context(self)
    }

    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        objc_method_decl::as_ref_named_decl(self)
    }
}

impl<'ctx> AsRef<DeclContext<'ctx>> for ObjcMethodDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &DeclContext<'ctx> {
        self.as_ref_decl_context()
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for ObjcMethodDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ObjcMethodDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
