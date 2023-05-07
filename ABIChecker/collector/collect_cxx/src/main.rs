use collect_cxx::parse;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();

    let cinfo = parse(&args.file, None, None).expect("parse failed");
    println!("{}", serde_json::to_string(&cinfo).unwrap());
}
