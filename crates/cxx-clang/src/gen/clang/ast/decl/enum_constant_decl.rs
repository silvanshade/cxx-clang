#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/EnumConstantDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::enum_constant_decl"]
        type EnumConstantDecl<'ctx> = crate::ffi::clang::ast::decl::enum_constant_decl::EnumConstantDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::enum_constant_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this EnumConstantDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(
            This: Pin<&'this mut EnumConstantDecl<'ctx>>,
        ) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
