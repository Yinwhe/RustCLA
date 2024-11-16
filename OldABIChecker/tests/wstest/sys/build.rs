fn main() {
    println!("cargo:rerun-if-changed=csrc");

    let mut build = cc::Build::new();
    build.file("csrc/hello.cpp");
    build.compile("hello");
}
