#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DecompositionDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/VarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decomposition_decl"]
        type DecompositionDecl<'ctx> = crate::ffi::clang::ast::decl::decomposition_decl::DecompositionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::decomposition_decl"]
    unsafe extern "C++" {
        fn as_ref_var_decl<'this, 'ctx>(This: &'this DecompositionDecl<'ctx>) -> &'this VarDecl<'ctx>;

        fn as_pin_var_decl<'this, 'ctx>(This: Pin<&'this mut DecompositionDecl<'ctx>>)
        -> Pin<&'this mut VarDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
