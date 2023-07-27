#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamespaceAliasDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::namespace_alias_decl"]
        type NamespaceAliasDecl<'ctx> = crate::ffi::clang::ast::decl::namespace_alias_decl::NamespaceAliasDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::namespace_alias_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this NamespaceAliasDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut NamespaceAliasDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
