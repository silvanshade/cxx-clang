#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCInterfaceDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::objc_container_decl"]
        #[cxx_name = "ObjCContainerDecl"]
        type ObjcContainerDecl<'ctx> = crate::ffi::clang::ast::decl::objc_container_decl::ObjcContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_interface_decl"]
        #[cxx_name = "ObjCInterfaceDecl"]
        type ObjcInterfaceDecl<'ctx> = crate::ffi::clang::ast::decl::objc_interface_decl::ObjcInterfaceDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::objc_interface_decl"]
    unsafe extern "C++" {
        fn as_ref_objc_container_decl<'this, 'ctx>(
            This: &'this ObjcInterfaceDecl<'ctx>,
        ) -> &'this ObjcContainerDecl<'ctx>;
    }
}
pub(crate) use self::ffi::*;
