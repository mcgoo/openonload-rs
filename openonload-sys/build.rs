extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
//        .no_unstable_rust()
        .header("wrapper.h")
//        .clang_arg("-I../../libs/toyframe")
//        .clang_arg("-I../../libs/sutl")
//        .clang_arg("-I../../services/pfrechan/include")
//        .clang_arg("--std=c++11")
//        .whitelisted_type("rtpx::.*")
//        .whitelisted_type("pfrechanmsgs::.*")
//        .enable_cxx_namespaces()
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
