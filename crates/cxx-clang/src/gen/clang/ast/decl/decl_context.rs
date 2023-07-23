#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");

        // #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        // type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
