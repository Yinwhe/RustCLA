#include <stdio.h>

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

void override(int d) {
    printf("Override 2!\n");
}