use cbindgen_rust_parser::Builder;

pub struct Foo {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let res = Builder::new()
        .with_src("binding.rs")
        .with_std_types(false).parse();
    println!("{:?}", res)
}
