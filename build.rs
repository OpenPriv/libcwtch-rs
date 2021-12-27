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
    // https://git.openprivacy.ca/cwtch.im/libcwtch-go/releases v1.5.2
    Command::new("wget").arg("https://git.openprivacy.ca/attachments/3e563341-c206-4e69-98e3-e0dca449f947").arg("-O").arg(lib_cwtch_path).output().expect("failed to download libCwtch.so");
}