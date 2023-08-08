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
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/AccessSpecDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/BaseUsingDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/BindingDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/BlockDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/BuiltinTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CapturedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassScopeFunctionSpecializationDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassTemplatePartialSpecializationDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ClassTemplateSpecializationDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ConceptDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ConstructorUsingShadowDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXConstructorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXConversionDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXDeductionGuideDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXDestructorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXMethodDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/CXXRecordDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DeclaratorDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/DecompositionDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/EmptyDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/EnumConstantDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/EnumDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ExportDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ExternCContextDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FieldDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FileScopeAsmDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FriendDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FriendTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FunctionDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/FunctionTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ImplicitParamDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ImportDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/IndirectFieldDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/LabelDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/LifetimeExtendedTemporaryDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/LinkageSpecDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/MSGuidDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/MSPropertyDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamespaceAliasDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NamespaceDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/NonTypeTemplateParmDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCAtDefsFieldDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCCategoryDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCCategoryImplDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCCompatibleAliasDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCContainerDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCImplDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCImplementationDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCInterfaceDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCIvarDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCMethodDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCPropertyDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCPropertyImplDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCProtocolDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ObjCTypeParamDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPAllocateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPCapturedExprDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclarativeDirectiveValueDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclareMapperDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPDeclareReductionDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPRequiresDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/OMPThreadPrivateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ParmVarDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/PragmaCommentDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/PragmaDetectMismatchDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RecordDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RedeclarableTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/RequiresExprBodyDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/StaticAssertDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TagDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateParamObjectDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateTemplateParmDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TemplateTypeParmDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TranslationUnitDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypeAliasDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypeAliasTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypeDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypedefDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/TypedefNameDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnnamedGlobalConstantDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnresolvedUsingIfExistsDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnresolvedUsingTypenameDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UnresolvedUsingValueDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingDirectiveDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingEnumDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingPackDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/UsingShadowDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/ValueDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarTemplateDecl.hxx");
        include!("cxx-clang-auto/cxx/include/clang/AST/Decl/VarTemplateSpecializationDecl.hxx");

        #[namespace = "cxx_clang::clang::ast::decl::access_spec_decl"]
        type AccessSpecDecl<'ctx> = crate::ffi::clang::ast::decl::access_spec_decl::AccessSpecDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::base_using_decl"]
        type BaseUsingDecl<'ctx> = crate::ffi::clang::ast::decl::base_using_decl::BaseUsingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::binding_decl"]
        type BindingDecl<'ctx> = crate::ffi::clang::ast::decl::binding_decl::BindingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::block_decl"]
        type BlockDecl<'ctx> = crate::ffi::clang::ast::decl::block_decl::BlockDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::builtin_template_decl"]
        type BuiltinTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::builtin_template_decl::BuiltinTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::captured_decl"]
        type CapturedDecl<'ctx> = crate::ffi::clang::ast::decl::captured_decl::CapturedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_scope_function_specialization_decl"]
        type ClassScopeFunctionSpecializationDecl<'ctx> = crate::ffi::clang::ast::decl::class_scope_function_specialization_decl::ClassScopeFunctionSpecializationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_template_decl"]
        type ClassTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::class_template_decl::ClassTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_template_partial_specialization_decl"]
        type ClassTemplatePartialSpecializationDecl<'ctx> = crate::ffi::clang::ast::decl::class_template_partial_specialization_decl::ClassTemplatePartialSpecializationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::class_template_specialization_decl"]
        type ClassTemplateSpecializationDecl<'ctx> =
            crate::ffi::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::concept_decl"]
        type ConceptDecl<'ctx> = crate::ffi::clang::ast::decl::concept_decl::ConceptDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::constructor_using_shadow_decl"]
        type ConstructorUsingShadowDecl<'ctx> =
            crate::ffi::clang::ast::decl::constructor_using_shadow_decl::ConstructorUsingShadowDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_constructor_decl"]
        #[cxx_name = "CXXConstructorDecl"]
        type CxxConstructorDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_constructor_decl::CxxConstructorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_conversion_decl"]
        #[cxx_name = "CXXConversionDecl"]
        type CxxConversionDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_conversion_decl::CxxConversionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_deduction_guide_decl"]
        #[cxx_name = "CXXDeductionGuideDecl"]
        type CxxDeductionGuideDecl<'ctx> =
            crate::ffi::clang::ast::decl::cxx_deduction_guide_decl::CxxDeductionGuideDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_destructor_decl"]
        #[cxx_name = "CXXDestructorDecl"]
        type CxxDestructorDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_destructor_decl::CxxDestructorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_method_decl"]
        #[cxx_name = "CXXMethodDecl"]
        type CxxMethodDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::cxx_record_decl"]
        #[cxx_name = "CXXRecordDecl"]
        type CxxRecordDecl<'ctx> = crate::ffi::clang::ast::decl::cxx_record_decl::CxxRecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl"]
        type Decl<'ctx> = crate::ffi::clang::ast::decl::Decl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::declarator_decl"]
        type DeclaratorDecl<'ctx> = crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::decomposition_decl"]
        type DecompositionDecl<'ctx> = crate::ffi::clang::ast::decl::decomposition_decl::DecompositionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::empty_decl"]
        type EmptyDecl<'ctx> = crate::ffi::clang::ast::decl::empty_decl::EmptyDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::enum_constant_decl"]
        type EnumConstantDecl<'ctx> = crate::ffi::clang::ast::decl::enum_constant_decl::EnumConstantDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::enum_decl"]
        type EnumDecl<'ctx> = crate::ffi::clang::ast::decl::enum_decl::EnumDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::export_decl"]
        type ExportDecl<'ctx> = crate::ffi::clang::ast::decl::export_decl::ExportDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::extern_c_context_decl"]
        type ExternCContextDecl<'ctx> = crate::ffi::clang::ast::decl::extern_c_context_decl::ExternCContextDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::field_decl"]
        type FieldDecl<'ctx> = crate::ffi::clang::ast::decl::field_decl::FieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::file_scope_asm_decl"]
        type FileScopeAsmDecl<'ctx> = crate::ffi::clang::ast::decl::file_scope_asm_decl::FileScopeAsmDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::friend_decl"]
        type FriendDecl<'ctx> = crate::ffi::clang::ast::decl::friend_decl::FriendDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::friend_template_decl"]
        type FriendTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::friend_template_decl::FriendTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_decl"]
        type FunctionDecl<'ctx> = crate::ffi::clang::ast::decl::function_decl::FunctionDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::function_template_decl"]
        type FunctionTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::function_template_decl::FunctionTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::implicit_param_decl"]
        type ImplicitParamDecl<'ctx> = crate::ffi::clang::ast::decl::implicit_param_decl::ImplicitParamDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::import_decl"]
        type ImportDecl<'ctx> = crate::ffi::clang::ast::decl::import_decl::ImportDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::indirect_field_decl"]
        type IndirectFieldDecl<'ctx> = crate::ffi::clang::ast::decl::indirect_field_decl::IndirectFieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::label_decl"]
        type LabelDecl<'ctx> = crate::ffi::clang::ast::decl::label_decl::LabelDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::lifetime_extended_temporary_decl"]
        type LifetimeExtendedTemporaryDecl<'ctx> =
            crate::ffi::clang::ast::decl::lifetime_extended_temporary_decl::LifetimeExtendedTemporaryDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::linkage_spec_decl"]
        type LinkageSpecDecl<'ctx> = crate::ffi::clang::ast::decl::linkage_spec_decl::LinkageSpecDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::ms_guid_decl"]
        #[cxx_name = "MSGuidDecl"]
        type MsGuidDecl<'ctx> = crate::ffi::clang::ast::decl::ms_guid_decl::MsGuidDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::ms_property_decl"]
        #[cxx_name = "MSPropertyDecl"]
        type MsPropertyDecl<'ctx> = crate::ffi::clang::ast::decl::ms_property_decl::MsPropertyDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        type NamedDecl<'ctx> = crate::ffi::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::namespace_alias_decl"]
        type NamespaceAliasDecl<'ctx> = crate::ffi::clang::ast::decl::namespace_alias_decl::NamespaceAliasDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::namespace_decl"]
        type NamespaceDecl<'ctx> = crate::ffi::clang::ast::decl::namespace_decl::NamespaceDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::non_type_template_parm_decl"]
        type NonTypeTemplateParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::non_type_template_parm_decl::NonTypeTemplateParmDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_at_defs_field_decl"]
        type ObjCAtDefsFieldDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_at_defs_field_decl::ObjCAtDefsFieldDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_category_decl"]
        type ObjCCategoryDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_category_decl::ObjCCategoryDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_category_impl_decl"]
        type ObjCCategoryImplDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_category_impl_decl::ObjCCategoryImplDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_compatible_alias_decl"]
        type ObjCCompatibleAliasDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_compatible_alias_decl::ObjCCompatibleAliasDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_container_decl"]
        type ObjCContainerDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_impl_decl"]
        type ObjCImplDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_implementation_decl"]
        type ObjCImplementationDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_implementation_decl::ObjCImplementationDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_interface_decl"]
        type ObjCInterfaceDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_interface_decl::ObjCInterfaceDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_ivar_decl"]
        type ObjCIvarDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_ivar_decl::ObjCIvarDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_method_decl"]
        type ObjCMethodDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_method_decl::ObjCMethodDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_decl"]
        type ObjCPropertyDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_property_decl::ObjCPropertyDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_property_impl_decl"]
        type ObjCPropertyImplDecl<'ctx> =
            crate::ffi::clang::ast::decl::obj_c_property_impl_decl::ObjCPropertyImplDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_protocol_decl"]
        type ObjCProtocolDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_protocol_decl::ObjCProtocolDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::obj_c_type_param_decl"]
        type ObjCTypeParamDecl<'ctx> = crate::ffi::clang::ast::decl::obj_c_type_param_decl::ObjCTypeParamDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_allocate_decl"]
        // #[cxx_name = "OMPAllocateDecl"]
        // type OmpAllocateDecl<'ctx> = crate::ffi::clang::ast::decl::omp_allocate_decl::OmpAllocateDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_captured_expr_decl"]
        // #[cxx_name = "OMPCapturedExprDecl"]
        // type OmpCapturedExprDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_captured_expr_decl::OmpCapturedExprDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_decl"]
        // #[cxx_name = "OMPDeclarativeDirectiveDecl"]
        // type OmpDeclarativeDirectiveDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_declarative_directive_value_decl"]
        // #[cxx_name = "OMPDeclarativeDirectiveValueDecl"]
        // type OmpDeclarativeDirectiveValueDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_declare_mapper_decl"]
        // #[cxx_name = "OMPDeclareMapperDecl"]
        // type OmpDeclareMapperDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_declare_mapper_decl::OmpDeclareMapperDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_declare_reduction_decl"]
        // #[cxx_name = "OMPDeclareReductionDecl"]
        // type OmpDeclareReductionDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_declare_reduction_decl::OmpDeclareReductionDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_requires_decl"]
        // #[cxx_name = "OMPRequiresDecl"]
        // type OmpRequiresDecl<'ctx> = crate::ffi::clang::ast::decl::omp_requires_decl::OmpRequiresDecl<'ctx>;

        // #[namespace = "cxx_clang::clang::ast::decl::omp_thread_private_decl"]
        // #[cxx_name = "OMPThreadPrivateDecl"]
        // type OmpThreadPrivateDecl<'ctx> =
        //     crate::ffi::clang::ast::decl::omp_thread_private_decl::OmpThreadPrivateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::parm_var_decl"]
        type ParmVarDecl<'ctx> = crate::ffi::clang::ast::decl::parm_var_decl::ParmVarDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::pragma_comment_decl"]
        type PragmaCommentDecl<'ctx> = crate::ffi::clang::ast::decl::pragma_comment_decl::PragmaCommentDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::pragma_detect_mismatch_decl"]
        type PragmaDetectMismatchDecl<'ctx> =
            crate::ffi::clang::ast::decl::pragma_detect_mismatch_decl::PragmaDetectMismatchDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::record_decl"]
        type RecordDecl<'ctx> = crate::ffi::clang::ast::decl::record_decl::RecordDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::redeclarable_template_decl"]
        type RedeclarableTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::redeclarable_template_decl::RedeclarableTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::requires_expr_body_decl"]
        type RequiresExprBodyDecl<'ctx> =
            crate::ffi::clang::ast::decl::requires_expr_body_decl::RequiresExprBodyDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::static_assert_decl"]
        type StaticAssertDecl<'ctx> = crate::ffi::clang::ast::decl::static_assert_decl::StaticAssertDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::tag_decl"]
        type TagDecl<'ctx> = crate::ffi::clang::ast::decl::tag_decl::TagDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_decl"]
        type TemplateDecl<'ctx> = crate::ffi::clang::ast::decl::template_decl::TemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_param_object_decl"]
        type TemplateParamObjectDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_param_object_decl::TemplateParamObjectDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_template_parm_decl"]
        type TemplateTemplateParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_template_parm_decl::TemplateTemplateParmDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::template_type_parm_decl"]
        type TemplateTypeParmDecl<'ctx> =
            crate::ffi::clang::ast::decl::template_type_parm_decl::TemplateTypeParmDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::translation_unit_decl"]
        type TranslationUnitDecl<'ctx> = crate::ffi::clang::ast::decl::translation_unit_decl::TranslationUnitDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_alias_decl"]
        type TypeAliasDecl<'ctx> = crate::ffi::clang::ast::decl::type_alias_decl::TypeAliasDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_alias_template_decl"]
        type TypeAliasTemplateDecl<'ctx> =
            crate::ffi::clang::ast::decl::type_alias_template_decl::TypeAliasTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::type_decl"]
        type TypeDecl<'ctx> = crate::ffi::clang::ast::decl::type_decl::TypeDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_decl"]
        type TypedefDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_decl::TypedefDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::typedef_name_decl"]
        type TypedefNameDecl<'ctx> = crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::unnamed_global_constant_decl"]
        type UnnamedGlobalConstantDecl<'ctx> =
            crate::ffi::clang::ast::decl::unnamed_global_constant_decl::UnnamedGlobalConstantDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_if_exists_decl"]
        type UnresolvedUsingIfExistsDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_typename_decl"]
        type UnresolvedUsingTypenameDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_typename_decl::UnresolvedUsingTypenameDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::unresolved_using_value_decl"]
        type UnresolvedUsingValueDecl<'ctx> =
            crate::ffi::clang::ast::decl::unresolved_using_value_decl::UnresolvedUsingValueDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_decl"]
        type UsingDecl<'ctx> = crate::ffi::clang::ast::decl::using_decl::UsingDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_directive_decl"]
        type UsingDirectiveDecl<'ctx> = crate::ffi::clang::ast::decl::using_directive_decl::UsingDirectiveDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_enum_decl"]
        type UsingEnumDecl<'ctx> = crate::ffi::clang::ast::decl::using_enum_decl::UsingEnumDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_pack_decl"]
        type UsingPackDecl<'ctx> = crate::ffi::clang::ast::decl::using_pack_decl::UsingPackDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::using_shadow_decl"]
        type UsingShadowDecl<'ctx> = crate::ffi::clang::ast::decl::using_shadow_decl::UsingShadowDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::value_decl"]
        type ValueDecl<'ctx> = crate::ffi::clang::ast::decl::value_decl::ValueDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_decl"]
        type VarDecl<'ctx> = crate::ffi::clang::ast::decl::var_decl::VarDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_template_decl"]
        type VarTemplateDecl<'ctx> = crate::ffi::clang::ast::decl::var_template_decl::VarTemplateDecl<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::var_template_specialization_decl"]
        type VarTemplateSpecializationDecl<'ctx> =
            crate::ffi::clang::ast::decl::var_template_specialization_decl::VarTemplateSpecializationDecl<'ctx>;

        type DeclKind;
    }

    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        fn get_kind(This: &Decl) -> DeclKind;
    }

    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        fn cast_as_access_spec_decl<'ctx>(This: &Decl<'ctx>) -> *const AccessSpecDecl<'ctx>;

        fn cast_as_base_using_decl<'ctx>(This: &Decl<'ctx>) -> *const BaseUsingDecl<'ctx>;

        fn cast_as_binding_decl<'ctx>(This: &Decl<'ctx>) -> *const BindingDecl<'ctx>;

        fn cast_as_block_decl<'ctx>(This: &Decl<'ctx>) -> *const BlockDecl<'ctx>;

        fn cast_as_builtin_template_decl<'ctx>(This: &Decl<'ctx>) -> *const BuiltinTemplateDecl<'ctx>;

        fn cast_as_captured_decl<'ctx>(This: &Decl<'ctx>) -> *const CapturedDecl<'ctx>;

        fn cast_as_class_scope_function_specialization_decl<'ctx>(
            This: &Decl<'ctx>,
        ) -> *const ClassScopeFunctionSpecializationDecl<'ctx>;

        fn cast_as_class_template_decl<'ctx>(This: &Decl<'ctx>) -> *const ClassTemplateDecl<'ctx>;

        fn cast_as_class_template_partial_specialization_decl<'ctx>(
            This: &Decl<'ctx>,
        ) -> *const ClassTemplatePartialSpecializationDecl<'ctx>;

        fn cast_as_class_template_specialization_decl<'ctx>(
            This: &Decl<'ctx>,
        ) -> *const ClassTemplateSpecializationDecl<'ctx>;

        fn cast_as_concept_decl<'ctx>(This: &Decl<'ctx>) -> *const ConceptDecl<'ctx>;

        fn cast_as_constructor_using_shadow_decl<'ctx>(This: &Decl<'ctx>) -> *const ConstructorUsingShadowDecl<'ctx>;

        fn cast_as_cxx_constructor_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxConstructorDecl<'ctx>;

        fn cast_as_cxx_conversion_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxConversionDecl<'ctx>;

        fn cast_as_cxx_deduction_guide_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxDeductionGuideDecl<'ctx>;

        fn cast_as_cxx_destructor_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxDestructorDecl<'ctx>;

        fn cast_as_cxx_method_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxMethodDecl<'ctx>;

        fn cast_as_cxx_record_decl<'ctx>(This: &Decl<'ctx>) -> *const CxxRecordDecl<'ctx>;

        fn cast_as_declarator_decl<'ctx>(This: &Decl<'ctx>) -> *const DeclaratorDecl<'ctx>;

        fn cast_as_decomposition_decl<'ctx>(This: &Decl<'ctx>) -> *const DecompositionDecl<'ctx>;

        fn cast_as_empty_decl<'ctx>(This: &Decl<'ctx>) -> *const EmptyDecl<'ctx>;

        fn cast_as_enum_constant_decl<'ctx>(This: &Decl<'ctx>) -> *const EnumConstantDecl<'ctx>;

        fn cast_as_enum_decl<'ctx>(This: &Decl<'ctx>) -> *const EnumDecl<'ctx>;

        fn cast_as_export_decl<'ctx>(This: &Decl<'ctx>) -> *const ExportDecl<'ctx>;

        fn cast_as_extern_c_context_decl<'ctx>(This: &Decl<'ctx>) -> *const ExternCContextDecl<'ctx>;

        fn cast_as_field_decl<'ctx>(This: &Decl<'ctx>) -> *const FieldDecl<'ctx>;

        fn cast_as_file_scope_asm_decl<'ctx>(This: &Decl<'ctx>) -> *const FileScopeAsmDecl<'ctx>;

        fn cast_as_friend_decl<'ctx>(This: &Decl<'ctx>) -> *const FriendDecl<'ctx>;

        fn cast_as_friend_template_decl<'ctx>(This: &Decl<'ctx>) -> *const FriendTemplateDecl<'ctx>;

        fn cast_as_function_decl<'ctx>(This: &Decl<'ctx>) -> *const FunctionDecl<'ctx>;

        fn cast_as_function_template_decl<'ctx>(This: &Decl<'ctx>) -> *const FunctionTemplateDecl<'ctx>;

        fn cast_as_implicit_param_decl<'ctx>(This: &Decl<'ctx>) -> *const ImplicitParamDecl<'ctx>;

        fn cast_as_import_decl<'ctx>(This: &Decl<'ctx>) -> *const ImportDecl<'ctx>;

        fn cast_as_indirect_field_decl<'ctx>(This: &Decl<'ctx>) -> *const IndirectFieldDecl<'ctx>;

        fn cast_as_label_decl<'ctx>(This: &Decl<'ctx>) -> *const LabelDecl<'ctx>;

        fn cast_as_lifetime_extended_temporary_decl<'ctx>(
            This: &Decl<'ctx>,
        ) -> *const LifetimeExtendedTemporaryDecl<'ctx>;

        fn cast_as_linkage_spec_decl<'ctx>(This: &Decl<'ctx>) -> *const LinkageSpecDecl<'ctx>;

        fn cast_as_ms_guid_decl<'ctx>(This: &Decl<'ctx>) -> *const MsGuidDecl<'ctx>;

        fn cast_as_ms_property_decl<'ctx>(This: &Decl<'ctx>) -> *const MsPropertyDecl<'ctx>;

        fn cast_as_named_decl<'ctx>(This: &Decl<'ctx>) -> *const NamedDecl<'ctx>;

        fn cast_as_namespace_alias_decl<'ctx>(This: &Decl<'ctx>) -> *const NamespaceAliasDecl<'ctx>;

        fn cast_as_namespace_decl<'ctx>(This: &Decl<'ctx>) -> *const NamespaceDecl<'ctx>;

        fn cast_as_non_type_template_parm_decl<'ctx>(This: &Decl<'ctx>) -> *const NonTypeTemplateParmDecl<'ctx>;

        fn cast_as_obj_c_at_defs_field_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCAtDefsFieldDecl<'ctx>;

        fn cast_as_obj_c_category_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCCategoryDecl<'ctx>;

        fn cast_as_obj_c_category_impl_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCCategoryImplDecl<'ctx>;

        fn cast_as_obj_c_compatible_alias_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCCompatibleAliasDecl<'ctx>;

        fn cast_as_obj_c_container_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCContainerDecl<'ctx>;

        fn cast_as_obj_c_impl_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCImplDecl<'ctx>;

        fn cast_as_obj_c_implementation_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCImplementationDecl<'ctx>;

        fn cast_as_obj_c_interface_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCInterfaceDecl<'ctx>;

        fn cast_as_obj_c_ivar_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCIvarDecl<'ctx>;

        fn cast_as_obj_c_method_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCMethodDecl<'ctx>;

        fn cast_as_obj_c_property_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCPropertyDecl<'ctx>;

        fn cast_as_obj_c_property_impl_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCPropertyImplDecl<'ctx>;

        fn cast_as_obj_c_protocol_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCProtocolDecl<'ctx>;

        fn cast_as_obj_c_type_param_decl<'ctx>(This: &Decl<'ctx>) -> *const ObjCTypeParamDecl<'ctx>;

        // fn cast_as_omp_allocate_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpAllocateDecl<'ctx>;

        // fn cast_as_omp_captured_expr_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpCapturedExprDecl<'ctx>;

        // fn cast_as_omp_declarative_directive_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpDeclarativeDirectiveDecl<'ctx>;

        // fn cast_as_omp_declarative_directive_value_decl<'ctx>(
        //     This: &Decl<'ctx>,
        // ) -> *const OmpDeclarativeDirectiveValueDecl<'ctx>;

        // fn cast_as_omp_declare_mapper_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpDeclareMapperDecl<'ctx>;

        // fn cast_as_omp_declare_reduction_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpDeclareReductionDecl<'ctx>;

        // fn cast_as_omp_requires_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpRequiresDecl<'ctx>;

        // fn cast_as_omp_thread_private_decl<'ctx>(This: &Decl<'ctx>) -> *const OmpThreadPrivateDecl<'ctx>;

        fn cast_as_parm_var_decl<'ctx>(This: &Decl<'ctx>) -> *const ParmVarDecl<'ctx>;

        fn cast_as_pragma_comment_decl<'ctx>(This: &Decl<'ctx>) -> *const PragmaCommentDecl<'ctx>;

        fn cast_as_pragma_detect_mismatch_decl<'ctx>(This: &Decl<'ctx>) -> *const PragmaDetectMismatchDecl<'ctx>;

        fn cast_as_record_decl<'ctx>(This: &Decl<'ctx>) -> *const RecordDecl<'ctx>;

        fn cast_as_redeclarable_template_decl<'ctx>(This: &Decl<'ctx>) -> *const RedeclarableTemplateDecl<'ctx>;

        fn cast_as_requires_expr_body_decl<'ctx>(This: &Decl<'ctx>) -> *const RequiresExprBodyDecl<'ctx>;

        fn cast_as_static_assert_decl<'ctx>(This: &Decl<'ctx>) -> *const StaticAssertDecl<'ctx>;

        fn cast_as_tag_decl<'ctx>(This: &Decl<'ctx>) -> *const TagDecl<'ctx>;

        fn cast_as_template_decl<'ctx>(This: &Decl<'ctx>) -> *const TemplateDecl<'ctx>;

        fn cast_as_template_param_object_decl<'ctx>(This: &Decl<'ctx>) -> *const TemplateParamObjectDecl<'ctx>;

        fn cast_as_template_template_parm_decl<'ctx>(This: &Decl<'ctx>) -> *const TemplateTemplateParmDecl<'ctx>;

        fn cast_as_template_type_parm_decl<'ctx>(This: &Decl<'ctx>) -> *const TemplateTypeParmDecl<'ctx>;

        fn cast_as_translation_unit_decl<'ctx>(This: &Decl<'ctx>) -> *const TranslationUnitDecl<'ctx>;

        fn cast_as_type_alias_decl<'ctx>(This: &Decl<'ctx>) -> *const TypeAliasDecl<'ctx>;

        fn cast_as_type_alias_template_decl<'ctx>(This: &Decl<'ctx>) -> *const TypeAliasTemplateDecl<'ctx>;

        fn cast_as_type_decl<'ctx>(This: &Decl<'ctx>) -> *const TypeDecl<'ctx>;

        fn cast_as_typedef_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefDecl<'ctx>;

        fn cast_as_typedef_name_decl<'ctx>(This: &Decl<'ctx>) -> *const TypedefNameDecl<'ctx>;

        fn cast_as_unnamed_global_constant_decl<'ctx>(This: &Decl<'ctx>) -> *const UnnamedGlobalConstantDecl<'ctx>;

        fn cast_as_unresolved_using_if_exists_decl<'ctx>(This: &Decl<'ctx>)
        -> *const UnresolvedUsingIfExistsDecl<'ctx>;

        fn cast_as_unresolved_using_typename_decl<'ctx>(This: &Decl<'ctx>) -> *const UnresolvedUsingTypenameDecl<'ctx>;

        fn cast_as_unresolved_using_value_decl<'ctx>(This: &Decl<'ctx>) -> *const UnresolvedUsingValueDecl<'ctx>;

        fn cast_as_using_decl<'ctx>(This: &Decl<'ctx>) -> *const UsingDecl<'ctx>;

        fn cast_as_using_directive_decl<'ctx>(This: &Decl<'ctx>) -> *const UsingDirectiveDecl<'ctx>;

        fn cast_as_using_enum_decl<'ctx>(This: &Decl<'ctx>) -> *const UsingEnumDecl<'ctx>;

        fn cast_as_using_pack_decl<'ctx>(This: &Decl<'ctx>) -> *const UsingPackDecl<'ctx>;

        fn cast_as_using_shadow_decl<'ctx>(This: &Decl<'ctx>) -> *const UsingShadowDecl<'ctx>;

        fn cast_as_value_decl<'ctx>(This: &Decl<'ctx>) -> *const ValueDecl<'ctx>;

        fn cast_as_var_decl<'ctx>(This: &Decl<'ctx>) -> *const VarDecl<'ctx>;

        fn cast_as_var_template_decl<'ctx>(This: &Decl<'ctx>) -> *const VarTemplateDecl<'ctx>;

        fn cast_as_var_template_specialization_decl<'ctx>(
            This: &Decl<'ctx>,
        ) -> *const VarTemplateSpecializationDecl<'ctx>;
    }
}
pub use self::ffi::DeclKind as Kind;
pub(crate) use self::ffi::*;
