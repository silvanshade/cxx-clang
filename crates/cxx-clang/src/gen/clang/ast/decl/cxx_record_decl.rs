#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/CXXRecordDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/RecordDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::cxx_record_decl"]
        #[cxx_name = "CXXRecordDecl"]
        type CxxRecordDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_record_decl::CxxRecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::record_decl"]
        type RecordDecl<'ctx> = crate::ffi::clang::ast::decl::record_decl::RecordDecl<'ctx>;
    }

    #[namespace = "cxx_clang::clang::ast::decl::cxx_record_decl"]
    unsafe extern "C++" {
        fn as_ref_record_decl<'this, 'ctx>(This: &'this CxxRecordDecl<'ctx>) -> &'this RecordDecl<'ctx>;

        fn as_pin_record_decl<'this, 'ctx>(
            This: Pin<&'this mut CxxRecordDecl<'ctx>>,
        ) -> Pin<&'this mut RecordDecl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
