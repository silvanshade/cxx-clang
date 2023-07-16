#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod ast;
pub mod basic;
pub mod lex;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["clang"];
    let path_descendants = &["ast", "basic", "lex"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
