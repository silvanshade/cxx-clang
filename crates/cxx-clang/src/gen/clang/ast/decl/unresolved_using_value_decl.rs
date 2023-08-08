#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnresolvedUsingValueDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_value_decl"]
        type UnresolvedUsingValueDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_value_decl::UnresolvedUsingValueDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_value_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this UnresolvedUsingValueDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut UnresolvedUsingValueDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
