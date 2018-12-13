extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rasqal");
    println!("cargo:rustc-link-lib=raptor2");

    let bindings = bindgen::Builder::default()
        .header("rasqal.h")
        .layout_tests(false)
        .generate_comments(false)
        .ctypes_prefix("libc")
        .raw_line("use libc;")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
