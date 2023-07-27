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
        field_decl::FieldDecl,
        function_decl::FunctionDecl,
        named_decl::NamedDecl,
        obj_c_interface_decl::ObjCInterfaceDecl,
        obj_c_method_decl::ObjCMethodDecl,
        obj_c_protocol_decl::ObjCProtocolDecl,
        record_decl::RecordDecl,
        type_decl::TypeDecl,
        typedef_decl::TypedefDecl,
        typedef_name_decl::TypedefNameDecl,
    },
    gen::clang::ast::decl,
};

pub use crate::{abi::clang::ast::decl::Decl, gen::clang::ast::decl::Kind};

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn get_kind(&self) -> decl::Kind {
        decl::get_kind(self)
    }
}

impl<'ctx> Decl<'ctx> {
    #[inline]
    pub fn cast_as_field_decl(&self) -> Option<&FieldDecl<'ctx>> {
        let ptr = decl::cast_as_field_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_function_decl(&self) -> Option<&FunctionDecl<'ctx>> {
        let ptr = decl::cast_as_function_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_named_decl(&self) -> Option<&NamedDecl<'ctx>> {
        let ptr = decl::cast_as_named_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_obj_c_interface_decl(&self) -> Option<&ObjCInterfaceDecl<'ctx>> {
        let ptr = decl::cast_as_obj_c_interface_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_obj_c_method_decl(&self) -> Option<&ObjCMethodDecl<'ctx>> {
        let ptr = decl::cast_as_obj_c_method_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_obj_c_protocol_decl(&self) -> Option<&ObjCProtocolDecl<'ctx>> {
        let ptr = decl::cast_as_obj_c_protocol_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_record_decl(&self) -> Option<&RecordDecl<'ctx>> {
        let ptr = decl::cast_as_record_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_type_decl(&self) -> Option<&TypeDecl<'ctx>> {
        let ptr = decl::cast_as_type_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_typedef_decl(&self) -> Option<&TypedefDecl<'ctx>> {
        let ptr = decl::cast_as_typedef_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_typedef_name_decl(&self) -> Option<&TypedefNameDecl<'ctx>> {
        let ptr = decl::cast_as_typedef_name_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}
