#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/LifetimeExtendedTemporaryDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl"]
        type LifetimeExtendedTemporaryDecl<'ctx> =
            crate::ffi::clang::ast::decl::lifetime_extended_temporary_decl::LifetimeExtendedTemporaryDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this LifetimeExtendedTemporaryDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(
            This: Pin<&'this mut LifetimeExtendedTemporaryDecl<'ctx>>,
        ) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
