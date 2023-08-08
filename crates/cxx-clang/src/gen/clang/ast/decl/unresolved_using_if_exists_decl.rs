#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnresolvedUsingIfExistsDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl"]
        type UnresolvedUsingIfExistsDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this UnresolvedUsingIfExistsDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut UnresolvedUsingIfExistsDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
