use std::env;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}/lib/",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    match env::var("TARGET").unwrap().as_ref() {
        "x86_64-pc-windows-msvc" |
        "x86_64-pc-windows-gnu" |
        "x86_64-unknown-linux-gnu" => println!("cargo:rustc-link-lib=dylib=VMProtectSDK64"),
        "x86-pc-windows-msvc" |
        "i686-unknown-linux-gnu" |
        "i686-pc-windows-gnu" |
        "x86-unknown-linux-gnu" => println!("cargo:rustc-link-lib=dylib=VMProtectSDK32"),
        v => panic!("Unsupported target: {}", v),
    }
}
