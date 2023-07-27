#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ClassTemplateSpecializationDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ClassTemplatePartialSpecializationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::class_template_partial_specialization_decl"]
        type ClassTemplatePartialSpecializationDecl<'ctx> = crate::ffi::clang::ast::decl::class_template_partial_specialization_decl::ClassTemplatePartialSpecializationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_template_specialization_decl"]
        type ClassTemplateSpecializationDecl<'ctx> =
            crate::ffi::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::class_template_partial_specialization_decl"]
    unsafe extern "C++" {
        fn as_ref_class_template_specialization_decl<'this, 'ctx>(
            This: &'this ClassTemplatePartialSpecializationDecl<'ctx>,
        ) -> &'this ClassTemplateSpecializationDecl<'ctx>;

        fn as_pin_class_template_specialization_decl<'this, 'ctx>(
            This: Pin<&'this mut ClassTemplatePartialSpecializationDecl<'ctx>>,
        ) -> Pin<&'this mut ClassTemplateSpecializationDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
