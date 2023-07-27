#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXMethodDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/FunctionDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_method_decl"]
        #[cxx_name = "CXXMethodDecl"]
        type CxxMethodDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
        type FunctionDecl<'ctx> = crate::ffi::clang::ast::decl::function_decl::FunctionDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::cxx_method_decl"]
    unsafe extern "C++" {
        fn as_ref_function_decl<'this, 'ctx>(This: &'this CxxMethodDecl<'ctx>) -> &'this FunctionDecl<'ctx>;

        fn as_pin_function_decl<'this, 'ctx>(
            This: Pin<&'this mut CxxMethodDecl<'ctx>>,
        ) -> Pin<&'this mut FunctionDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
