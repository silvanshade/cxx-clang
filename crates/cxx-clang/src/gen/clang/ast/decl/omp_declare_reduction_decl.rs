#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclareReductionDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_declare_reduction_decl"]
        #[cxx_name = "OMPDeclareReductionDecl"]
        type OmpDeclareReductionDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declare_reduction_decl::OmpDeclareReductionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_declare_reduction_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this OmpDeclareReductionDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclareReductionDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;

        fn as_ref_decl_context<'this, 'ctx>(This: &'this OmpDeclareReductionDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclareReductionDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
