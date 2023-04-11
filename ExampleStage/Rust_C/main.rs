extern "C" {
    fn read_val(x: &mut [i64; 3]);
    fn read_box(x: &mut i64);
}

pub struct Data {
    vals: [i64; 3],
    cf: fn(&mut i64),
    bx: Box<i64>
}

#[inline(never)]
pub fn doubler(x: &mut i64) {
    *x *= 2;
}

fn main() {
    let mut data = Data {
        vals: [0, 1, 2],
        cf: doubler,
        bx: Box::new(0)
    };

    // Spacial Attack
    unsafe{ read_val(&mut data.vals) }
    println!("Execute data.cf");
    let mut x = 10;
    (data.cf)(&mut x);

    // Temporal Attack
    unsafe{ read_box(&mut data.bx.as_mut()) }
    println!("read box: {}", data.bx);
}