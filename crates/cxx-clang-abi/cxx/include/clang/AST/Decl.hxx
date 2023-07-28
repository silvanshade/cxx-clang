#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/DeclObjC.h"
#include "clang/AST/DeclOpenMP.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/OpenMPClause.h"
#include "llvm/Support/Casting.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using DeclKind = ::clang::Decl::Kind;

namespace cxx_clang::clang::ast::decl {
CXX_MEMORY_ABI_PRELUDE(Decl, ::clang::Decl)
} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_kind(Self const& This [[clang::lifetimebound]]) -> ::clang::Decl::Kind
{
  return This.getKind();
}

} // namespace cxx_clang::clang::ast::decl

namespace cxx_clang::clang::ast::decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_access_spec_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::AccessSpecDecl const*
{
  return llvm::dyn_cast<::clang::AccessSpecDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_base_using_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::BaseUsingDecl const*
{
  return llvm::dyn_cast<::clang::BaseUsingDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_binding_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::BindingDecl const*
{
  return llvm::dyn_cast<::clang::BindingDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_block_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::BlockDecl const*
{
  return llvm::dyn_cast<::clang::BlockDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_builtin_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::BuiltinTemplateDecl const*
{
  return llvm::dyn_cast<::clang::BuiltinTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_captured_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CapturedDecl const*
{
  return llvm::dyn_cast<::clang::CapturedDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_class_scope_function_specialization_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ClassScopeFunctionSpecializationDecl const*
{
  return llvm::dyn_cast<::clang::ClassScopeFunctionSpecializationDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_class_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ClassTemplateDecl const*
{
  return llvm::dyn_cast<::clang::ClassTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_class_template_partial_specialization_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ClassTemplatePartialSpecializationDecl const*
{
  return llvm::dyn_cast<::clang::ClassTemplatePartialSpecializationDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_class_template_specialization_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ClassTemplateSpecializationDecl const*
{
  return llvm::dyn_cast<::clang::ClassTemplateSpecializationDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_concept_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ConceptDecl const*
{
  return llvm::dyn_cast<::clang::ConceptDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_constructor_using_shadow_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ConstructorUsingShadowDecl const*
{
  return llvm::dyn_cast<::clang::ConstructorUsingShadowDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_constructor_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXConstructorDecl const*
{
  return llvm::dyn_cast<::clang::CXXConstructorDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_conversion_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXConversionDecl const*
{
  return llvm::dyn_cast<::clang::CXXConversionDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_deduction_guide_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXDeductionGuideDecl const*
{
  return llvm::dyn_cast<::clang::CXXDeductionGuideDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_destructor_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXDestructorDecl const*
{
  return llvm::dyn_cast<::clang::CXXDestructorDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_method_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXMethodDecl const*
{
  return llvm::dyn_cast<::clang::CXXMethodDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_cxx_record_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::CXXRecordDecl const*
{
  return llvm::dyn_cast<::clang::CXXRecordDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_declarator_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::DeclaratorDecl const*
{
  return llvm::dyn_cast<::clang::DeclaratorDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_decomposition_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::DecompositionDecl const*
{
  return llvm::dyn_cast<::clang::DecompositionDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_empty_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::EmptyDecl const*
{
  return llvm::dyn_cast<::clang::EmptyDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_enum_constant_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::EnumConstantDecl const*
{
  return llvm::dyn_cast<::clang::EnumConstantDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_enum_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::EnumDecl const*
{
  return llvm::dyn_cast<::clang::EnumDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_export_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ExportDecl const*
{
  return llvm::dyn_cast<::clang::ExportDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_extern_c_context_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ExternCContextDecl const*
{
  return llvm::dyn_cast<::clang::ExternCContextDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_field_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FieldDecl const*
{
  return llvm::dyn_cast<::clang::FieldDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_file_scope_asm_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FileScopeAsmDecl const*
{
  return llvm::dyn_cast<::clang::FileScopeAsmDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_friend_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FriendDecl const*
{
  return llvm::dyn_cast<::clang::FriendDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_friend_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FriendTemplateDecl const*
{
  return llvm::dyn_cast<::clang::FriendTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_function_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FunctionDecl const*
{
  return llvm::dyn_cast<::clang::FunctionDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_function_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::FunctionTemplateDecl const*
{
  return llvm::dyn_cast<::clang::FunctionTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_implicit_param_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ImplicitParamDecl const*
{
  return llvm::dyn_cast<::clang::ImplicitParamDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_import_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ImportDecl const*
{
  return llvm::dyn_cast<::clang::ImportDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_indirect_field_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::IndirectFieldDecl const*
{
  return llvm::dyn_cast<::clang::IndirectFieldDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_label_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::LabelDecl const*
{
  return llvm::dyn_cast<::clang::LabelDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_lifetime_extended_temporary_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::LifetimeExtendedTemporaryDecl const*
{
  return llvm::dyn_cast<::clang::LifetimeExtendedTemporaryDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_linkage_spec_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::LinkageSpecDecl const*
{
  return llvm::dyn_cast<::clang::LinkageSpecDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_ms_guid_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::MSGuidDecl const*
{
  return llvm::dyn_cast<::clang::MSGuidDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_ms_property_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::MSPropertyDecl const*
{
  return llvm::dyn_cast<::clang::MSPropertyDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_named_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamedDecl const*
{
  return llvm::dyn_cast<::clang::NamedDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_namespace_alias_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamespaceAliasDecl const*
{
  return llvm::dyn_cast<::clang::NamespaceAliasDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_namespace_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::NamespaceDecl const*
{
  return llvm::dyn_cast<::clang::NamespaceDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_non_type_template_parm_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::NonTypeTemplateParmDecl const*
{
  return llvm::dyn_cast<::clang::NonTypeTemplateParmDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_at_defs_field_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCAtDefsFieldDecl const*
{
  return llvm::dyn_cast<::clang::ObjCAtDefsFieldDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_category_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCCategoryDecl const*
{
  return llvm::dyn_cast<::clang::ObjCCategoryDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_category_impl_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCCategoryImplDecl const*
{
  return llvm::dyn_cast<::clang::ObjCCategoryImplDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_compatible_alias_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::ObjCCompatibleAliasDecl const*
{
  return llvm::dyn_cast<::clang::ObjCCompatibleAliasDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_container_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCContainerDecl const*
{
  return llvm::dyn_cast<::clang::ObjCContainerDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_impl_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCImplDecl const*
{
  return llvm::dyn_cast<::clang::ObjCImplDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_implementation_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCImplementationDecl const*
{
  return llvm::dyn_cast<::clang::ObjCImplementationDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_interface_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCInterfaceDecl const*
{
  return llvm::dyn_cast<::clang::ObjCInterfaceDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_ivar_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCIvarDecl const*
{
  return llvm::dyn_cast<::clang::ObjCIvarDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_method_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCMethodDecl const*
{
  return llvm::dyn_cast<::clang::ObjCMethodDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_property_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCPropertyDecl const*
{
  return llvm::dyn_cast<::clang::ObjCPropertyDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_property_impl_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCPropertyImplDecl const*
{
  return llvm::dyn_cast<::clang::ObjCPropertyImplDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_protocol_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCProtocolDecl const*
{
  return llvm::dyn_cast<::clang::ObjCProtocolDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_obj_c_type_param_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ObjCTypeParamDecl const*
{
  return llvm::dyn_cast<::clang::ObjCTypeParamDecl const>(&This);
}

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_allocate_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPAllocateClause const*
// {
//   return llvm::dyn_cast<::clang::OMPAllocateClause const>(&This);
// }

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_captured_expr_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPCapturedExprDecl const*
// {
//   return llvm::dyn_cast<::clang::OMPCapturedExprDecl const>(&This);
// }

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_declarative_directive_decl(Self const& This [[clang::lifetimebound]])
//   -> ::clang::OMPDeclarativeDirective<::clang::Decl> const*
// {
//   return llvm::dyn_cast<::clang::OMPDeclarativeDirective<::clang::Decl> const>(&This);
// }

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_declarative_directive_value_decl(Self const& This [[clang::lifetimebound]])
//   -> ::clang::OMPDeclarativeDirective<::clang::ValueDecl> const*
// {
//   return llvm::dyn_cast<::clang::OMPDeclarativeDirective<::clang::ValueDecl> const>(&This);
// }

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_omp_declare_mapper_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPDeclareMapperDecl const*
{
  return llvm::dyn_cast<::clang::OMPDeclareMapperDecl const>(&This);
}

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_declare_reduction_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPDeclareReductionDecl
// const*
// {
//   return llvm::dyn_cast<::clang::OMPDeclareReductionDecl const>(&This);
// }

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_requires_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPRequiresDecl const*
// {
//   return llvm::dyn_cast<::clang::OMPRequiresDecl const>(&This);
// }

// [[nodiscard]] [[gnu::always_inline]]
// static inline auto
// cast_as_omp_thread_private_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::OMPThreadPrivateDecl const*
// {
//   return llvm::dyn_cast<::clang::OMPThreadPrivateDecl const>(&This);
// }

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_parm_var_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ParmVarDecl const*
{
  return llvm::dyn_cast<::clang::ParmVarDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_pragma_comment_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::PragmaCommentDecl const*
{
  return llvm::dyn_cast<::clang::PragmaCommentDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_pragma_detect_mismatch_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::PragmaDetectMismatchDecl const*
{
  return llvm::dyn_cast<::clang::PragmaDetectMismatchDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_record_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::RecordDecl const*
{
  return llvm::dyn_cast<::clang::RecordDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_redeclarable_template_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::RedeclarableTemplateDecl const*
{
  return llvm::dyn_cast<::clang::RedeclarableTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_requires_expr_body_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::RequiresExprBodyDecl const*
{
  return llvm::dyn_cast<::clang::RequiresExprBodyDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_static_assert_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::StaticAssertDecl const*
{
  return llvm::dyn_cast<::clang::StaticAssertDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_tag_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TagDecl const*
{
  return llvm::dyn_cast<::clang::TagDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TemplateDecl const*
{
  return llvm::dyn_cast<::clang::TemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_template_param_object_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TemplateParamObjectDecl const*
{
  return llvm::dyn_cast<::clang::TemplateParamObjectDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_template_template_parm_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::TemplateTemplateParmDecl const*
{
  return llvm::dyn_cast<::clang::TemplateTemplateParmDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_template_type_parm_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TemplateTypeParmDecl const*
{
  return llvm::dyn_cast<::clang::TemplateTypeParmDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_translation_unit_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TranslationUnitDecl const*
{
  return llvm::dyn_cast<::clang::TranslationUnitDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_type_alias_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeAliasDecl const*
{
  return llvm::dyn_cast<::clang::TypeAliasDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_type_alias_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeAliasTemplateDecl const*
{
  return llvm::dyn_cast<::clang::TypeAliasTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_type_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypeDecl const*
{
  return llvm::dyn_cast<::clang::TypeDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefDecl const*
{
  return llvm::dyn_cast<::clang::TypedefDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_typedef_name_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::TypedefNameDecl const*
{
  return llvm::dyn_cast<::clang::TypedefNameDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_unnamed_global_constant_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::UnnamedGlobalConstantDecl const*
{
  return llvm::dyn_cast<::clang::UnnamedGlobalConstantDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_unresolved_using_if_exists_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::UnresolvedUsingIfExistsDecl const*
{
  return llvm::dyn_cast<::clang::UnresolvedUsingIfExistsDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_unresolved_using_typename_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::UnresolvedUsingTypenameDecl const*
{
  return llvm::dyn_cast<::clang::UnresolvedUsingTypenameDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_unresolved_using_value_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::UnresolvedUsingValueDecl const*
{
  return llvm::dyn_cast<::clang::UnresolvedUsingValueDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_using_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingDecl const*
{
  return llvm::dyn_cast<::clang::UsingDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_using_directive_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingDirectiveDecl const*
{
  return llvm::dyn_cast<::clang::UsingDirectiveDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_using_enum_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingEnumDecl const*
{
  return llvm::dyn_cast<::clang::UsingEnumDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_using_pack_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingPackDecl const*
{
  return llvm::dyn_cast<::clang::UsingPackDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_using_shadow_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::UsingShadowDecl const*
{
  return llvm::dyn_cast<::clang::UsingShadowDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_value_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::ValueDecl const*
{
  return llvm::dyn_cast<::clang::ValueDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_var_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::VarDecl const*
{
  return llvm::dyn_cast<::clang::VarDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_var_template_decl(Self const& This [[clang::lifetimebound]]) -> ::clang::VarTemplateDecl const*
{
  return llvm::dyn_cast<::clang::VarTemplateDecl const>(&This);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_var_template_specialization_decl(Self const& This [[clang::lifetimebound]])
  -> ::clang::VarTemplateSpecializationDecl const*
{
  return llvm::dyn_cast<::clang::VarTemplateSpecializationDecl const>(&This);
}

} // namespace cxx_clang::clang::ast::decl
