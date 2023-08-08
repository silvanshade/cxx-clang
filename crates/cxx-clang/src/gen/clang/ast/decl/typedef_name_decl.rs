#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypeDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypedefNameDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
        type TypedefNameDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
    unsafe extern "C++" {
        fn as_ref_type_decl<'this, 'ctx>(This: &'this TypedefNameDecl<'ctx>) -> &'this TypeDecl<'ctx>;

        fn as_pin_type_decl<'this, 'ctx>(This: Pin<&'this mut TypedefNameDecl<'ctx>>)
        -> Pin<&'this mut TypeDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
