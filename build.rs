use std::fs;
use std::path::Path;
use std::process::Command;
use std::{env, io};

use hex_literal::hex;
use sha2::{Digest, Sha512};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("cargo:rustc-flags=-L {}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=Cwtch");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libCwtch.h");

    let lib_cwtch_path = Path::new(&out_dir).join("libCwtch.so");
    // https://git.openprivacy.ca/cwtch.im/libcwtch-go/releases v1.5.4
    Command::new("wget")
        .arg("https://git.openprivacy.ca/attachments/dd3c6b41-98e4-4e7b-81af-d21893bfe389")
        .arg("-O")
        .arg(lib_cwtch_path)
        .output()
        .expect("failed to download libCwtch.so");

    let lib_cwtch_path = Path::new(&out_dir).join("libCwtch.so");
    let mut hasher = Sha512::new();
    let mut file = fs::File::open(&lib_cwtch_path).expect("could not open lib to hash");
    io::copy(&mut file, &mut hasher).expect("failed to copy file into hasher");
    let hash_bytes = hasher.finalize();

    assert_eq!(hash_bytes[..], hex!("776a26076dfad3370d1b2edec9ad954187584f54483ec201163be0dc356c10b0fe74168e8e95f2116f458e5676e1fb07fbd0357cab1e4389ac762fe03bd5ef67")[..]);
}
