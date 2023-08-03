use cxx_llvm_build_common::prelude::*;
use normpath::PathExt;
use std::path::{Path, PathBuf};

fn locate_clang_project_path(cargo_manifest_dir: &Path, llvm_project: &Path) -> BoxResult<PathBuf> {
    let err =
        Err("Failed to locate Clang project path. Try setting the `CLANG_PROJECT_PATH` environment variable.".into());

    let mut should_ignore = false;

    if let Some(clang_project) = get_path_from_env("CLANG_PROJECT_PATH", &mut should_ignore)? {
        return Ok(clang_project);
    }

    if should_ignore {
        return err;
    }

    let clang_project = llvm_project.join("clang");
    if clang_project.exists() {
        return Ok(clang_project);
    }

    let clang_project = PathBuf::from(cargo_manifest_dir).join("../../../clang");
    if clang_project.exists() {
        return Ok(clang_project.normalize()?.into_path_buf());
    }

    err
}

fn locate_clang_cmake_build_path(llvm_cmake_build: &Path, clang_project: &Path) -> BoxResult<PathBuf> {
    let err = Err(
        "Failed to locate Clang CMake build path. Try setting the `CLANG_CMAKE_BUILD_PATH` environment variable."
            .into(),
    );

    let mut should_ignore = false;

    if let Some(clang_cmake_build) = get_path_from_env("CLANG_CMAKE_BUILD_PATH", &mut should_ignore)? {
        return Ok(clang_cmake_build);
    }

    if should_ignore {
        return err;
    }

    let clang_cmake_build = llvm_cmake_build.join("tools/clang");
    if clang_cmake_build.exists() {
        return Ok(clang_cmake_build);
    }

    let clang_cmake_build = clang_project.join(PathBuf::from_iter([
        "build",
        &NINJA_BUILD_DIR(),
        &format!("clang-{CMAKE_BUILD_TARGET}"),
    ]));
    if clang_cmake_build.exists() {
        return Ok(clang_cmake_build);
    }

    err
}

pub struct Dirs {
    pub clang_project: PathBuf,
    pub clang_cmake_build: PathBuf,
}

impl Dirs {
    pub fn new(cargo_pkg_name: &str, llvm_dirs: &cxx_llvm_build::Dirs) -> BoxResult<Self> {
        let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        let cxx_llvm_build::Dirs {
            llvm_project,
            llvm_cmake_build,
            ..
        } = &llvm_dirs;
        let clang_project = locate_clang_project_path(&cargo_manifest_dir, llvm_project)?;
        println!(
            "cargo:warning=[{cargo_pkg_name}]: Clang project path: \"{}\"",
            clang_project.display()
        );
        let clang_cmake_build = locate_clang_cmake_build_path(llvm_cmake_build, &clang_project)?;
        println!(
            "cargo:warning=[{cargo_pkg_name}]: Clang CMake build path: \"{}\"",
            clang_cmake_build.display()
        );
        let dirs = Dirs {
            clang_project,
            clang_cmake_build,
        };
        Ok(dirs)
    }
}

pub fn cxx_build(
    llvm_dirs: &cxx_llvm_build::Dirs,
    clang_dirs: &Dirs,
    rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>,
    files: impl IntoIterator<Item = impl AsRef<Path>>,
    output: &str,
) -> BoxResult<()> {
    rustc_link_searches(llvm_dirs, clang_dirs);
    cxx_build::bridges(rust_source_files)
        .llvm_common_compiler()
        .llvm_common_defines()
        .llvm_common_flags()
        .files(files)
        .try_compile(output)?;
    rustc_link_libs();
    Ok(())
}

pub fn rustc_link_searches(_llvm_dirs: &cxx_llvm_build::Dirs, _clang_dirs: &Dirs) {
}

pub fn rustc_link_libs() {
    println!("cargo:rustc-link-lib=static=clangIndex");
    println!("cargo:rustc-link-lib=static=clangDependencyScanning");
    println!("cargo:rustc-link-lib=static=clangCodeGen");
    println!("cargo:rustc-link-lib=static=clangTooling");
    println!("cargo:rustc-link-lib=static=clangFrontend");
    println!("cargo:rustc-link-lib=static=clangCAS");
    println!("cargo:rustc-link-lib=static=clangParse");
    println!("cargo:rustc-link-lib=static=clangDriver");
    println!("cargo:rustc-link-lib=static=clangSerialization");
    println!("cargo:rustc-link-lib=static=clangSema");
    println!("cargo:rustc-link-lib=static=clangEdit");
    println!("cargo:rustc-link-lib=static=clangAnalysis");
    println!("cargo:rustc-link-lib=static=clangAPINotes");
    println!("cargo:rustc-link-lib=static=clangAST");
    println!("cargo:rustc-link-lib=static=clangLex");
    println!("cargo:rustc-link-lib=static=clangBasic");
    cxx_llvm_build::rustc_link_libs();
}
