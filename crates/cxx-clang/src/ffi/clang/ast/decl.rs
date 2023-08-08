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

use crate::{
    ffi::clang::ast::decl::{
        access_spec_decl::AccessSpecDecl,
        base_using_decl::BaseUsingDecl,
        binding_decl::BindingDecl,
        block_decl::BlockDecl,
        builtin_template_decl::BuiltinTemplateDecl,
        captured_decl::CapturedDecl,
        class_scope_function_specialization_decl::ClassScopeFunctionSpecializationDecl,
        class_template_decl::ClassTemplateDecl,
        class_template_partial_specialization_decl::ClassTemplatePartialSpecializationDecl,
        class_template_specialization_decl::ClassTemplateSpecializationDecl,
        concept_decl::ConceptDecl,
        constructor_using_shadow_decl::ConstructorUsingShadowDecl,
        cxx_constructor_decl::CxxConstructorDecl,
        cxx_conversion_decl::CxxConversionDecl,
        cxx_deduction_guide_decl::CxxDeductionGuideDecl,
        cxx_destructor_decl::CxxDestructorDecl,
        cxx_method_decl::CxxMethodDecl,
        cxx_record_decl::CxxRecordDecl,
        declarator_decl::DeclaratorDecl,
        decomposition_decl::DecompositionDecl,
        empty_decl::EmptyDecl,
        enum_constant_decl::EnumConstantDecl,
        enum_decl::EnumDecl,
        export_decl::ExportDecl,
        extern_c_context_decl::ExternCContextDecl,
        field_decl::FieldDecl,
        file_scope_asm_decl::FileScopeAsmDecl,
        friend_decl::FriendDecl,
        friend_template_decl::FriendTemplateDecl,
        function_decl::FunctionDecl,
        function_template_decl::FunctionTemplateDecl,
        implicit_param_decl::ImplicitParamDecl,
        import_decl::ImportDecl,
        indirect_field_decl::IndirectFieldDecl,
        label_decl::LabelDecl,
        lifetime_extended_temporary_decl::LifetimeExtendedTemporaryDecl,
        linkage_spec_decl::LinkageSpecDecl,
        ms_guid_decl::MsGuidDecl,
        ms_property_decl::MsPropertyDecl,
        named_decl::NamedDecl,
        namespace_alias_decl::NamespaceAliasDecl,
        namespace_decl::NamespaceDecl,
        non_type_template_parm_decl::NonTypeTemplateParmDecl,
        obj_c_at_defs_field_decl::ObjCAtDefsFieldDecl,
        obj_c_category_decl::ObjCCategoryDecl,
        obj_c_category_impl_decl::ObjCCategoryImplDecl,
        obj_c_compatible_alias_decl::ObjCCompatibleAliasDecl,
        obj_c_container_decl::ObjCContainerDecl,
        obj_c_impl_decl::ObjCImplDecl,
        obj_c_implementation_decl::ObjCImplementationDecl,
        obj_c_interface_decl::ObjCInterfaceDecl,
        obj_c_ivar_decl::ObjCIvarDecl,
        obj_c_method_decl::ObjCMethodDecl,
        obj_c_property_decl::ObjCPropertyDecl,
        obj_c_property_impl_decl::ObjCPropertyImplDecl,
        obj_c_protocol_decl::ObjCProtocolDecl,
        obj_c_type_param_decl::ObjCTypeParamDecl,
        // omp_allocate_decl::OmpAllocateDecl,
        // omp_captured_expr_decl::OmpCapturedExprDecl,
        // omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl,
        // omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl,
        // omp_declare_mapper_decl::OmpDeclareMapperDecl,
        // omp_declare_reduction_decl::OmpDeclareReductionDecl,
        // omp_requires_decl::OmpRequiresDecl,
        // omp_thread_private_decl::OmpThreadPrivateDecl,
        parm_var_decl::ParmVarDecl,
        pragma_comment_decl::PragmaCommentDecl,
        pragma_detect_mismatch_decl::PragmaDetectMismatchDecl,
        record_decl::RecordDecl,
        redeclarable_template_decl::RedeclarableTemplateDecl,
        requires_expr_body_decl::RequiresExprBodyDecl,
        static_assert_decl::StaticAssertDecl,
        tag_decl::TagDecl,
        template_decl::TemplateDecl,
        template_param_object_decl::TemplateParamObjectDecl,
        template_template_parm_decl::TemplateTemplateParmDecl,
        template_type_parm_decl::TemplateTypeParmDecl,
        translation_unit_decl::TranslationUnitDecl,
        type_alias_decl::TypeAliasDecl,
        type_alias_template_decl::TypeAliasTemplateDecl,
        type_decl::TypeDecl,
        typedef_decl::TypedefDecl,
        typedef_name_decl::TypedefNameDecl,
        unnamed_global_constant_decl::UnnamedGlobalConstantDecl,
        unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl,
        unresolved_using_typename_decl::UnresolvedUsingTypenameDecl,
        unresolved_using_value_decl::UnresolvedUsingValueDecl,
        using_decl::UsingDecl,
        using_directive_decl::UsingDirectiveDecl,
        using_enum_decl::UsingEnumDecl,
        using_pack_decl::UsingPackDecl,
        using_shadow_decl::UsingShadowDecl,
        value_decl::ValueDecl,
        var_decl::VarDecl,
        var_template_decl::VarTemplateDecl,
        var_template_specialization_decl::VarTemplateSpecializationDecl,
    },
    gen::clang::ast::decl,
};

