#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/EmptyDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::empty_decl"]
        type EmptyDecl<'ctx> = crate::ffi::clang::ast::decl::empty_decl::EmptyDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::empty_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this EmptyDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut EmptyDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
