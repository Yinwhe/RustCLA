fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cpp");
    println!("cargo:rerun-if-changed=include/hello.cc");

    cxx_build::bridge("src/main.rs")
        .file("src/hello.cc")
        .flag_if_supported("-std=c++14")
        .compile("rcpp_cxx");
    
}