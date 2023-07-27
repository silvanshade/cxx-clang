#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveValueDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/OMPDeclareMapperDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveValueDecl"]
        type OmpDeclarativeDirectiveValueDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_declare_mapper_decl"]
        #[cxx_name = "OMPDeclareMapperDecl"]
        type OmpDeclareMapperDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declare_mapper_decl::OmpDeclareMapperDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_declare_mapper_decl"]
    unsafe extern "C++" {
        fn as_ref_declarative_directive_value_decl<'this, 'ctx>(
            This: &'this OmpDeclareMapperDecl<'ctx>,
        ) -> &'this OmpDeclarativeDirectiveValueDecl<'ctx>;

        fn as_pin_declarative_directive_value_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclareMapperDecl<'ctx>>,
        ) -> Pin<&'this mut OmpDeclarativeDirectiveValueDecl<'ctx>>;

        fn as_ref_decl_context<'this, 'ctx>(This: &'this OmpDeclareMapperDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclareMapperDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
