#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ConstructorUsingShadowDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/UsingShadowDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::constructor_using_shadow_decl"]
        type ConstructorUsingShadowDecl<'ctx> =
            crate::ffi::clang::ast::decl::constructor_using_shadow_decl::ConstructorUsingShadowDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_shadow_decl"]
        type UsingShadowDecl<'ctx> = crate::ffi::clang::ast::decl::using_shadow_decl::UsingShadowDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::constructor_using_shadow_decl"]
    unsafe extern "C++" {
        fn as_ref_using_shadow_decl<'this, 'ctx>(
            This: &'this ConstructorUsingShadowDecl<'ctx>,
        ) -> &'this UsingShadowDecl<'ctx>;

        fn as_pin_using_shadow_decl<'this, 'ctx>(
            This: Pin<&'this mut ConstructorUsingShadowDecl<'ctx>>,
        ) -> Pin<&'this mut UsingShadowDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
