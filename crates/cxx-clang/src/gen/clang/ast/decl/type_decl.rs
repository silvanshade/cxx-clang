#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this TypeDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(This: Pin<&'this mut TypeDecl<'ctx>>) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
