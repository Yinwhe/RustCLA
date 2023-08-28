mod utils;

extern "C" {
    fn hello();
}

fn main() {
    utils::test();
    unsafe {
        hello();
    }
}
