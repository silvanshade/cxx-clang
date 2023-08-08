#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/IndirectFieldDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::indirect_field_decl"]
        type IndirectFieldDecl<'ctx> = crate::ffi::clang::ast::decl::indirect_field_decl::IndirectFieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::indirect_field_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this IndirectFieldDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut IndirectFieldDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
