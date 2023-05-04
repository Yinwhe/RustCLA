use collect_rust::Resolver;

fn main() {
    let mut resolver = Resolver::new("tests/struct.rs").unwrap();
    resolver.resolve_cbindgen_one();
}
