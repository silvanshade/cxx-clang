#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPRequiresDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveDecl"]
        type OmpDeclarativeDirectiveDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_requires_decl"]
        #[cxx_name = "OMPRequiresDecl"]
        type OmpRequiresDecl<'ctx> = crate::ffi::clang::ast::decl::omp_requires_decl::OmpRequiresDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_requires_decl"]
    unsafe extern "C++" {
        fn as_ref_declarative_directive_decl<'this, 'ctx>(
            This: &'this OmpRequiresDecl<'ctx>,
        ) -> &'this OmpDeclarativeDirectiveDecl<'ctx>;

        fn as_pin_declarative_directive_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpRequiresDecl<'ctx>>,
        ) -> Pin<&'this mut OmpDeclarativeDirectiveDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
