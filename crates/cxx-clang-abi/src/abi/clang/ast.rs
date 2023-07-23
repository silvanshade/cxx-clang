#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod ast_context;
pub mod decl;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["clang", "ast"];
    let path_descendants = &["ast_context", "decl"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
