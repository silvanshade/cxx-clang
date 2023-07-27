#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/FieldDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCAtDefsFieldDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
        type FieldDecl<'ctx> = crate::ffi::clang::ast::decl::field_decl::FieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl"]
        type ObjCAtDefsFieldDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_at_defs_field_decl::ObjCAtDefsFieldDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl"]
    unsafe extern "C++" {
        fn as_ref_field_decl<'this, 'ctx>(This: &'this ObjCAtDefsFieldDecl<'ctx>) -> &'this FieldDecl<'ctx>;

        fn as_pin_field_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCAtDefsFieldDecl<'ctx>>,
        ) -> Pin<&'this mut FieldDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
