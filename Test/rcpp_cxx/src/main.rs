use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type RHello;

        fn getRHello(data: String) -> Box<RHello>;
        fn hello(&self);
    }

    unsafe extern "C++" {
        include!("rcpp_cxx/include/hello.hpp");

        type CHello;

        fn getCHello(data: &CxxString) -> UniquePtr<CHello>;
        fn hello(&self);
        fn setData(self: Pin<&mut CHello>, data: &CxxString);
        fn testRHello(&self);
    }
}

struct RHello {
    data: String,
}

impl RHello {
    pub fn new(data: String) -> RHello {
        RHello { data }
    }

    pub fn hello(&self) {
        println!("Hello from Rust! My data is {}!", self.data);
    }
}

fn getRHello(data: String) -> Box<RHello> {
    Box::new(RHello::new(data))
}

fn main() {
    {
        let_cxx_string!(data = "CHelloName");
        let mut h = ffi::getCHello(&data);
        h.hello();

        let_cxx_string!(data = "CHelloNameAgain");
        h.as_mut().unwrap().setData(&data);
        h.hello();
    }
    // drop h
}
