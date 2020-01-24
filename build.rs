extern crate built;

use std::env;
use std::path::Path;

fn main() {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    let mut options = built::Options::default();
    options.set_compiler(true).set_env(true).set_features(false).set_cfg(false).set_ci(false);
    built::write_built_file_with_opts(&options, src, out).expect("Failed to acquire build-time information");
}
