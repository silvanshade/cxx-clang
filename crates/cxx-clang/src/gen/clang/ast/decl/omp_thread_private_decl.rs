#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPThreadPrivateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveDecl"]
        type OmpDeclarativeDirectiveDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_thread_private_decl"]
        #[cxx_name = "OMPThreadPrivateDecl"]
        type OmpThreadPrivateDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_thread_private_decl::OmpThreadPrivateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_thread_private_decl"]
    unsafe extern "C++" {
        fn as_ref_declarative_directive_decl<'this, 'ctx>(
            This: &'this OmpThreadPrivateDecl<'ctx>,
        ) -> &'this OmpDeclarativeDirectiveDecl<'ctx>;

        fn as_pin_declarative_directive_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpThreadPrivateDecl<'ctx>>,
        ) -> Pin<&'this mut OmpDeclarativeDirectiveDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
