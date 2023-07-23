use crate::{ffi::clang::ast::decl::named_decl::NamedDecl, gen::clang::ast::decl::value_decl};

pub use crate::abi::clang::ast::decl::value_decl::ValueDecl;

impl<'ctx> ValueDecl<'ctx> {
    #[inline]
    pub fn as_ref_named_decl(&self) -> &NamedDecl<'ctx> {
        value_decl::as_ref_named_decl(self)
    }
}

impl<'ctx> AsRef<NamedDecl<'ctx>> for ValueDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &NamedDecl<'ctx> {
        self.as_ref_named_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ValueDecl<'ctx> {
    type Target = NamedDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_named_decl()
    }
}
