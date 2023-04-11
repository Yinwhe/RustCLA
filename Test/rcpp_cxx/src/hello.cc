#include "rcpp_cxx/include/hello.hpp"
#include "rcpp_cxx/src/main.rs.h"
#include <stdio.h>

CHello::~CHello() {
    printf("CHello destructor called!\n");
}

void CHello::hello() const {
    printf("Hello from C++! My data is %s!\n", this->data.c_str());
}

void CHello::setData(const std::string &data) {
    this->data = data;
}

void CHello::testRHello() const {
    rust::box<RHello> rh = getRHello(rust::String("RHelloName"));
    rh->hello();
}

std::unique_ptr<CHello> getCHello(const std::string &data) {
    return std::make_unique<CHello>(data);
}