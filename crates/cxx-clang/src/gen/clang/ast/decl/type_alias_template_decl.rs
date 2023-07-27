#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/RedeclarableTemplateDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeAliasTemplateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::redeclarable_template_decl"]
        type RedeclarableTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_alias_template_decl"]
        type TypeAliasTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::type_alias_template_decl::TypeAliasTemplateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::type_alias_template_decl"]
    unsafe extern "C++" {
        fn as_ref_redeclarable_template_decl<'this, 'ctx>(
            This: &'this TypeAliasTemplateDecl<'ctx>,
        ) -> &'this RedeclarableTemplateDecl<'ctx>;

        fn as_pin_redeclarable_template_decl<'this, 'ctx>(
            This: Pin<&'this mut TypeAliasTemplateDecl<'ctx>>,
        ) -> Pin<&'this mut RedeclarableTemplateDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
