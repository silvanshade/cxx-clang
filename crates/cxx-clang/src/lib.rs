mod abi;
mod ffi {
    pub(crate) mod clang {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod decl;
        }
        pub(crate) mod basic {
            pub(crate) mod module;
        }
        pub(crate) mod lex {
            pub(crate) mod macro_info;
        }
    }
}
mod gen {
    pub(crate) mod clang {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod decl;
        }
        pub(crate) mod basic {
            pub(crate) mod module;
        }
        pub(crate) mod lex {
            pub(crate) mod macro_info;
        }
    }
}
pub mod clang {
    pub mod ast {
        pub mod ast_context {
            pub use crate::ffi::clang::ast::ast_context::AstContext;
        }
        pub mod decl {
            pub mod access_spec_decl {
                pub use crate::ffi::clang::ast::decl::access_spec_decl::AccessSpecDecl;
            }
            pub mod base_using_decl {
                pub use crate::ffi::clang::ast::decl::base_using_decl::BaseUsingDecl;
            }
            pub mod binding_decl {
                pub use crate::ffi::clang::ast::decl::binding_decl::BindingDecl;
            }
            pub mod block_decl {
                pub use crate::ffi::clang::ast::decl::block_decl::BlockDecl;
            }
            pub mod builtin_template_decl {
                pub use crate::ffi::clang::ast::decl::builtin_template_decl::BuiltinTemplateDecl;
            }
            pub mod captured_decl {
                pub use crate::ffi::clang::ast::decl::captured_decl::CapturedDecl;
            }
            pub mod class_scope_function_specialization_decl {
                pub use crate::ffi::clang::ast::decl::class_scope_function_specialization_decl::ClassScopeFunctionSpecializationDecl;
            }
            pub mod class_template_decl {
                pub use crate::ffi::clang::ast::decl::class_template_decl::ClassTemplateDecl;
            }
            pub mod class_template_partial_specialization_decl {
                pub use crate::ffi::clang::ast::decl::class_template_partial_specialization_decl::ClassTemplatePartialSpecializationDecl;
            }
            pub mod class_template_specialization_decl {
                pub use crate::ffi::clang::ast::decl::class_template_specialization_decl::ClassTemplateSpecializationDecl;
            }
            pub mod concept_decl {
                pub use crate::ffi::clang::ast::decl::concept_decl::ConceptDecl;
            }
            pub mod constructor_using_shadow_decl {
                pub use crate::ffi::clang::ast::decl::constructor_using_shadow_decl::ConstructorUsingShadowDecl;
            }
            pub mod cxx_constructor_decl {
                pub use crate::ffi::clang::ast::decl::cxx_constructor_decl::CxxConstructorDecl;
            }
            pub mod cxx_conversion_decl {
                pub use crate::ffi::clang::ast::decl::cxx_conversion_decl::CxxConversionDecl;
            }
            pub mod cxx_deduction_guide_decl {
                pub use crate::ffi::clang::ast::decl::cxx_deduction_guide_decl::CxxDeductionGuideDecl;
            }
            pub mod cxx_destructor_decl {
                pub use crate::ffi::clang::ast::decl::cxx_destructor_decl::CxxDestructorDecl;
            }
            pub mod cxx_method_decl {
                pub use crate::ffi::clang::ast::decl::cxx_method_decl::CxxMethodDecl;
            }
            pub mod cxx_record_decl {
                pub use crate::ffi::clang::ast::decl::cxx_record_decl::CxxRecordDecl;
            }
            pub mod decl_context {
                pub use crate::ffi::clang::ast::decl::decl_context::DeclContext;
            }
            pub mod declarator_decl {
                pub use crate::ffi::clang::ast::decl::declarator_decl::DeclaratorDecl;
            }
            pub mod decomposition_decl {
                pub use crate::ffi::clang::ast::decl::decomposition_decl::DecompositionDecl;
            }
            pub mod empty_decl {
                pub use crate::ffi::clang::ast::decl::empty_decl::EmptyDecl;
            }
            pub mod enum_constant_decl {
                pub use crate::ffi::clang::ast::decl::enum_constant_decl::EnumConstantDecl;
            }
            pub mod enum_decl {
                pub use crate::ffi::clang::ast::decl::enum_decl::EnumDecl;
            }
            pub mod export_decl {
                pub use crate::ffi::clang::ast::decl::export_decl::ExportDecl;
            }
            pub mod extern_c_context_decl {
                pub use crate::ffi::clang::ast::decl::extern_c_context_decl::ExternCContextDecl;
            }
            pub mod field_decl {
                pub use crate::ffi::clang::ast::decl::field_decl::FieldDecl;
            }
            pub mod file_scope_asm_decl {
                pub use crate::ffi::clang::ast::decl::file_scope_asm_decl::FileScopeAsmDecl;
            }
            pub mod friend_decl {
                pub use crate::ffi::clang::ast::decl::friend_decl::FriendDecl;
            }
            pub mod friend_template_decl {
                pub use crate::ffi::clang::ast::decl::friend_template_decl::FriendTemplateDecl;
            }
            pub mod function_decl {
                pub use crate::ffi::clang::ast::decl::function_decl::FunctionDecl;
            }
            pub mod function_template_decl {
                pub use crate::ffi::clang::ast::decl::function_template_decl::FunctionTemplateDecl;
            }
            pub mod implicit_param_decl {
                pub use crate::ffi::clang::ast::decl::implicit_param_decl::ImplicitParamDecl;
            }
            pub mod import_decl {
                pub use crate::ffi::clang::ast::decl::import_decl::ImportDecl;
            }
            pub mod indirect_field_decl {
                pub use crate::ffi::clang::ast::decl::indirect_field_decl::IndirectFieldDecl;
            }
            pub mod label_decl {
                pub use crate::ffi::clang::ast::decl::label_decl::LabelDecl;
            }
            pub mod lifetime_extended_temporary_decl {
                pub use crate::ffi::clang::ast::decl::lifetime_extended_temporary_decl::LifetimeExtendedTemporaryDecl;
            }
            pub mod linkage_spec_decl {
                pub use crate::ffi::clang::ast::decl::linkage_spec_decl::LinkageSpecDecl;
            }
            pub mod ms_guid_decl {
                pub use crate::ffi::clang::ast::decl::ms_guid_decl::MsGuidDecl;
            }
            pub mod ms_property_decl {
                pub use crate::ffi::clang::ast::decl::ms_property_decl::MsPropertyDecl;
            }
            pub mod named_decl {
                pub use crate::ffi::clang::ast::decl::named_decl::NamedDecl;
            }
            pub mod namespace_alias_decl {
                pub use crate::ffi::clang::ast::decl::namespace_alias_decl::NamespaceAliasDecl;
            }
            pub mod namespace_decl {
                pub use crate::ffi::clang::ast::decl::namespace_decl::NamespaceDecl;
            }
            pub mod non_type_template_parm_decl {
                pub use crate::ffi::clang::ast::decl::non_type_template_parm_decl::NonTypeTemplateParmDecl;
            }
            pub mod obj_c_at_defs_field_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_at_defs_field_decl::ObjCAtDefsFieldDecl;
            }
            pub mod obj_c_category_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_category_decl::ObjCCategoryDecl;
            }
            pub mod obj_c_category_impl_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_category_impl_decl::ObjCCategoryImplDecl;
            }
            pub mod obj_c_compatible_alias_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_compatible_alias_decl::ObjCCompatibleAliasDecl;
            }
            pub mod obj_c_container_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_container_decl::ObjCContainerDecl;
            }
            pub mod obj_c_impl_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_impl_decl::ObjCImplDecl;
            }
            pub mod obj_c_implementation_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_implementation_decl::ObjCImplementationDecl;
            }
            pub mod obj_c_interface_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_interface_decl::ObjCInterfaceDecl;
            }
            pub mod obj_c_ivar_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_ivar_decl::ObjCIvarDecl;
            }
            pub mod obj_c_method_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_method_decl::ObjCMethodDecl;
            }
            pub mod obj_c_property_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_property_decl::ObjCPropertyDecl;
            }
            pub mod obj_c_property_impl_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_property_impl_decl::ObjCPropertyImplDecl;
            }
            pub mod obj_c_protocol_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_protocol_decl::ObjCProtocolDecl;
            }
            pub mod obj_c_type_param_decl {
                pub use crate::ffi::clang::ast::decl::obj_c_type_param_decl::ObjCTypeParamDecl;
            }
            pub mod omp_allocate_decl {
                pub use crate::ffi::clang::ast::decl::omp_allocate_decl::OmpAllocateDecl;
            }
            pub mod omp_captured_expr_decl {
                pub use crate::ffi::clang::ast::decl::omp_captured_expr_decl::OmpCapturedExprDecl;
            }
            pub mod omp_declarative_directive_decl {
                pub use crate::ffi::clang::ast::decl::omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl;
            }
            pub mod omp_declarative_directive_value_decl {
                pub use crate::ffi::clang::ast::decl::omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl;
            }
            pub mod omp_declare_mapper_decl {
                pub use crate::ffi::clang::ast::decl::omp_declare_mapper_decl::OmpDeclareMapperDecl;
            }
            pub mod omp_declare_reduction_decl {
                pub use crate::ffi::clang::ast::decl::omp_declare_reduction_decl::OmpDeclareReductionDecl;
            }
            pub mod omp_requires_decl {
                pub use crate::ffi::clang::ast::decl::omp_requires_decl::OmpRequiresDecl;
            }
            pub mod omp_thread_private_decl {
                pub use crate::ffi::clang::ast::decl::omp_thread_private_decl::OmpThreadPrivateDecl;
            }
            pub mod parm_var_decl {
                pub use crate::ffi::clang::ast::decl::parm_var_decl::ParmVarDecl;
            }
            pub mod pragma_comment_decl {
                pub use crate::ffi::clang::ast::decl::pragma_comment_decl::PragmaCommentDecl;
            }
            pub mod pragma_detect_mismatch_decl {
                pub use crate::ffi::clang::ast::decl::pragma_detect_mismatch_decl::PragmaDetectMismatchDecl;
            }
            pub mod record_decl {
                pub use crate::ffi::clang::ast::decl::record_decl::RecordDecl;
            }
            pub mod requires_expr_body_decl {
                pub use crate::ffi::clang::ast::decl::requires_expr_body_decl::RequiresExprBodyDecl;
            }
            pub mod static_assert_decl {
                pub use crate::ffi::clang::ast::decl::static_assert_decl::StaticAssertDecl;
            }
            pub mod tag_decl {
                pub use crate::ffi::clang::ast::decl::tag_decl::TagDecl;
            }
            pub mod template_decl {
                pub use crate::ffi::clang::ast::decl::template_decl::TemplateDecl;
            }
            pub mod template_param_object_decl {
                pub use crate::ffi::clang::ast::decl::template_param_object_decl::TemplateParamObjectDecl;
            }
            pub mod template_template_parm_decl {
                pub use crate::ffi::clang::ast::decl::template_template_parm_decl::TemplateTemplateParmDecl;
            }
            pub mod template_type_parm_decl {
                pub use crate::ffi::clang::ast::decl::template_type_parm_decl::TemplateTypeParmDecl;
            }
            pub mod translation_unit_decl {
                pub use crate::ffi::clang::ast::decl::translation_unit_decl::TranslationUnitDecl;
            }
            pub mod type_alias_decl {
                pub use crate::ffi::clang::ast::decl::type_alias_decl::TypeAliasDecl;
            }
            pub mod type_alias_template_decl {
                pub use crate::ffi::clang::ast::decl::type_alias_template_decl::TypeAliasTemplateDecl;
            }
            pub mod type_decl {
                pub use crate::ffi::clang::ast::decl::type_decl::TypeDecl;
            }
            pub mod typedef_decl {
                pub use crate::ffi::clang::ast::decl::typedef_decl::TypedefDecl;
            }
            pub mod typedef_name_decl {
                pub use crate::ffi::clang::ast::decl::typedef_name_decl::TypedefNameDecl;
            }
            pub mod unnamed_global_constant_decl {
                pub use crate::ffi::clang::ast::decl::unnamed_global_constant_decl::UnnamedGlobalConstantDecl;
            }
            pub mod unresolved_using_if_exists_decl {
                pub use crate::ffi::clang::ast::decl::unresolved_using_if_exists_decl::UnresolvedUsingIfExistsDecl;
            }
            pub mod unresolved_using_typename_decl {
                pub use crate::ffi::clang::ast::decl::unresolved_using_typename_decl::UnresolvedUsingTypenameDecl;
            }
            pub mod unresolved_using_value_decl {
                pub use crate::ffi::clang::ast::decl::unresolved_using_value_decl::UnresolvedUsingValueDecl;
            }
            pub mod using_decl {
                pub use crate::ffi::clang::ast::decl::using_decl::UsingDecl;
            }
            pub mod using_directive_decl {
                pub use crate::ffi::clang::ast::decl::using_directive_decl::UsingDirectiveDecl;
            }
            pub mod using_enum_decl {
                pub use crate::ffi::clang::ast::decl::using_enum_decl::UsingEnumDecl;
            }
            pub mod using_pack_decl {
                pub use crate::ffi::clang::ast::decl::using_pack_decl::UsingPackDecl;
            }
            pub mod using_shadow_decl {
                pub use crate::ffi::clang::ast::decl::using_shadow_decl::UsingShadowDecl;
            }
            pub mod value_decl {
                pub use crate::ffi::clang::ast::decl::value_decl::ValueDecl;
            }
            pub mod var_decl {
                pub use crate::ffi::clang::ast::decl::var_decl::VarDecl;
            }
            pub mod var_template_decl {
                pub use crate::ffi::clang::ast::decl::var_template_decl::VarTemplateDecl;
            }
            pub mod var_template_specialization_decl {
                pub use crate::ffi::clang::ast::decl::var_template_specialization_decl::VarTemplateSpecializationDecl;
            }
            pub use crate::ffi::clang::ast::decl::{Decl, Kind};
        }
    }
    pub mod basic {
        pub mod module {
            pub use crate::ffi::clang::basic::module::Module;
        }
    }
    pub mod lex {
        pub mod macro_info {
            pub mod module_macro {
                pub use crate::ffi::clang::lex::macro_info::module_macro::ModuleMacro;
            }
            pub use crate::ffi::clang::lex::macro_info::MacroInfo;
        }
    }
    pub use crate::ffi::clang::{
        ast::{
            ast_context::AstContext,
            decl::{
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
                decl_context::DeclContext,
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
                omp_allocate_decl::OmpAllocateDecl,
                omp_captured_expr_decl::OmpCapturedExprDecl,
                omp_declarative_directive_decl::OmpDeclarativeDirectiveDecl,
                omp_declarative_directive_value_decl::OmpDeclarativeDirectiveValueDecl,
                omp_declare_mapper_decl::OmpDeclareMapperDecl,
                omp_declare_reduction_decl::OmpDeclareReductionDecl,
                omp_requires_decl::OmpRequiresDecl,
                omp_thread_private_decl::OmpThreadPrivateDecl,
                parm_var_decl::ParmVarDecl,
                pragma_comment_decl::PragmaCommentDecl,
                pragma_detect_mismatch_decl::PragmaDetectMismatchDecl,
                record_decl::RecordDecl,
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
                Decl,
                Kind as DeclKind,
            },
        },
        basic::module::Module,
        lex::macro_info::{module_macro::ModuleMacro, MacroInfo},
    };
}
