#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypedefDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypedefNameDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::typedef_decl"]
        type TypedefDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_decl::TypedefDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
        type TypedefNameDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::typedef_decl"]
    unsafe extern "C++" {
        fn as_ref_typedef_name_decl<'this, 'ctx>(This: &'this TypedefDecl<'ctx>) -> &'this TypedefNameDecl<'ctx>;
    }
}
pub(crate) use self::ffi::*;
