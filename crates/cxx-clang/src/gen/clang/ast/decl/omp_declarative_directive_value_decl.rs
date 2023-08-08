#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveValueDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveValueDecl"]
        type OmpDeclarativeDirectiveValueDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(
            This: &'this OmpDeclarativeDirectiveValueDecl<'ctx>,
        ) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclarativeDirectiveValueDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
