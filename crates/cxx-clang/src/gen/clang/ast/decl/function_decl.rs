#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FunctionDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
        type FunctionDecl<'ctx> = crate::ffi::clang::ast::decl::function_decl::FunctionDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
    unsafe extern "C++" {
        fn as_ref_decl_context<'this, 'ctx>(This: &'this FunctionDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut FunctionDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;

        fn as_ref_declarator_decl<'this, 'ctx>(This: &'this FunctionDecl<'ctx>) -> &'this DeclaratorDecl<'ctx>;

        fn as_pin_declarator_decl<'this, 'ctx>(
            This: Pin<&'this mut FunctionDecl<'ctx>>,
        ) -> Pin<&'this mut DeclaratorDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
