use clap::Parser;
use collect_cxx::{parse, Args};

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let cinfo = parse(args).expect("parse failed");
    println!("{}", serde_json::to_string(&cinfo).unwrap());
}
