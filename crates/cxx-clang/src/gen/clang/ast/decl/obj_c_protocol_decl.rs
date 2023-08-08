#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCProtocolDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
        type ObjCContainerDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_protocol_decl"]
        type ObjCProtocolDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_protocol_decl::ObjCProtocolDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_protocol_decl"]
    unsafe extern "C++" {
        fn as_ref_obj_c_container_decl<'this, 'ctx>(
            This: &'this ObjCProtocolDecl<'ctx>,
        ) -> &'this ObjCContainerDecl<'ctx>;

        fn as_pin_obj_c_container_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCProtocolDecl<'ctx>>,
        ) -> Pin<&'this mut ObjCContainerDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
