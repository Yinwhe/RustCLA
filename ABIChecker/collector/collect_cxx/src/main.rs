use collect_cxx::Resolver;

fn main() {
    let mut resolver = Resolver::new("tests/struct.cc", &["-x", "c++", "-std=c++11"]).unwrap();
    resolver.resolve_bindgen_one();
}
