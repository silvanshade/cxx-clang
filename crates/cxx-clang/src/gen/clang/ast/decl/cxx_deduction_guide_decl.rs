#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXDeductionGuideDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FunctionDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_deduction_guide_decl"]
        #[cxx_name = "CXXDeductionGuideDecl"]
        type CxxDeductionGuideDecl<'ctx> =
            crate::ffi::clang::ast::decl::cxx_deduction_guide_decl::CxxDeductionGuideDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
        type FunctionDecl<'ctx> = crate::ffi::clang::ast::decl::function_decl::FunctionDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::cxx_deduction_guide_decl"]
    unsafe extern "C++" {
        fn as_ref_function_decl<'this, 'ctx>(This: &'this CxxDeductionGuideDecl<'ctx>) -> &'this FunctionDecl<'ctx>;

        fn as_pin_function_decl<'this, 'ctx>(
            This: Pin<&'this mut CxxDeductionGuideDecl<'ctx>>,
        ) -> Pin<&'this mut FunctionDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
