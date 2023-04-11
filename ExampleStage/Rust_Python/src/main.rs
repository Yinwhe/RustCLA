use pyo3::prelude::*;

#[pyclass]
pub struct Data {
    vals: [i64; 3],
    cf: fn(&mut i64),
    bx: Box<i64>,
}

#[inline(never)]
pub fn doubler(x: &mut i64) {
    *x *= 2;
}

fn main() -> PyResult<()> {
    let code = include_str!("hello.py");

    let data = Data {
        vals: [0, 1, 2],
        cf: doubler,
        bx: Box::new(0),
    };

    Python::with_gil(|py| -> PyResult<()> {
        let _test = PyModule::from_code(py, code, "", "")?.getattr("test")?;
        let read_val = PyModule::from_code(py, code, "", "")?.getattr("read_val")?;
        let read_box = PyModule::from_code(py, code, "", "")?.getattr("read_box")?;
        
        
        // Spacial Attack
        read_val.call1((data.vals.as_ptr() as i64,))?;
        println!("data.cf: {:x}", data.cf as *mut i64 as i64);

        // Temporal Attack
        read_box.call1((data.bx.as_ref() as *const i64 as i64,))?;
        Ok(())
    }).unwrap_or_default();

    println!("data.bx: {}", data.bx);

    Ok(())
}
