pub(crate) mod access_spec_decl;
pub(crate) mod base_using_decl;
pub(crate) mod binding_decl;
pub(crate) mod block_decl;
pub(crate) mod builtin_template_decl;
pub(crate) mod captured_decl;
pub(crate) mod class_scope_function_specialization_decl;
pub(crate) mod class_template_decl;
pub(crate) mod class_template_partial_specialization_decl;
pub(crate) mod class_template_specialization_decl;
pub(crate) mod concept_decl;
pub(crate) mod constructor_using_shadow_decl;
pub(crate) mod cxx_constructor_decl;
pub(crate) mod cxx_conversion_decl;
pub(crate) mod cxx_deduction_guide_decl;
pub(crate) mod cxx_destructor_decl;
pub(crate) mod cxx_method_decl;
pub(crate) mod cxx_record_decl;
pub(crate) mod decl_context;
pub(crate) mod declarator_decl;
pub(crate) mod decomposition_decl;
pub(crate) mod empty_decl;
pub(crate) mod enum_constant_decl;
pub(crate) mod enum_decl;
pub(crate) mod export_decl;
pub(crate) mod extern_c_context_decl;
pub(crate) mod field_decl;
pub(crate) mod file_scope_asm_decl;
pub(crate) mod friend_decl;
pub(crate) mod friend_template_decl;
pub(crate) mod function_decl;
pub(crate) mod function_template_decl;
pub(crate) mod implicit_param_decl;
pub(crate) mod import_decl;
pub(crate) mod indirect_field_decl;
pub(crate) mod label_decl;
pub(crate) mod lifetime_extended_temporary_decl;
pub(crate) mod linkage_spec_decl;
pub(crate) mod ms_guid_decl;
pub(crate) mod ms_property_decl;
pub(crate) mod named_decl;
pub(crate) mod namespace_alias_decl;
pub(crate) mod namespace_decl;
pub(crate) mod non_type_template_parm_decl;
pub(crate) mod obj_c_at_defs_field_decl;
pub(crate) mod obj_c_category_decl;
pub(crate) mod obj_c_category_impl_decl;
pub(crate) mod obj_c_compatible_alias_decl;
pub(crate) mod obj_c_container_decl;
pub(crate) mod obj_c_impl_decl;
pub(crate) mod obj_c_implementation_decl;
pub(crate) mod obj_c_interface_decl;
pub(crate) mod obj_c_ivar_decl;
pub(crate) mod obj_c_method_decl;
pub(crate) mod obj_c_property_decl;
pub(crate) mod obj_c_property_impl_decl;
pub(crate) mod obj_c_protocol_decl;
pub(crate) mod obj_c_type_param_decl;
pub(crate) mod omp_allocate_decl;
pub(crate) mod omp_captured_expr_decl;
pub(crate) mod omp_declarative_directive_decl;
pub(crate) mod omp_declarative_directive_value_decl;
pub(crate) mod omp_declare_mapper_decl;
pub(crate) mod omp_declare_reduction_decl;
pub(crate) mod omp_requires_decl;
pub(crate) mod omp_thread_private_decl;
pub(crate) mod parm_var_decl;
pub(crate) mod pragma_comment_decl;
pub(crate) mod pragma_detect_mismatch_decl;
pub(crate) mod record_decl;
pub(crate) mod redeclarable_template_decl;
pub(crate) mod requires_expr_body_decl;
pub(crate) mod static_assert_decl;
pub(crate) mod tag_decl;
pub(crate) mod template_decl;
pub(crate) mod template_param_object_decl;
pub(crate) mod template_template_parm_decl;
pub(crate) mod template_type_parm_decl;
pub(crate) mod translation_unit_decl;
pub(crate) mod type_alias_decl;
pub(crate) mod type_alias_template_decl;
pub(crate) mod type_decl;
pub(crate) mod typedef_decl;
pub(crate) mod typedef_name_decl;
pub(crate) mod unnamed_global_constant_decl;
pub(crate) mod unresolved_using_if_exists_decl;
pub(crate) mod unresolved_using_typename_decl;
pub(crate) mod unresolved_using_value_decl;
pub(crate) mod using_decl;
pub(crate) mod using_directive_decl;
pub(crate) mod using_enum_decl;
pub(crate) mod using_pack_decl;
pub(crate) mod using_shadow_decl;
pub(crate) mod value_decl;
pub(crate) mod var_decl;
pub(crate) mod var_template_decl;
pub(crate) mod var_template_specialization_decl;

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

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_interface_decl"]
        #[cxx_name = "ObjCInterfaceDecl"]
        type ObjCInterfaceDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_interface_decl::ObjCInterfaceDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_method_decl"]
        #[cxx_name = "ObjCMethodDecl"]
        type ObjCMethodDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_method_decl::ObjCMethodDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_protocol_decl"]
        #[cxx_name = "ObjCProtocolDecl"]
        type ObjCProtocolDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_protocol_decl::ObjCProtocolDecl<'ctx>;

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

        fn cast_as_obj_c_interface_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCInterfaceDecl<'ctx>;

        fn cast_as_obj_c_method_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCMethodDecl<'ctx>;

        fn cast_as_obj_c_protocol_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCProtocolDecl<'ctx>;

        fn cast_as_record_decl<'ctx>(This: &Decl<'ctx>) -> *const RecordDecl<'ctx>;

        fn cast_as_type_decl<'ctx>(This: &Decl<'ctx>) -> *const TypeDecl<'ctx>;

        fn cast_as_typedef_name_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefNameDecl<'ctx>;

        fn cast_as_typedef_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefDecl<'ctx>;
    }
}
pub use self::ffi::DeclKind as Kind;
pub(crate) use self::ffi::*;
