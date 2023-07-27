#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TagDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::tag_decl"]
        type TagDecl<'ctx> = crate::ffi::clang::ast::decl::tag_decl::TagDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::tag_decl"]
    unsafe extern "C++" {
        fn as_ref_type_decl<'this, 'ctx>(This: &'this TagDecl<'ctx>) -> &'this TypeDecl<'ctx>;

        fn as_pin_type_decl<'this, 'ctx>(This: Pin<&'this mut TagDecl<'ctx>>) -> Pin<&'this mut TypeDecl<'ctx>>;

        fn as_ref_decl_context<'this, 'ctx>(This: &'this TagDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(This: Pin<&'this mut TagDecl<'ctx>>) -> Pin<&'this mut DeclContext<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
