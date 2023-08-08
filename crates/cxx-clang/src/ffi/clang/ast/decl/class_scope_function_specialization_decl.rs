use crate::{ffi::clang::ast::decl::Decl, gen::clang::ast::decl::class_scope_function_specialization_decl};
use core::pin::Pin;

pub use crate::auto::clang::ast::decl::class_scope_function_specialization_decl::ClassScopeFunctionSpecializationDecl;

impl<'ctx> ClassScopeFunctionSpecializationDecl<'ctx> {
    #[inline]
    pub fn as_ref_decl(&self) -> &Decl<'ctx> {
        class_scope_function_specialization_decl::as_ref_decl(self)
    }

    #[inline]
    pub fn as_pin_decl(self: Pin<&mut Self>) -> Pin<&mut Decl<'ctx>> {
        class_scope_function_specialization_decl::as_pin_decl(self)
    }
}

impl<'ctx> AsRef<Decl<'ctx>> for ClassScopeFunctionSpecializationDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &Decl<'ctx> {
        self.as_ref_decl()
    }
}

impl<'ctx> ::core::ops::Deref for ClassScopeFunctionSpecializationDecl<'ctx> {
    type Target = Decl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_decl()
    }
}
