#include <stdio.h>

struct T {
    int a;
    int b;
    int c[10];
};

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

void override(T t) {
    printf("Override 2!\n");
    printf("t.a = %d\n", t.a);
    printf("t.b = %d\n", t.b);
}