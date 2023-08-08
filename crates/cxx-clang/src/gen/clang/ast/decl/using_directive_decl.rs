#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingDirectiveDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_directive_decl"]
        type UsingDirectiveDecl<'ctx> = crate::ffi::clang::ast::decl::using_directive_decl::UsingDirectiveDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::using_directive_decl"]
    unsafe extern "C++" {
        fn as_ref_named_decl<'this, 'ctx>(This: &'this UsingDirectiveDecl<'ctx>) -> &'this NamedDecl<'ctx>;

        fn as_pin_named_decl<'this, 'ctx>(
            This: Pin<&'this mut UsingDirectiveDecl<'ctx>>,
        ) -> Pin<&'this mut NamedDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
