use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("cargo:rustc-flags=-L {}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=Cwtch");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libCwtch.h");

    let lib_cwtch_path = Path::new(&out_dir).join("libCwtch.so");
    // https://git.openprivacy.ca/cwtch.im/libcwtch-go/releases v1.2.0
    Command::new("wget").arg("https://git.openprivacy.ca/attachments/e77c69f0-9487-4808-bc23-092d943bc4a6").arg("-O").arg(lib_cwtch_path).output().expect("failed to download libCwtch.so");
}