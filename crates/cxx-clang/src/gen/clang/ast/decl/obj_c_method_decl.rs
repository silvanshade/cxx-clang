#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCMethodDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_method_decl"]
        type ObjCMethodDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_method_decl::ObjCMethodDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_method_decl"]
    unsafe extern "C++" {
        fn as_ref_decl_context<'this, 'ctx>(This: &'this ObjCMethodDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut ObjCMethodDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;

        fn as_ref_named_decl<'this, 'ctx>(This: &'this ObjCMethodDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCMethodDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
