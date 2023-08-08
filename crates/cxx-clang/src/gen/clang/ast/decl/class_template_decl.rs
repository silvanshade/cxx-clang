#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RedeclarableTemplateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::class_template_decl"]
        type ClassTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::class_template_decl::ClassTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::redeclarable_template_decl"]
        type RedeclarableTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::class_template_decl"]
    unsafe extern "C++" {
        fn as_ref_redeclarable_template_decl<'this, 'ctx>(
            This: &'this ClassTemplateDecl<'ctx>,
        ) -> &'this RedeclarableTemplateDecl<'ctx>;

        fn as_pin_redeclarable_template_decl<'this, 'ctx>(
            This: Pin<&'this mut ClassTemplateDecl<'ctx>>,
        ) -> Pin<&'this mut RedeclarableTemplateDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
