#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXRecordDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ClassTemplateSpecializationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_record_decl"]
        #[cxx_name = "CXXRecordDecl"]
        type CxxRecordDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_record_decl::CxxRecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_template_specialization_decl"]
        type ClassTemplateSpecializationDecl<'ctx> =
            crate::ffi::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::class_template_specialization_decl"]
    unsafe extern "C++" {
        fn as_ref_cxx_record_decl<'this, 'ctx>(
            This: &'this ClassTemplateSpecializationDecl<'ctx>,
        ) -> &'this CxxRecordDecl<'ctx>;

        fn as_pin_cxx_record_decl<'this, 'ctx>(
            This: Pin<&'this mut ClassTemplateSpecializationDecl<'ctx>>,
        ) -> Pin<&'this mut CxxRecordDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
