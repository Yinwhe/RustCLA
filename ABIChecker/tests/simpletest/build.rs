fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/c_function.cpp");

    let bindings = bindgen::Builder::default()
        .header("src/c_function.cpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("hello")
        .allowlist_function("override_")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");


    cc::Build::new()
        // .cpp(true)
        .file("src/c_function.cpp")
        .compile("c_function");
}
