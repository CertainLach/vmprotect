use std::{env, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("lib").display()
    );
    let target = env::var("TARGET").unwrap();
    if target == "x86_64-apple-darwin" {
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK");
    } else if target.starts_with("x86_64-") {
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK64");
    } else if target.starts_with("i686-") {
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK32")
    } else {
        panic!("Unsupported target: {}", target)
    }
}
