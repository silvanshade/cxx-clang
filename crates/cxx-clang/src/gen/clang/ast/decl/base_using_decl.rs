#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/BaseUsingDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::base_using_decl"]
        type BaseUsingDecl<'ctx> = crate::ffi::clang::ast::decl::base_using_decl::BaseUsingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::base_using_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this BaseUsingDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(This: Pin<&'this mut BaseUsingDecl<'ctx>>)
        -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
