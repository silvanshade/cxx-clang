#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCCategoryImplDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCImplDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_category_impl_decl"]
        type ObjCCategoryImplDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_category_impl_decl::ObjCCategoryImplDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_impl_decl"]
        type ObjCImplDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::obj_c_category_impl_decl"]
    unsafe extern "C++" {
        fn as_ref_obj_c_impl_decl<'this, 'ctx>(This: &'this ObjCCategoryImplDecl<'ctx>) -> &'this ObjCImplDecl<'ctx>;

        fn as_pin_obj_c_impl_decl<'this, 'ctx>(
            This: Pin<&'this mut ObjCCategoryImplDecl<'ctx>>,
        ) -> Pin<&'this mut ObjCImplDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
