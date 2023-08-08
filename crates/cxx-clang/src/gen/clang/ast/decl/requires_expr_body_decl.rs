#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RequiresExprBodyDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::requires_expr_body_decl"]
        type RequiresExprBodyDecl<'ctx> =
            crate::ffi::clang::ast::decl::requires_expr_body_decl::RequiresExprBodyDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::requires_expr_body_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this RequiresExprBodyDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut RequiresExprBodyDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;

        fn as_ref_decl_context<'this, 'ctx>(This: &'this RequiresExprBodyDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut RequiresExprBodyDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
