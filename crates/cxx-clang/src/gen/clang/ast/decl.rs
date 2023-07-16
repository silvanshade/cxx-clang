#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");

        // #[namespace = "cxx_clang::clang::ast::decl"]
        // type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
