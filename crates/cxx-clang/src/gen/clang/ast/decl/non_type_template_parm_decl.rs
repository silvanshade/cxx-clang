#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NonTypeTemplateParmDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::non_type_template_parm_decl"]
        type NonTypeTemplateParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::non_type_template_parm_decl::NonTypeTemplateParmDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::non_type_template_parm_decl"]
    unsafe extern "C++" {
        fn as_ref_declarator_decl<'this, 'ctx>(
            This: &'this NonTypeTemplateParmDecl<'ctx>,
        ) -> &'this DeclaratorDecl<'ctx>;

        fn as_pin_declarator_decl<'this, 'ctx>(
            This: Pin<&'this mut NonTypeTemplateParmDecl<'ctx>>,
        ) -> Pin<&'this mut DeclaratorDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
