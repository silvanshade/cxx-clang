#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/AccessSpecDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::access_spec_decl"]
        type AccessSpecDecl<'ctx> = crate::ffi::clang::ast::decl::access_spec_decl::AccessSpecDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::access_spec_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this AccessSpecDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut AccessSpecDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
