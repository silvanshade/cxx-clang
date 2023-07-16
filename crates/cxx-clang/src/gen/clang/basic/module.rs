#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Basic/Module.hxx");

        // #[namespace = "cxx_clang::clang::basic::module"]
        // type Module<'ctx> = crate::ffi::clang::basic::module::Module<'ctx>;
    }

    #[namespace = "cxx_clang::clang::basic::module"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
