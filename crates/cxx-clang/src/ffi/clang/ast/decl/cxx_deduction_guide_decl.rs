use crate::{ffi::clang::ast::decl::function_decl::FunctionDecl, gen::clang::ast::decl::cxx_deduction_guide_decl};
use core::pin::Pin;

pub use crate::abi::clang::ast::decl::cxx_deduction_guide_decl::CxxDeductionGuideDecl;

impl<'ctx> CxxDeductionGuideDecl<'ctx> {
    #[inline]
    pub fn as_ref_function_decl(&self) -> &FunctionDecl<'ctx> {
        cxx_deduction_guide_decl::as_ref_function_decl(self)
    }

    #[inline]
    pub fn as_pin_function_decl(self: Pin<&mut Self>) -> Pin<&mut FunctionDecl<'ctx>> {
        cxx_deduction_guide_decl::as_pin_function_decl(self)
    }
}

impl<'ctx> AsRef<FunctionDecl<'ctx>> for CxxDeductionGuideDecl<'ctx> {
    #[inline]
    fn as_ref(&self) -> &FunctionDecl<'ctx> {
        self.as_ref_function_decl()
    }
}

impl<'ctx> ::core::ops::Deref for CxxDeductionGuideDecl<'ctx> {
    type Target = FunctionDecl<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_function_decl()
    }
}
