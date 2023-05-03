use collect_cxx::parse;

fn main() {
    let res = parse("tests/typedef.h", None, None);
    print!("{:#?}", res)
}
