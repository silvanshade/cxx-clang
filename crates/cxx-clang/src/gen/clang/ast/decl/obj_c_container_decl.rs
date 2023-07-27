#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclContext.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::decl_context"]
        type DeclContext<'ctx> = crate::ffi::clang::ast::decl::decl_context::DeclContext<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
        type ObjCContainerDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
    unsafe extern "C++" {
        fn as_ref_decl_context<'this, 'ctx>(This: &'this ObjCContainerDecl<'ctx>) -> &'this DeclContext<'ctx>;

        fn as_pin_decl_context<'this, 'ctx>(
            This: Pin<&'this mut ObjCContainerDecl<'ctx>>,
        ) -> Pin<&'this mut DeclContext<'ctx>>;

        fn as_ref_named_decl<'this, 'ctx>(This: &'this ObjCContainerDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCContainerDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
