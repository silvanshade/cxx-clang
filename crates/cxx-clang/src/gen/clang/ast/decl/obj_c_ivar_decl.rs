#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FieldDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCIvarDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
        type FieldDecl<'ctx> = crate::ffi::clang::ast::decl::field_decl::FieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_ivar_decl"]
        type ObjCIvarDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_ivar_decl::ObjCIvarDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_ivar_decl"]
    unsafe extern "C++" {
        fn as_ref_field_decl<'this, 'ctx>(This: &'this ObjCIvarDecl<'ctx>) -> &'this FieldDecl<'ctx>;

        fn as_pin_field_decl<'this, 'ctx>(This: Pin<&'this mut ObjCIvarDecl<'ctx>>) -> Pin<&'this mut FieldDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