pub use crate::{auto::clang::ast::decl::Decl, gen::clang::ast::decl::Kind};

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn dyn_cast<T>(&self) -> Option<&T>
    where
        Self: cxx_llvm::casting::DynCast<T>,
    {
        cxx_llvm::casting::DynCast::dyn_cast(self)
    }

    #[inline]
    pub fn get_kind(&self) -> decl::Kind {
        decl::get_kind(self)
    }

    #[inline]
    fn decl_ptr_into_ref<'t, T: 't>(ptr: *const T) -> Option<&'t T> {
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<AccessSpecDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&AccessSpecDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_access_spec_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<BaseUsingDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&BaseUsingDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_base_using_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<BindingDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&BindingDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_binding_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<BlockDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&BlockDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_block_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<BuiltinTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&BuiltinTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_builtin_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CapturedDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CapturedDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_captured_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ClassScopeFunctionSpecializationDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ClassScopeFunctionSpecializationDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_class_scope_function_specialization_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ClassTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ClassTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_class_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ClassTemplatePartialSpecializationDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ClassTemplatePartialSpecializationDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_class_template_partial_specialization_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ClassTemplateSpecializationDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ClassTemplateSpecializationDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_class_template_specialization_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ConceptDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ConceptDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_concept_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ConstructorUsingShadowDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ConstructorUsingShadowDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_constructor_using_shadow_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxConstructorDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxConstructorDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_constructor_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxConversionDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxConversionDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_conversion_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxDeductionGuideDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxDeductionGuideDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_deduction_guide_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxDestructorDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxDestructorDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_destructor_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxMethodDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxMethodDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_method_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<CxxRecordDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&CxxRecordDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_cxx_record_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<DeclaratorDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&DeclaratorDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_declarator_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<DecompositionDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&DecompositionDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_decomposition_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<EmptyDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&EmptyDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_empty_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<EnumConstantDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&EnumConstantDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_enum_constant_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<EnumDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&EnumDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_enum_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ExportDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ExportDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_export_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ExternCContextDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ExternCContextDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_extern_c_context_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FieldDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FieldDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_field_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FileScopeAsmDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FileScopeAsmDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_file_scope_asm_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FriendDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FriendDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_friend_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FriendTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FriendTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_friend_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FunctionDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FunctionDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_function_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<FunctionTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&FunctionTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_function_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ImplicitParamDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ImplicitParamDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_implicit_param_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ImportDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ImportDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_import_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<IndirectFieldDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&IndirectFieldDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_indirect_field_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<LabelDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&LabelDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_label_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<LifetimeExtendedTemporaryDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&LifetimeExtendedTemporaryDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_lifetime_extended_temporary_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<LinkageSpecDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&LinkageSpecDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_linkage_spec_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<MsGuidDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&MsGuidDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_ms_guid_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<MsPropertyDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&MsPropertyDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_ms_property_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<NamedDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&NamedDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_named_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<NamespaceAliasDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&NamespaceAliasDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_namespace_alias_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<NamespaceDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&NamespaceDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_namespace_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<NonTypeTemplateParmDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&NonTypeTemplateParmDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_non_type_template_parm_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCAtDefsFieldDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCAtDefsFieldDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_at_defs_field_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCCategoryDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCCategoryDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_category_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCCategoryImplDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCCategoryImplDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_category_impl_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCCompatibleAliasDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCCompatibleAliasDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_compatible_alias_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCContainerDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCContainerDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_container_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCImplDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCImplDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_impl_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCImplementationDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCImplementationDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_implementation_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCInterfaceDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCInterfaceDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_interface_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCIvarDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCIvarDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_ivar_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCMethodDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCMethodDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_method_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCPropertyDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCPropertyDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_property_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCPropertyImplDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCPropertyImplDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_property_impl_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCProtocolDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCProtocolDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_protocol_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ObjCTypeParamDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ObjCTypeParamDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_obj_c_type_param_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ParmVarDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ParmVarDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_parm_var_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<PragmaCommentDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&PragmaCommentDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_pragma_comment_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<PragmaDetectMismatchDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&PragmaDetectMismatchDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_pragma_detect_mismatch_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<RecordDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&RecordDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_record_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<RedeclarableTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&RedeclarableTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_redeclarable_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<RequiresExprBodyDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&RequiresExprBodyDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_requires_expr_body_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<StaticAssertDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&StaticAssertDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_static_assert_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TagDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TagDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_tag_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TemplateParamObjectDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TemplateParamObjectDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_template_param_object_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TemplateTemplateParmDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TemplateTemplateParmDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_template_template_parm_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TemplateTypeParmDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TemplateTypeParmDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_template_type_parm_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TranslationUnitDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TranslationUnitDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_translation_unit_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TypeAliasDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TypeAliasDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_type_alias_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TypeAliasTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TypeAliasTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_type_alias_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TypeDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TypeDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_type_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TypedefDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TypedefDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_typedef_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<TypedefNameDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&TypedefNameDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_typedef_name_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UnnamedGlobalConstantDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UnnamedGlobalConstantDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_unnamed_global_constant_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UnresolvedUsingIfExistsDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UnresolvedUsingIfExistsDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_unresolved_using_if_exists_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UnresolvedUsingTypenameDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UnresolvedUsingTypenameDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_unresolved_using_typename_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UnresolvedUsingValueDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UnresolvedUsingValueDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_unresolved_using_value_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UsingDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UsingDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_using_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UsingDirectiveDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UsingDirectiveDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_using_directive_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UsingEnumDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UsingEnumDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_using_enum_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UsingPackDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UsingPackDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_using_pack_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<UsingShadowDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&UsingShadowDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_using_shadow_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<ValueDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&ValueDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_value_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<VarDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&VarDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_var_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<VarTemplateDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&VarTemplateDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_var_template_decl(self))
    }
}

impl<'ctx> cxx_llvm::casting::DynCast<VarTemplateSpecializationDecl<'ctx>> for Decl<'ctx> {
    #[inline]
    fn dyn_cast(&self) -> Option<&VarTemplateSpecializationDecl<'ctx>> {
        Self::decl_ptr_into_ref(decl::cast_as_var_template_specialization_decl(self))
    }
}
