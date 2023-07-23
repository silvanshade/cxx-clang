#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCProtocolDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::objc_container_decl"]
        #[cxx_name = "ObjCContainerDecl"]
        type ObjcContainerDecl<'ctx> = crate::ffi::clang::ast::decl::objc_container_decl::ObjcContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_protocol_decl"]
        #[cxx_name = "ObjCProtocolDecl"]
        type ObjcProtocolDecl<'ctx> = crate::ffi::clang::ast::decl::objc_protocol_decl::ObjcProtocolDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::objc_protocol_decl"]
    unsafe extern "C++" {
        fn as_ref_objc_container_decl<'this, 'ctx>(
            This: &'this ObjcProtocolDecl<'ctx>,
        ) -> &'this ObjcContainerDecl<'ctx>;
    }
}
pub(crate) use self::ffi::*;
