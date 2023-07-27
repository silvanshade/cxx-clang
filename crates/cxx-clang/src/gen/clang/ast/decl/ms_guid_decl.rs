#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/MSGuidDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ValueDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::ms_guid_decl"]
        #[cxx_name = "MSGuidDecl"]
        type MsGuidDecl<'ctx> = crate::ffi::clang::ast::decl::ms_guid_decl::MsGuidDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::ms_guid_decl"]
    unsafe extern "C++" {
        fn as_ref_value_decl<'this, 'ctx>(This: &'this MsGuidDecl<'ctx>) -> &'this ValueDecl<'ctx>;

        fn as_pin_value_decl<'this, 'ctx>(This: Pin<&'this mut MsGuidDecl<'ctx>>) -> Pin<&'this mut ValueDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
