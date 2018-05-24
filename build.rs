extern crate cc;
extern crate bindgen;

use std::path;
use std::env;

fn main() {

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg("-x c++")
        .header("native/tinydx.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
       bindings
           .write_to_file(out_path.join("bindings.rs"))
           .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(true)
        .define("TINY_RENDERER_IMPLEMENTATION", "1")
        .file("native/tinydx.cpp")
        .compile("tinydx");
}
