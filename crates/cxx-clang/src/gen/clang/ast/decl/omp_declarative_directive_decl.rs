#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
        #[cxx_name = "OMPDeclarativeDirectiveDecl"]
        type OmpDeclarativeDirectiveDecl<'ctx> =
            crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this OmpDeclarativeDirectiveDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(
            This: Pin<&'this mut OmpDeclarativeDirectiveDecl<'ctx>>,
        ) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
