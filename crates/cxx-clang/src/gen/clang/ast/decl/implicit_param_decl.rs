#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ImplicitParamDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::implicit_param_decl"]
        type ImplicitParamDecl<'ctx> = crate::ffi::clang::ast::decl::implicit_param_decl::ImplicitParamDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::implicit_param_decl"]
    unsafe extern "C++" {
        fn as_ref_var_decl<'this, 'ctx>(This: &'this ImplicitParamDecl<'ctx>) -> &'this VarDecl<'ctx>;

        fn as_pin_var_decl<'this, 'ctx>(This: Pin<&'this mut ImplicitParamDecl<'ctx>>)
        -> Pin<&'this mut VarDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
