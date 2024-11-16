#include <stdio.h>
#include <algorithm>
#include "c_function.hpp"

extern "C" {
    void c_hello();
}

void hello() {
    printf("Hello Rust!\n");
    c_hello();
}

void override() {
    printf("Override 1!\n");
}

void override(TT t) {
    printf("Override 2!\n");
}