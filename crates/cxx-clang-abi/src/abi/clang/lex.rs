#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod macro_info;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["clang", "lex"];
    let path_descendants = &["macro_info"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
