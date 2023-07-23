type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn process_cxx() -> BoxResult<()> {
    let dirs = cxx_llvm_common::Dirs::new()?;
    let rust_source_files: &[&str] = &[
        "src/abi/clang/ast/ast_context.rs",
        "src/abi/clang/ast/decl.rs",
        "src/abi/clang/ast/decl/decl_context.rs",
        "src/abi/clang/ast/decl/declarator_decl.rs",
        "src/abi/clang/ast/decl/field_decl.rs",
        "src/abi/clang/ast/decl/function_decl.rs",
        "src/abi/clang/ast/decl/named_decl.rs",
        "src/abi/clang/ast/decl/objc_container_decl.rs",
        "src/abi/clang/ast/decl/objc_interface_decl.rs",
        "src/abi/clang/ast/decl/objc_method_decl.rs",
        "src/abi/clang/ast/decl/objc_protocol_decl.rs",
        "src/abi/clang/ast/decl/record_decl.rs",
        "src/abi/clang/ast/decl/tag_decl.rs",
        "src/abi/clang/ast/decl/type_decl.rs",
        "src/abi/clang/ast/decl/typedef_decl.rs",
        "src/abi/clang/ast/decl/typedef_name_decl.rs",
        "src/abi/clang/ast/decl/value_decl.rs",
        "src/abi/clang/basic/module.rs",
        "src/abi/clang/lex/macro_info.rs",
        "src/abi/clang/lex/macro_info/module_macro.rs",
        "src/gen/clang/ast/ast_context.rs",
        "src/gen/clang/ast/decl.rs",
        "src/gen/clang/ast/decl/decl_context.rs",
        "src/gen/clang/ast/decl/declarator_decl.rs",
        "src/gen/clang/ast/decl/field_decl.rs",
        "src/gen/clang/ast/decl/function_decl.rs",
        "src/gen/clang/ast/decl/named_decl.rs",
        "src/gen/clang/ast/decl/objc_container_decl.rs",
        "src/gen/clang/ast/decl/objc_interface_decl.rs",
        "src/gen/clang/ast/decl/objc_method_decl.rs",
        "src/gen/clang/ast/decl/objc_protocol_decl.rs",
        "src/gen/clang/ast/decl/record_decl.rs",
        "src/gen/clang/ast/decl/tag_decl.rs",
        "src/gen/clang/ast/decl/type_decl.rs",
        "src/gen/clang/ast/decl/typedef_decl.rs",
        "src/gen/clang/ast/decl/typedef_name_decl.rs",
        "src/gen/clang/ast/decl/value_decl.rs",
        "src/gen/clang/basic/module.rs",
        "src/gen/clang/lex/macro_info.rs",
        "src/gen/clang/lex/macro_info/module_macro.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-clang";
    cxx_llvm_common::cxx_build(&dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=src/gen");
    println!("cargo:rerun-if-changed=../cxx-clang-abi");
    cxx_clang_abi::abi::process_artifacts()?;
    process_cxx()?;
    Ok(())
}
