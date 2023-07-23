pub(crate) mod decl_context;
pub(crate) mod declarator_decl;
pub(crate) mod field_decl;
pub(crate) mod function_decl;
pub(crate) mod named_decl;
pub(crate) mod objc_container_decl;
pub(crate) mod objc_interface_decl;
pub(crate) mod objc_method_decl;
pub(crate) mod objc_protocol_decl;
pub(crate) mod record_decl;
pub(crate) mod tag_decl;
pub(crate) mod type_decl;
pub(crate) mod typedef_decl;
pub(crate) mod typedef_name_decl;
pub(crate) mod value_decl;

#[cxx::bridge]
mod ffi {
    #![allow(unreachable_patterns)]

    #[namespace = "cxx_clang::clang::ast::decl"]
    #[derive(Debug)]
    #[repr(u32)]
    enum DeclKind {
        AccessSpec = 0,
        Block = 1,
        Captured = 2,
        ClassScopeFunctionSpecialization = 3,
        Empty = 4,
        Export = 5,
        ExternCContext = 6,
        FileScopeAsm = 7,
        Friend = 8,
        FriendTemplate = 9,
        Import = 10,
        LifetimeExtendedTemporary = 11,
        LinkageSpec = 12,
        Using = 13,
        UsingEnum = 14,
        firstBaseUsing = 13,
        lastBaseUsing = 14,
        Label = 15,
        Namespace = 16,
        NamespaceAlias = 17,
        ObjCCompatibleAlias = 18,
        ObjCCategory = 19,
        ObjCCategoryImpl = 20,
        ObjCImplementation = 21,
        firstObjCImpl = 20,
        lastObjCImpl = 21,
        ObjCInterface = 22,
        ObjCProtocol = 23,
        firstObjCContainer = 19,
        lastObjCContainer = 23,
        ObjCMethod = 24,
        ObjCProperty = 25,
        BuiltinTemplate = 26,
        Concept = 27,
        ClassTemplate = 28,
        FunctionTemplate = 29,
        TypeAliasTemplate = 30,
        VarTemplate = 31,
        firstRedeclarableTemplate = 28,
        lastRedeclarableTemplate = 31,
        TemplateTemplateParm = 32,
        firstTemplate = 26,
        lastTemplate = 32,
        Enum = 33,
        Record = 34,
        CXXRecord = 35,
        ClassTemplateSpecialization = 36,
        ClassTemplatePartialSpecialization = 37,
        firstClassTemplateSpecialization = 36,
        lastClassTemplateSpecialization = 37,
        firstCXXRecord = 35,
        lastCXXRecord = 37,
        firstRecord = 34,
        lastRecord = 37,
        firstTag = 33,
        lastTag = 37,
        TemplateTypeParm = 38,
        ObjCTypeParam = 39,
        TypeAlias = 40,
        Typedef = 41,
        firstTypedefName = 39,
        lastTypedefName = 41,
        UnresolvedUsingTypename = 42,
        firstType = 33,
        lastType = 42,
        UnresolvedUsingIfExists = 43,
        UsingDirective = 44,
        UsingPack = 45,
        UsingShadow = 46,
        ConstructorUsingShadow = 47,
        firstUsingShadow = 46,
        lastUsingShadow = 47,
        Binding = 48,
        Field = 49,
        ObjCAtDefsField = 50,
        ObjCIvar = 51,
        firstField = 49,
        lastField = 51,
        Function = 52,
        CXXDeductionGuide = 53,
        CXXMethod = 54,
        CXXConstructor = 55,
        CXXConversion = 56,
        CXXDestructor = 57,
        firstCXXMethod = 54,
        lastCXXMethod = 57,
        firstFunction = 52,
        lastFunction = 57,
        MSProperty = 58,
        NonTypeTemplateParm = 59,
        Var = 60,
        Decomposition = 61,
        ImplicitParam = 62,
        OMPCapturedExpr = 63,
        ParmVar = 64,
        VarTemplateSpecialization = 65,
        VarTemplatePartialSpecialization = 66,
        firstVarTemplateSpecialization = 65,
        lastVarTemplateSpecialization = 66,
        firstVar = 60,
        lastVar = 66,
        firstDeclarator = 49,
        lastDeclarator = 66,
        EnumConstant = 67,
        IndirectField = 68,
        MSGuid = 69,
        OMPDeclareMapper = 70,
        OMPDeclareReduction = 71,
        TemplateParamObject = 72,
        UnnamedGlobalConstant = 73,
        UnresolvedUsingValue = 74,
        firstValue = 48,
        lastValue = 74,
        firstNamed = 13,
        lastNamed = 74,
        OMPAllocate = 75,
        OMPRequires = 76,
        OMPThreadPrivate = 77,
        ObjCPropertyImpl = 78,
        PragmaComment = 79,
        PragmaDetectMismatch = 80,
        RequiresExprBody = 81,
        StaticAssert = 82,
        TranslationUnit = 83,
        firstDecl = 0,
        lastDecl = 83,
    }

    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/FieldDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/FunctionDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCInterfaceDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCMethodDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/ObjCProtocolDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/RecordDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypeDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypedefDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/TypedefNameDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
        type FieldDecl<'ctx> = crate::ffi::clang::ast::decl::field_decl::FieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
        type FunctionDecl<'ctx> = crate::ffi::clang::ast::decl::function_decl::FunctionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_interface_decl"]
        #[cxx_name = "ObjCInterfaceDecl"]
        type ObjcInterfaceDecl<'ctx> = crate::ffi::clang::ast::decl::objc_interface_decl::ObjcInterfaceDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_method_decl"]
        #[cxx_name = "ObjCMethodDecl"]
        type ObjcMethodDecl<'ctx> = crate::ffi::clang::ast::decl::objc_method_decl::ObjcMethodDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::objc_protocol_decl"]
        #[cxx_name = "ObjCProtocolDecl"]
        type ObjcProtocolDecl<'ctx> = crate::ffi::clang::ast::decl::objc_protocol_decl::ObjcProtocolDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::record_decl"]
        type RecordDecl<'ctx> = crate::ffi::clang::ast::decl::record_decl::RecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_decl"]
        type TypedefDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_decl::TypedefDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
        type TypedefNameDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl<'ctx>;

        type DeclKind;
    }

    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        fn get_kind(This: &Decl) -> DeclKind;
    }

    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        fn cast_as_field_decl<'ctx>(This: &Decl<'ctx>) -> *const FieldDecl<'ctx>;

        fn cast_as_function_decl<'ctx>(This: &Decl<'ctx>) -> *const FunctionDecl<'ctx>;

        fn cast_as_named_decl<'ctx>(This: &Decl<'ctx>) -> *const NamedDecl<'ctx>;

        fn cast_as_objc_interface_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjcInterfaceDecl<'ctx>;

        fn cast_as_objc_method_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjcMethodDecl<'ctx>;

        fn cast_as_objc_protocol_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjcProtocolDecl<'ctx>;

        fn cast_as_record_decl<'ctx>(This: &Decl<'ctx>) -> *const RecordDecl<'ctx>;

        fn cast_as_type_decl<'ctx>(This: &Decl<'ctx>) -> *const TypeDecl<'ctx>;

        fn cast_as_typedef_name_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefNameDecl<'ctx>;

        fn cast_as_typedef_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefDecl<'ctx>;
    }
}
pub use self::ffi::DeclKind as Kind;
pub(crate) use self::ffi::*;
