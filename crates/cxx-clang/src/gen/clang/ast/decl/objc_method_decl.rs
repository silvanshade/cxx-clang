#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCMethodDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_method_decl"]
        #[cxx_name = "ObjCMethodDecl"]
        type ObjcMethodDecl<'ctx> = crate::ffi::clang::ast::decl::objc_method_decl::ObjcMethodDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::objc_method_decl"]
    unsafe extern "C++" {
        fn as_ref_decl_context<'this, 'ctx>(This: &'this ObjcMethodDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_ref_named_decl<'this, 'ctx>(This: &'this ObjcMethodDecl<'ctx>) -> &'this NamedDecl<'ctx>;
    }
}
pub(crate) use self::ffi::*;
