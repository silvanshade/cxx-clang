#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXDestructorDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXMethodDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_destructor_decl"]
        #[cxx_name = "CXXDestructorDecl"]
        type CxxDestructorDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_destructor_decl::CxxDestructorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_method_decl"]
        #[cxx_name = "CXXMethodDecl"]
        type CxxMethodDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::cxx_destructor_decl"]
    unsafe extern "C++" {
        fn as_ref_cxx_method_decl<'this, 'ctx>(This: &'this CxxDestructorDecl<'ctx>) -> &'this CxxMethodDecl<'ctx>;

        fn as_pin_cxx_method_decl<'this, 'ctx>(
            This: Pin<&'this mut CxxDestructorDecl<'ctx>>,
        ) -> Pin<&'this mut CxxMethodDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
