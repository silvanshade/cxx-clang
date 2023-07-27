#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/BaseUsingDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/UsingDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::base_using_decl"]
        type BaseUsingDecl<'ctx> = crate::ffi::clang::ast::decl::base_using_decl::BaseUsingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_decl"]
        type UsingDecl<'ctx> = crate::ffi::clang::ast::decl::using_decl::UsingDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::using_decl"]
    unsafe extern "C++" {
        fn as_ref_base_using_decl<'this, 'ctx>(This: &'this UsingDecl<'ctx>) -> &'this BaseUsingDecl<'ctx>;

        fn as_pin_base_using_decl<'this, 'ctx>(
            This: Pin<&'this mut UsingDecl<'ctx>>,
        ) -> Pin<&'this mut BaseUsingDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
