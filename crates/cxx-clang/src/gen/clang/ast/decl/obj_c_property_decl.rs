#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCPropertyDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_decl"]
        type ObjCPropertyDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_property_decl::ObjCPropertyDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this ObjCPropertyDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCPropertyDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
