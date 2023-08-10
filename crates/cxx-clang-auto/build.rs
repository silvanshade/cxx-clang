use cxx_llvm_build_common::prelude::*;
use std::path::{Path, PathBuf};

pub fn project_dir() -> BoxResult<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let project_dir = std::path::PathBuf::from(&cargo_manifest_dir);
    Ok(project_dir)
}

fn process_cxx(out_dir: &Path) -> BoxResult<()> {
    let cargo_pkg_name = "cxx-clang-auto";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let clang_dirs = cxx_clang_build::Dirs::new(cargo_pkg_name, &llvm_dirs)?;
    let includes = &[
        llvm_dirs.llvm_project.join("llvm/include"),
        llvm_dirs.llvm_cmake_build.join("include"),
        clang_dirs.clang_project.join("include"),
        clang_dirs.clang_cmake_build.join("include"),
    ];
    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(PathBuf::as_path));
    let rust_source_files = &[
        out_dir.join("src/auto/clang/ast/ast_context.rs"),
        out_dir.join("src/auto/clang/ast/decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/access_spec_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/base_using_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/binding_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/block_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/builtin_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/captured_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/class_scope_function_specialization_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/class_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/class_template_partial_specialization_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/class_template_specialization_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/concept_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/constructor_using_shadow_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_constructor_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_conversion_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_deduction_guide_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_destructor_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_method_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/cxx_record_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/decl_context.rs"),
        out_dir.join("src/auto/clang/ast/decl/declarator_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/decomposition_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/empty_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/enum_constant_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/enum_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/export_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/extern_c_context_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/field_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/file_scope_asm_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/friend_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/friend_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/function_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/function_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/implicit_param_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/import_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/indirect_field_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/label_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/lifetime_extended_temporary_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/linkage_spec_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/ms_guid_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/ms_property_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/named_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/namespace_alias_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/namespace_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/non_type_template_parm_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_at_defs_field_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_category_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_category_impl_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_compatible_alias_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_container_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_impl_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_implementation_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_interface_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_ivar_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_method_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_property_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_property_impl_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_protocol_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/obj_c_type_param_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_allocate_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_captured_expr_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_declarative_directive_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_declarative_directive_value_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_declare_mapper_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_declare_reduction_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_requires_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/omp_thread_private_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/parm_var_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/pragma_comment_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/pragma_detect_mismatch_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/record_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/redeclarable_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/requires_expr_body_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/static_assert_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/tag_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/template_param_object_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/template_template_parm_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/template_type_parm_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/translation_unit_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/type_alias_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/type_alias_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/type_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/typedef_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/typedef_name_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/unnamed_global_constant_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/unresolved_using_if_exists_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/unresolved_using_typename_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/unresolved_using_value_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/using_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/using_directive_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/using_enum_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/using_pack_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/using_shadow_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/value_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/var_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/var_template_decl.rs"),
        out_dir.join("src/auto/clang/ast/decl/var_template_specialization_decl.rs"),
        out_dir.join("src/auto/clang/ast/type_.rs"),
        out_dir.join("src/auto/clang/basic/module.rs"),
        out_dir.join("src/auto/clang/lex/macro_info.rs"),
        out_dir.join("src/auto/clang/lex/macro_info/module_macro.rs"),
    ];
    let files: &[&str] = &[];
    let output = "cxx-clang-auto";
    cxx_clang_build::cxx_build(&llvm_dirs, &clang_dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=auto");
    println!("cargo:rerun-if-changed=cxx");
    let project_dir = project_dir()?;
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    let cfg_dir = project_dir.join("auto");
    cxx_auto::process_artifacts(&project_dir, &out_dir, &cfg_dir)?;
    process_cxx(&out_dir)?;
    Ok(())
}
