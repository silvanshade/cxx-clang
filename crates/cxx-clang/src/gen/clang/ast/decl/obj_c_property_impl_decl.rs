#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCPropertyImplDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_impl_decl"]
        type ObjCPropertyImplDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_property_impl_decl::ObjCPropertyImplDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_impl_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this ObjCPropertyImplDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut ObjCPropertyImplDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
