#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RedeclarableTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::redeclarable_template_decl"]
        type RedeclarableTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_decl"]
        type TemplateDecl<'ctx> = crate::ffi::clang::ast::decl::template_decl::TemplateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::redeclarable_template_decl"]
    unsafe extern "C++" {
        fn as_ref_template_decl<'this, 'ctx>(This: &'this RedeclarableTemplateDecl<'ctx>) -> &'this TemplateDecl<'ctx>;

        fn as_pin_template_decl<'this, 'ctx>(
            This: Pin<&'this mut RedeclarableTemplateDecl<'ctx>>,
        ) -> Pin<&'this mut TemplateDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
