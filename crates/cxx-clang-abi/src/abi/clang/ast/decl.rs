#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod access_spec_decl;
pub mod base_using_decl;
pub mod binding_decl;
pub mod block_decl;
pub mod builtin_template_decl;
pub mod captured_decl;
pub mod class_scope_function_specialization_decl;
pub mod class_template_decl;
pub mod class_template_partial_specialization_decl;
pub mod class_template_specialization_decl;
pub mod concept_decl;
pub mod constructor_using_shadow_decl;
pub mod cxx_constructor_decl;
pub mod cxx_conversion_decl;
pub mod cxx_deduction_guide_decl;
pub mod cxx_destructor_decl;
pub mod cxx_method_decl;
pub mod cxx_record_decl;
pub mod decl_context;
pub mod declarator_decl;
pub mod decomposition_decl;
pub mod empty_decl;
pub mod enum_constant_decl;
pub mod enum_decl;
pub mod export_decl;
pub mod extern_c_context_decl;
pub mod field_decl;
pub mod file_scope_asm_decl;
pub mod friend_decl;
pub mod friend_template_decl;
pub mod function_decl;
pub mod function_template_decl;
pub mod implicit_param_decl;
pub mod import_decl;
pub mod indirect_field_decl;
pub mod label_decl;
pub mod lifetime_extended_temporary_decl;
pub mod linkage_spec_decl;
pub mod ms_guid_decl;
pub mod ms_property_decl;
pub mod named_decl;
pub mod namespace_alias_decl;
pub mod namespace_decl;
pub mod non_type_template_parm_decl;
pub mod obj_c_at_defs_field_decl;
pub mod obj_c_category_decl;
pub mod obj_c_category_impl_decl;
pub mod obj_c_compatible_alias_decl;
pub mod obj_c_container_decl;
pub mod obj_c_impl_decl;
pub mod obj_c_implementation_decl;
pub mod obj_c_interface_decl;
pub mod obj_c_ivar_decl;
pub mod obj_c_method_decl;
pub mod obj_c_property_decl;
pub mod obj_c_property_impl_decl;
pub mod obj_c_protocol_decl;
pub mod obj_c_type_param_decl;
pub mod omp_allocate_decl;
pub mod omp_captured_expr_decl;
pub mod omp_declarative_directive_decl;
pub mod omp_declarative_directive_value_decl;
pub mod omp_declare_mapper_decl;
pub mod omp_declare_reduction_decl;
pub mod omp_requires_decl;
pub mod omp_thread_private_decl;
pub mod parm_var_decl;
pub mod pragma_comment_decl;
pub mod pragma_detect_mismatch_decl;
pub mod record_decl;
pub mod redeclarable_template_decl;
pub mod requires_expr_body_decl;
pub mod static_assert_decl;
pub mod tag_decl;
pub mod template_decl;
pub mod template_param_object_decl;
pub mod template_template_parm_decl;
pub mod template_type_parm_decl;
pub mod translation_unit_decl;
pub mod type_alias_decl;
pub mod type_alias_template_decl;
pub mod type_decl;
pub mod typedef_decl;
pub mod typedef_name_decl;
pub mod unnamed_global_constant_decl;
pub mod unresolved_using_if_exists_decl;
pub mod unresolved_using_typename_decl;
pub mod unresolved_using_value_decl;
pub mod using_decl;
pub mod using_directive_decl;
pub mod using_enum_decl;
pub mod using_pack_decl;
pub mod using_shadow_decl;
pub mod value_decl;
pub mod var_decl;
pub mod var_template_decl;
pub mod var_template_specialization_decl;
#[cxx::bridge]
mod ffi {
    #[namespace = "cxx_clang::clang::ast::decl"]
    unsafe extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl.hxx");
        fn cxx_abi_align() -> usize;
        fn cxx_abi_size() -> usize;
        fn cxx_is_copy_constructible() -> bool;
        fn cxx_is_move_constructible() -> bool;
        fn cxx_is_default_constructible() -> bool;
        fn cxx_is_destructible() -> bool;
        fn cxx_is_trivially_copyable() -> bool;
        fn cxx_is_trivially_movable() -> bool;
        fn cxx_is_trivially_destructible() -> bool;
        fn rust_should_impl_cxx_extern_type_trivial() -> bool;
        fn rust_should_impl_unpin() -> bool;
        fn rust_should_impl_send() -> bool;
        fn rust_should_impl_sync() -> bool;
        fn rust_should_impl_drop() -> bool;
        fn rust_should_impl_copy() -> bool;
        fn rust_should_impl_default() -> bool;
        fn rust_should_impl_cxx_memory_copy_new() -> bool;
        fn rust_should_impl_cxx_memory_move_new() -> bool;
    }
}
pub use ffi::*;
fn artifact_info() -> ::cxx_memory_abi::CxxAbiArtifactInfo {
    let path_components = vec!["clang", "ast", "decl"];
    let path_descendants = vec![
        "access_spec_decl",
        "base_using_decl",
        "binding_decl",
        "block_decl",
        "builtin_template_decl",
        "captured_decl",
        "class_scope_function_specialization_decl",
        "class_template_decl",
        "class_template_partial_specialization_decl",
        "class_template_specialization_decl",
        "concept_decl",
        "constructor_using_shadow_decl",
        "cxx_constructor_decl",
        "cxx_conversion_decl",
        "cxx_deduction_guide_decl",
        "cxx_destructor_decl",
        "cxx_method_decl",
        "cxx_record_decl",
        "decl_context",
        "declarator_decl",
        "decomposition_decl",
        "empty_decl",
        "enum_constant_decl",
        "enum_decl",
        "export_decl",
        "extern_c_context_decl",
        "field_decl",
        "file_scope_asm_decl",
        "friend_decl",
        "friend_template_decl",
        "function_decl",
        "function_template_decl",
        "implicit_param_decl",
        "import_decl",
        "indirect_field_decl",
        "label_decl",
        "lifetime_extended_temporary_decl",
        "linkage_spec_decl",
        "ms_guid_decl",
        "ms_property_decl",
        "named_decl",
        "namespace_alias_decl",
        "namespace_decl",
        "non_type_template_parm_decl",
        "obj_c_at_defs_field_decl",
        "obj_c_category_decl",
        "obj_c_category_impl_decl",
        "obj_c_compatible_alias_decl",
        "obj_c_container_decl",
        "obj_c_impl_decl",
        "obj_c_implementation_decl",
        "obj_c_interface_decl",
        "obj_c_ivar_decl",
        "obj_c_method_decl",
        "obj_c_property_decl",
        "obj_c_property_impl_decl",
        "obj_c_protocol_decl",
        "obj_c_type_param_decl",
        "omp_allocate_decl",
        "omp_captured_expr_decl",
        "omp_declarative_directive_decl",
        "omp_declarative_directive_value_decl",
        "omp_declare_mapper_decl",
        "omp_declare_reduction_decl",
        "omp_requires_decl",
        "omp_thread_private_decl",
        "parm_var_decl",
        "pragma_comment_decl",
        "pragma_detect_mismatch_decl",
        "record_decl",
        "redeclarable_template_decl",
        "requires_expr_body_decl",
        "static_assert_decl",
        "tag_decl",
        "template_decl",
        "template_param_object_decl",
        "template_template_parm_decl",
        "template_type_parm_decl",
        "translation_unit_decl",
        "type_alias_decl",
        "type_alias_template_decl",
        "type_decl",
        "typedef_decl",
        "typedef_name_decl",
        "unnamed_global_constant_decl",
        "unresolved_using_if_exists_decl",
        "unresolved_using_typename_decl",
        "unresolved_using_value_decl",
        "using_decl",
        "using_directive_decl",
        "using_enum_decl",
        "using_pack_decl",
        "using_shadow_decl",
        "value_decl",
        "var_decl",
        "var_template_decl",
        "var_template_specialization_decl",
    ];
    let cxx_include = "cxx-clang-abi/cxx/include/clang/AST/Decl.hxx";
    let cxx_namespace = "cxx_clang::clang::ast::decl";
    let cxx_name = "Decl";
    let rust_name = "Decl";
    let lifetimes = ::cxx_memory_abi::indexmap::IndexMap::from_iter([("ctx", vec![])].into_iter());
    let align = self::ffi::cxx_abi_align();
    let size = self::ffi::cxx_abi_size();
    let is_rust_cxx_extern_type_trivial = {
        let cxx_is_trivially_movable = self::ffi::cxx_is_trivially_movable();
        let rust_should_impl_cxx_extern_type_trivial = self::ffi::rust_should_impl_cxx_extern_type_trivial();
        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
            cxx_is_trivially_movable
        } else {
            rust_should_impl_cxx_extern_type_trivial
        }
    };
    let is_rust_unpin = self::ffi::rust_should_impl_unpin();
    let is_rust_send = self::ffi::rust_should_impl_send();
    let is_rust_sync = self::ffi::rust_should_impl_sync();
    let is_rust_copy = self::ffi::rust_should_impl_copy();
    let is_rust_drop = self::ffi::rust_should_impl_drop();
    let is_rust_default = self::ffi::rust_should_impl_default();
    let is_rust_copy_new = self::ffi::rust_should_impl_cxx_memory_copy_new();
    let is_rust_move_new = self::ffi::rust_should_impl_cxx_memory_move_new();
    ::cxx_memory_abi::CxxAbiArtifactInfo {
        path_components,
        path_descendants,
        cxx_include,
        cxx_namespace,
        cxx_name,
        rust_name,
        lifetimes,
        align,
        size,
        is_rust_cxx_extern_type_trivial,
        is_rust_unpin,
        is_rust_send,
        is_rust_sync,
        is_rust_copy,
        is_rust_default,
        is_rust_drop,
        is_rust_copy_new,
        is_rust_move_new,
    }
}
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    self::artifact_info().write_module_for_file()
}
