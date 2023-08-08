#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/ASTContext.hxx");

        // #[namespace = "cxx_clang::clang::ast::ast_context"]
        // #[cxx_name = "ASTContext"]
        // type AstContext<'ctx> = crate::ffi::clang::ast::ast_context::AstContext<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::ast_context"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
