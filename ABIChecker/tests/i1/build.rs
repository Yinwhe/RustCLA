extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=./");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=hello");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/hello.hpp");
    println!("cargo:rerun-if-changed=src/hello.cpp");

    // Compile
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("src/hello.cpp")
        .compile("hello");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-std=c++11")
        // whitelist
        // .whitelist_type("A")
        // .whitelist_type("B")
        // .whitelist_type("MyString")
        
        // The input header we would like to generate
        // bindings for.
        .header("include/hello.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
