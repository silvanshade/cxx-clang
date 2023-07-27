#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ConceptDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TemplateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::concept_decl"]
        type ConceptDecl<'ctx> = crate::ffi::clang::ast::decl::concept_decl::ConceptDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_decl"]
        type TemplateDecl<'ctx> = crate::ffi::clang::ast::decl::template_decl::TemplateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::concept_decl"]
    unsafe extern "C++" {
        fn as_ref_template_decl<'this, 'ctx>(This: &'this ConceptDecl<'ctx>) -> &'this TemplateDecl<'ctx>;

        fn as_pin_template_decl<'this, 'ctx>(
            This: Pin<&'this mut ConceptDecl<'ctx>>,
        ) -> Pin<&'this mut TemplateDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
