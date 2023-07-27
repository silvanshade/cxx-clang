#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/VarDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/VarTemplateSpecializationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_template_specialization_decl"]
        type VarTemplateSpecializationDecl<'ctx> =
            crate::ffi::clang::ast::decl::var_template_specialization_decl::VarTemplateSpecializationDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::var_template_specialization_decl"]
    unsafe extern "C++" {
        fn as_ref_var_decl<'this, 'ctx>(This: &'this VarTemplateSpecializationDecl<'ctx>) -> &'this VarDecl<'ctx>;

        fn as_pin_var_decl<'this, 'ctx>(
            This: Pin<&'this mut VarTemplateSpecializationDecl<'ctx>>,
        ) -> Pin<&'this mut VarDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
