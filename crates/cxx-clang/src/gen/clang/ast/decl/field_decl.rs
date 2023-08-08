#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FieldDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
        type FieldDecl<'ctx> = crate::ffi::clang::ast::decl::field_decl::FieldDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
    unsafe extern "C++" {
        fn as_ref_declarator_decl<'this, 'ctx>(This: &'this FieldDecl<'ctx>) -> &'this DeclaratorDecl<'ctx>;

        fn as_pin_declarator_decl<'this, 'ctx>(
            This: Pin<&'this mut FieldDecl<'ctx>>,
        ) -> Pin<&'this mut DeclaratorDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
