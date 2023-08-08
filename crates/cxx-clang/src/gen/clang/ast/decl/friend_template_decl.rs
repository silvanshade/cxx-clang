#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FriendTemplateDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::friend_template_decl"]
        type FriendTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::friend_template_decl::FriendTemplateDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::friend_template_decl"]
    unsafe extern "C++" {
        fn as_ref_decl<'this, 'ctx>(This: &'this FriendTemplateDecl<'ctx>) -> &'this Decl<'ctx>;

        fn as_pin_decl<'this, 'ctx>(This: Pin<&'this mut FriendTemplateDecl<'ctx>>) -> Pin<&'this mut Decl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
