#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ExportDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::export_decl"]
        type ExportDecl<'ctx> = crate::ffi::clang::ast::decl::export_decl::ExportDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::export_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this ExportDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut ExportDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;

        fn as_ref_decl_context<'this, 'ctx>(This: &'this ExportDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut ExportDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
