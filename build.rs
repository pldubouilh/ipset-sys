extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    Command::new("make")
        .args(["build-deps"])
        .output()
        .expect("failed to execute process");

    println!("cargo:rustc-link-search=./outlib/lib");

    println!("cargo:rustc-link-lib=ipset");
    // println!("cargo:rustc-link-lib=static=ipset");

    println!("cargo:include=./outlib/include");

    println!("cargo:rerun-if-changed=wrapper.h");

    // keeping pkgconfig attemps, might be useful in the future
    // let library = pkg_config::probe_library("ipset").unwrap();

    let bindings = bindgen::Builder::default()
        // .clang_args(library.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        .clang_args(&["-I./outlib/include"])
        .header("./wrapper.h")
        .whitelist_function("ipset_load_types|ipset_init|ipset_parse_line|ipset_fini")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
