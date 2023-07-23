use crate::{ffi::clang::ast::decl::value_decl::ValueDecl, gen::clang::ast::decl::declarator_decl};

pub use crate::abi::clang::ast::decl::declarator_decl::DeclaratorDecl;

impl<'ctx> DeclaratorDecl<'ctx> {
    #[inline]
    pub fn as_ref_value_decl(&self) -> &ValueDecl<'ctx> {
        declarator_decl::as_ref_value_decl(self)
    }
}

impl<'ctx> AsRef<ValueDecl<'ctx>> for DeclaratorDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &ValueDecl<'ctx> {
        self.as_ref_value_decl()
    }
}

impl<'ctx> ::core::ops::Deref for DeclaratorDecl<'ctx> {
    type Target = ValueDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_value_decl()
    }
}
