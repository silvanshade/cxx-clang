use normpath::PathExt;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

pub fn project_dir() -> BoxResult<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let project_dir = std::path::PathBuf::from(&cargo_manifest_dir);
    let project_dir = project_dir.normalize()?.into_path_buf();
    Ok(project_dir)
}

fn process_cxx() -> BoxResult<()> {
    let dirs = cxx_llvm_common::Dirs::new()?;
    let rust_source_files: &[&str] = &[
        "src/abi/clang/ast/ast_context.rs",
        "src/abi/clang/ast/decl.rs",
        "src/abi/clang/ast/named_decl.rs",
        "src/abi/clang/basic/module.rs",
        "src/abi/clang/lex/macro_info.rs",
        "src/abi/clang/lex/macro_info/module_macro.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-clang-abi";
    cxx_llvm_common::cxx_build(&dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=abi");
    println!("cargo:rerun-if-changed=cxx");
    let project_dir = project_dir()?;
    let abi_dir = project_dir.join("abi");
    let abi_crate_src_dir = project_dir.join("src");
    cxx_memory_abi::process_artifacts(&abi_dir, &abi_crate_src_dir)?;
    process_cxx()?;
    Ok(())
}
