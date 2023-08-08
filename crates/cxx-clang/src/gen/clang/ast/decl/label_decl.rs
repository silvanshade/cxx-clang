#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/LabelDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::label_decl"]
        type LabelDecl<'ctx> = crate::ffi::clang::ast::decl::label_decl::LabelDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::label_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this LabelDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(This: Pin<&'this mut LabelDecl<'ctx>>) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
