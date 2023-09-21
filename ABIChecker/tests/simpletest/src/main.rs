mod utils;

include!("bindings.rs");

fn main() {
    unsafe {
        hello();
        override_();
        let t = T {
            a: 1,
            b: 2,
            c: [3; 10],
        };
        let tt = TT {
            t: t,
            s: [3; 2],
        };
        override_1(tt);
    }
}