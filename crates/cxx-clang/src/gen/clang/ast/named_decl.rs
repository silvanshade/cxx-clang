#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/NamedDecl.hxx");

        // #[namespace = "cxx_clang::clang::ast::named_decl"]
        // type NamedDecl<'ctx> = crate::ffi::clang::ast::named_decl::NamedDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::named_decl"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
