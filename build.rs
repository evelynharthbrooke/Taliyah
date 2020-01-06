use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version());
}

fn rustc_version() -> String {
    // Compiler used by Cargo.
    let rustc = env::var_os("RUSTC").expect("missing RUSTC environment variable");

    // `rustc --version` output.
    let cmd = Command::new(rustc).arg("--version").output().expect("failed to get rust version");

    String::from_utf8_lossy(&cmd.stdout).to_string()
}

