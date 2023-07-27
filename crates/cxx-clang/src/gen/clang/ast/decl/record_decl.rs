#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/RecordDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TagDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::record_decl"]
        type RecordDecl<'ctx> = crate::ffi::clang::ast::decl::record_decl::RecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::tag_decl"]
        type TagDecl<'ctx> = crate::ffi::clang::ast::decl::tag_decl::TagDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::record_decl"]
    unsafe extern "C++" {
        fn as_ref_tag_decl<'this, 'ctx>(This: &'this RecordDecl<'ctx>) -> &'this TagDecl<'ctx>;

        fn as_pin_tag_decl<'this, 'ctx>(This: Pin<&'this mut RecordDecl<'ctx>>) -> Pin<&'this mut TagDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
