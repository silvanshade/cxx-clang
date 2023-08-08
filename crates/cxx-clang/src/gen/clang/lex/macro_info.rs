pub(crate) mod module_macro;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/Lex/MacroInfo.hxx");

        // #[namespace = "cxx_clang::clang::lex::macro_info"]
        // type MacroInfo<'ctx> = crate::ffi::clang::lex::macro_info::MacroInfo<'ctx>;
    }

    #[namespace = "cxx_clang::clang::lex::macro_info"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
