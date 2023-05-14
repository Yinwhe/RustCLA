use collect_rust::parse;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let rinfo = parse(&args.file).expect("parse failed");
    println!("{}", serde_json::to_string(&rinfo).unwrap());
}
