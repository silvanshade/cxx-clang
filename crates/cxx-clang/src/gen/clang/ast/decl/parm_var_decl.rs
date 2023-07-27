#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ParmVarDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/VarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::parm_var_decl"]
        type ParmVarDecl<'ctx> = crate::ffi::clang::ast::decl::parm_var_decl::ParmVarDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::parm_var_decl"]
    unsafe extern "C++" {
        fn as_ref_var_decl<'this, 'ctx>(This: &'this ParmVarDecl<'ctx>) -> &'this VarDecl<'ctx>;

        fn as_pin_var_decl<'this, 'ctx>(This: Pin<&'this mut ParmVarDecl<'ctx>>) -> Pin<&'this mut VarDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
