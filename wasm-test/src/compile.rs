use anyhow::{Context, Error};
use rustmatic_wasm::Program;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;

#[derive(Debug, Clone, PartialEq)]
pub struct Compiler {
    std_manifest_dir: PathBuf,
    target_dir: PathBuf,
}

impl Compiler {
    pub fn instantiate(&self, name: &str, src: &str) -> Result<Program, Error> {
        let wasm = compile_to_wasm(
            name,
            src,
            &self.target_dir,
            &self.std_manifest_dir,
        )
        .unwrap();

        Program::load(&wasm)
            .map_err(|e| anyhow::format_err!("WASM loading failed: {}", e))
    }
}

impl Default for Compiler {
    fn default() -> Compiler {
        let wasm_test_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        let crate_root = wasm_test_manifest_dir.parent().unwrap();

        Compiler {
            target_dir: crate_root.join("target"),
            std_manifest_dir: crate_root.join("iec-std"),
        }
    }
}

const CARGO_TOML_TEMPLATE: &str = r#"
[package]
name = "$TEST_NAME"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
edition = "2018"

[dependencies]
rustmatic-iec-std = { path = "$STD_PATH" }

[lib]
path = "lib.rs"
crate-type = ["cdylib"]
"#;

fn compile_to_wasm(
    name: &str,
    src: &str,
    target_dir: &Path,
    std_manifest_dir: &Path,
) -> Result<Vec<u8>, Error> {
    // first we'll need a crate
    let dir = TempDir::new().context("Unable to create a temporary dir")?;

    // then create a Cargo.toml file
    let std_manifest_dir = std_manifest_dir.display().to_string();
    let cargo_toml = CARGO_TOML_TEMPLATE
        .replace("$TEST_NAME", name)
        .replace("$STD_PATH", &std_manifest_dir);
    let cargo_toml_path = dir.path().join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml)
        .context("Couldn't write Cargo.toml")?;

    // copy our source code across
    fs::write(dir.path().join("lib.rs"), src)
        .context("Couldn't write lib.rs")?;

    // compile to wasm
    let output = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(&cargo_toml_path)
        .arg("--target-dir")
        .arg(&target_dir)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--offline")
        .status()
        .context("Unable to start cargo")?;

    anyhow::ensure!(output.success(), "Compilation failed");

    let blob = target_dir
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join(name)
        .with_extension("wasm");

    fs::read(&blob)
        .with_context(|| format!("Unable to read \"{}\"", blob.display()))
}
