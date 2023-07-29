#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Type.hxx");

        // #[namespace = "cxx_clang::clang::ast::type"]
        // type Type<'ctx> = crate::ffi::clang::ast::type_::Type<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::type"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
