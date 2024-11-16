fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/c_function.hpp");

    // let bindings = bindgen::Builder::default()
    //     .header("src/c_function.hpp")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file("src/bindings.rs")
    //     .expect("Couldn't write bindings!");


    cc::Build::new()
        // .cpp(true)
        .file("src/c_function.cpp")
        .compile("c_function");
}
