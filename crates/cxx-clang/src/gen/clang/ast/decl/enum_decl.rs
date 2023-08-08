#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/EnumDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TagDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::enum_decl"]
        type EnumDecl<'ctx> = crate::ffi::clang::ast::decl::enum_decl::EnumDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::tag_decl"]
        type TagDecl<'ctx> = crate::ffi::clang::ast::decl::tag_decl::TagDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::enum_decl"]
    unsafe extern "C++" {
        fn as_ref_tag_decl<'this, 'ctx>(This: &'this EnumDecl<'ctx>) -> &'this TagDecl<'ctx>;

        fn as_pin_tag_decl<'this, 'ctx>(This: Pin<&'this mut EnumDecl<'ctx>>) -> Pin<&'this mut TagDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
