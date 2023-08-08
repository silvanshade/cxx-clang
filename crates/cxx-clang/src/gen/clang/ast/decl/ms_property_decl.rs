#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/MSPropertyDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::ms_property_decl"]
        #[cxx_name = "MSPropertyDecl"]
        type MsPropertyDecl<'ctx> = crate::ffi::clang::ast::decl::ms_property_decl::MsPropertyDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::ms_property_decl"]
    unsafe extern "C++" {
        fn as_ref_declarator_decl<'this, 'ctx>(This: &'this MsPropertyDecl<'ctx>) -> &'this DeclaratorDecl<'ctx>;

        fn as_pin_declarator_decl<'this, 'ctx>(
            This: Pin<&'this mut MsPropertyDecl<'ctx>>,
        ) -> Pin<&'this mut DeclaratorDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
