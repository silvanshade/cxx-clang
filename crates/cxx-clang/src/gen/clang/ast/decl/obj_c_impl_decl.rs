#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCImplDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
        type ObjCContainerDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_impl_decl"]
        type ObjCImplDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_impl_decl"]
    unsafe extern "C++" {
        fn as_ref_obj_c_container_decl<'this, 'ctx>(This: &'this ObjCImplDecl<'ctx>) -> &'this ObjCContainerDecl<'ctx>;

        fn as_pin_obj_c_container_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCImplDecl<'ctx>>,
        ) -> Pin<&'this mut ObjCContainerDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
