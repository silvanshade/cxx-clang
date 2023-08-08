#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateTypeParmDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypeDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::template_type_parm_decl"]
        type TemplateTypeParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_type_parm_decl::TemplateTypeParmDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::template_type_parm_decl"]
    unsafe extern "C++" {
        fn as_ref_type_decl<'this, 'ctx>(This: &'this TemplateTypeParmDecl<'ctx>) -> &'this TypeDecl<'ctx>;

        fn as_pin_type_decl<'this, 'ctx>(
            This: Pin<&'this mut TemplateTypeParmDecl<'ctx>>,
        ) -> Pin<&'this mut TypeDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
