#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXConstructorDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXMethodDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_constructor_decl"]
        #[cxx_name = "CXXConstructorDecl"]
        type CxxConstructorDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_constructor_decl::CxxConstructorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_method_decl"]
        #[cxx_name = "CXXMethodDecl"]
        type CxxMethodDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::cxx_constructor_decl"]
    unsafe extern "C++" {
        fn as_ref_cxx_method_decl<'this, 'ctx>(This: &'this CxxConstructorDecl<'ctx>) -> &'this CxxMethodDecl<'ctx>;

        fn as_pin_cxx_method_decl<'this, 'ctx>(
            This: Pin<&'this mut CxxConstructorDecl<'ctx>>,
        ) -> Pin<&'this mut CxxMethodDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
