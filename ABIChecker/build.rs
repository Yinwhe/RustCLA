use std::fs::create_dir;
use std::process::Command;

fn main() {
    print!("cargo:rerun-if-changed=collector");
    let home = std::env::var("HOME").expect("Fail to get $HOME");
    create_dir(format!("{}/.abi_checker", home)).unwrap_or_default();

    let build_collect_cxx = Command::new("sh")
        .args(&[
            "-c",
            "cd collector/collect_cxx && cargo build --release && cp target/release/collect_cxx $HOME/.abi_checker/",
        ])
        .output()
        .expect("failed to execute process");

    if !build_collect_cxx.status.success() {
        panic!(
            "build collect_cxx fails, due to {:?}",
            String::from_utf8_lossy(&build_collect_cxx.stderr)
        );
    }

    let build_collect_rust = Command::new("sh")
        .args(&[
            "-c",
            "cd collector/collect_rust && cargo build --release && cp target/release/collect_rust $HOME/.abi_checker/",
        ])
        .output()
        .expect("failed to execute process");

    if !build_collect_rust.status.success() {
        panic!(
            "build collect_rust fails, due to {:?}",
            String::from_utf8_lossy(&build_collect_rust.stderr)
        );
    }
}
