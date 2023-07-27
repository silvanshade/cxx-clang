#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeAliasDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypedefNameDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::type_alias_decl"]
        type TypeAliasDecl<'ctx> = crate::ffi::clang::ast::decl::type_alias_decl::TypeAliasDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
        type TypedefNameDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::type_alias_decl"]
    unsafe extern "C++" {
        fn as_ref_typedef_name_decl<'this, 'ctx>(This: &'this TypeAliasDecl<'ctx>) -> &'this TypedefNameDecl<'ctx>;

        fn as_pin_typedef_name_decl<'this, 'ctx>(
            This: Pin<&'this mut TypeAliasDecl<'ctx>>,
        ) -> Pin<&'this mut TypedefNameDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
