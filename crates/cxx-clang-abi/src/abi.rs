#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod clang;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &[];
    let path_descendants = &["clang"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
pub fn process_artifacts() -> ::cxx_memory_abi::BoxResult<()> {
    self::write_module()?;
    self::clang::write_module()?;
    self::clang::ast::write_module()?;
    self::clang::ast::ast_context::write_module()?;
    self::clang::ast::decl::write_module()?;
    self::clang::ast::named_decl::write_module()?;
    self::clang::basic::write_module()?;
    self::clang::basic::module::write_module()?;
    self::clang::lex::write_module()?;
    self::clang::lex::macro_info::write_module()?;
    self::clang::lex::macro_info::module_macro::write_module()?;
    Ok(())
}
