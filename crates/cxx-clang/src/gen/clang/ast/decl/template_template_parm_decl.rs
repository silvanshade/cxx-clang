#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateTemplateParmDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::template_decl"]
        type TemplateDecl<'ctx> = crate::ffi::clang::ast::decl::template_decl::TemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_template_parm_decl"]
        type TemplateTemplateParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_template_parm_decl::TemplateTemplateParmDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::template_template_parm_decl"]
    unsafe extern "C++" {
        fn as_ref_template_decl<'this, 'ctx>(This: &'this TemplateTemplateParmDecl<'ctx>) -> &'this TemplateDecl<'ctx>;

        fn as_pin_template_decl<'this, 'ctx>(
            This: Pin<&'this mut TemplateTemplateParmDecl<'ctx>>,
        ) -> Pin<&'this mut TemplateDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
