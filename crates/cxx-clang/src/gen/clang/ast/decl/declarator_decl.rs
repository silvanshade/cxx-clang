#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this DeclaratorDecl<'ctx>) -> &'this ValueDecl<'ctx>;
    }
}
pub(crate) use self::ffi::*;
