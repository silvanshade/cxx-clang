#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this NamedDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut NamedDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
