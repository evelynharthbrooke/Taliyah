extern crate built;

use std::path::Path;
use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version());

    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    let mut options = built::Options::default();
    options.set_dependencies(true).set_compiler(true).set_env(true);

    built::write_built_file_with_opts(&options, src, out).expect("Failed to acquire build-time information");
}

fn rustc_version() -> String {
    // Compiler used by Cargo.
    let rustc = env::var_os("RUSTC").expect("missing RUSTC environment variable");

    // `rustc --version` output.
    let cmd = Command::new(rustc).arg("--version").output().expect("failed to get rust version");

    String::from_utf8_lossy(&cmd.stdout).to_string()
}

