#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/BaseUsingDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingEnumDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::base_using_decl"]
        type BaseUsingDecl<'ctx> = crate::ffi::clang::ast::decl::base_using_decl::BaseUsingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_enum_decl"]
        type UsingEnumDecl<'ctx> = crate::ffi::clang::ast::decl::using_enum_decl::UsingEnumDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::using_enum_decl"]
    unsafe extern "C++" {
        fn as_ref_base_using_decl<'this, 'ctx>(This: &'this UsingEnumDecl<'ctx>) -> &'this BaseUsingDecl<'ctx>;

        fn as_pin_base_using_decl<'this, 'ctx>(
            This: Pin<&'this mut UsingEnumDecl<'ctx>>,
        ) -> Pin<&'this mut BaseUsingDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
