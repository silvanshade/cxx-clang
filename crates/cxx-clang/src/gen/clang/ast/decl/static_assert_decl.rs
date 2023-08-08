#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/StaticAssertDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::static_assert_decl"]
        type StaticAssertDecl<'ctx> = crate::ffi::clang::ast::decl::static_assert_decl::StaticAssertDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::static_assert_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this StaticAssertDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut StaticAssertDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
