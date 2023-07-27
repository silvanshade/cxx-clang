#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ImportDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::import_decl"]
        type ImportDecl<'ctx> = crate::ffi::clang::ast::decl::import_decl::ImportDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::import_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this ImportDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut ImportDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
