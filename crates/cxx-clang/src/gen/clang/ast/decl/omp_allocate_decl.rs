#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/OMPAllocateDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::omp_allocate_decl"]
        #[cxx_name = "OMPAllocateDecl"]
        type OmpAllocateDecl<'ctx> = crate::ffi::clang::ast::decl::omp_allocate_decl::OmpAllocateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveDecl"]
        type OmpDeclarativeDirectiveDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_allocate_decl"]
    unsafe extern "C++" {
        fn as_ref_omp_declarative_directive_decl<'this, 'ctx>(
            This: &'this OmpAllocateDecl<'ctx>,
        ) -> &'this OmpDeclarativeDirectiveDecl<'ctx>;

        fn as_pin_omp_declarative_directive_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpAllocateDecl<'ctx>>,
        ) -> Pin<&'this mut OmpDeclarativeDirectiveDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
