#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
    unsafe extern "C++" {
        fn as_ref_declarator_decl<'this, 'ctx>(This: &'this VarDecl<'ctx>) -> &'this DeclaratorDecl<'ctx>;

        fn as_pin_declarator_decl<'this, 'ctx>(
            This: Pin<&'this mut VarDecl<'ctx>>,
        ) -> Pin<&'this mut DeclaratorDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
