#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPCapturedExprDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::omp_captured_expr_decl"]
        #[cxx_name = "OMPCapturedExprDecl"]
        type OmpCapturedExprDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_captured_expr_decl::OmpCapturedExprDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_captured_expr_decl"]
    unsafe extern "C++" {
        fn as_ref_var_decl<'this, 'ctx>(This: &'this OmpCapturedExprDecl<'ctx>) -> &'this VarDecl<'ctx>;

        fn as_pin_var_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpCapturedExprDecl<'ctx>>,
        ) -> Pin<&'this mut VarDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
