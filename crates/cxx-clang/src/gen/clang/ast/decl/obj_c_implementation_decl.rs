#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCImplDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCImplementationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_impl_decl"]
        type ObjCImplDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_implementation_decl"]
        type ObjCImplementationDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_implementation_decl::ObjCImplementationDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_implementation_decl"]
    unsafe extern "C++" {
        fn as_ref_obj_c_impl_decl<'this, 'ctx>(This: &'this ObjCImplementationDecl<'ctx>) -> &'this ObjCImplDecl<'ctx>;

        fn as_pin_obj_c_impl_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCImplementationDecl<'ctx>>,
        ) -> Pin<&'this mut ObjCImplDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
