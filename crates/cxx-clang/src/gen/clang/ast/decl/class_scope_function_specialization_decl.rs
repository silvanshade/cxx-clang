#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassScopeFunctionSpecializationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::class_scope_function_specialization_decl"]
        type ClassScopeFunctionSpecializationDecl<'ctx> = crate::ffi::clang::ast::decl::class_scope_function_specialization_decl::ClassScopeFunctionSpecializationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::class_scope_function_specialization_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this ClassScopeFunctionSpecializationDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(
            This: Pin<&'this mut ClassScopeFunctionSpecializationDecl<'ctx>>,
        ) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
