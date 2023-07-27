#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/PragmaDetectMismatchDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl"]
        type PragmaDetectMismatchDecl<'ctx> =
            crate::ffi::clang::ast::decl::pragma_detect_mismatch_decl::PragmaDetectMismatchDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this PragmaDetectMismatchDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut PragmaDetectMismatchDecl<'ctx>>)
        -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
