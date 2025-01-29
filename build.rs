use std::{env, ffi::OsString, path::PathBuf, str};

fn main() {
    // Bridge CPP standalone to Rust
    cxx_build::bridge("src/main.rs")
        .include("./src/")
        .opt_level(3)
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=./lib/libmd5_class.so");
    println!("cargo:rerun-if-changed=./lib/libmodel_sm_ma.a");

    // Link libraries
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let ld_path = env::var_os("LD_LIBRARY_PATH").unwrap();
    // Libraries directory
    println!(
        "cargo:rustc-link-search={}",
        src_dir.join("lib").to_str().unwrap()
    );
    // Link Flags
    println!("cargo:rustc-link-lib=md5_class",);
    println!("cargo:rustc-link-lib=model_sm_ma",);

    // Update envoiraments paths
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}:{}",
        src_dir.join("lib/").to_str().unwrap(),
        ld_path.to_str().unwrap()
    );
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}
