#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateParamObjectDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::template_param_object_decl"]
        type TemplateParamObjectDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_param_object_decl::TemplateParamObjectDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::template_param_object_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this TemplateParamObjectDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut TemplateParamObjectDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
