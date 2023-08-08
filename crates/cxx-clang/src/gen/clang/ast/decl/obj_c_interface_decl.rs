#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCInterfaceDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
        type ObjCContainerDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_interface_decl"]
        type ObjCInterfaceDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_interface_decl::ObjCInterfaceDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_interface_decl"]
    unsafe extern "C++" {
        fn as_ref_obj_c_container_decl<'this, 'ctx>(
            This: &'this ObjCInterfaceDecl<'ctx>,
        ) -> &'this ObjCContainerDecl<'ctx>;

        fn as_pin_obj_c_container_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCInterfaceDecl<'ctx>>,
        ) -> Pin<&'this mut ObjCContainerDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
