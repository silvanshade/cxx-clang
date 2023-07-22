#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Lex/MacroInfo/ModuleMacro.hxx");

        // #[namespace = "cxx_clang::clang::lex::macro_info::module_macro"]
        // type ModuleMacro<'ctx> = crate::ffi::clang::lex::macro_info::module_macro::ModuleMacro<'ctx>;
    }

    #[namespace = "cxx_clang::clang::lex::macro_info::module_macro"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
