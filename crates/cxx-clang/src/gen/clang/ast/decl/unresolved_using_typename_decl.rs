#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/UnresolvedUsingTypenameDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_typename_decl"]
        type UnresolvedUsingTypenameDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_typename_decl::UnresolvedUsingTypenameDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_typename_decl"]
    unsafe extern "C++" {
        fn as_ref_type_decl<'this, 'ctx>(This: &'this UnresolvedUsingTypenameDecl<'ctx>) -> &'this TypeDecl<'ctx>;

        fn as_pin_type_decl<'this, 'ctx>(
            This: Pin<&'this mut UnresolvedUsingTypenameDecl<'ctx>>,
        ) -> Pin<&'this mut TypeDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
