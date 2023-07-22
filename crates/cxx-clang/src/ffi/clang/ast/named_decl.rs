use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::named_decl};

pub use crate::abi::clang::ast::named_decl::NamedDecl;

impl<'ctx> NamedDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        named_decl::as_ref_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for NamedDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for NamedDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}
