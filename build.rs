extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=./ipset/lib");
    // println!("cargo:rustc-link-search=./ipset");

    // // Tell cargo to tell rustc to link the system libipset
    // // shared library.
    println!("cargo:rustc-link-lib=./ipset");
    // println!("cargo:rustc-link-lib=./ipset");

    // println!("cargo:include=./ipset/lib");
    // println!("cargo:include=./ipset");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::builder()
    // .clang_args(&["-I./ipset/include", "-l./ipset"])
        .clang_args(&["-I./ipset/include"])
        .header("./ipset/include/libipset/ipset.h")
        .whitelist_function("ipset_load_types|ipset_init|ipset_parse_line|ipset_fini")
        // The input header we would like to generate
        // bindings for.
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
