#include <stdio.h>
#include <stdlib.h>

void hello() {
    printf("Hello Rust! We hacked it!\n");
}

void read_val(__uint64_t *x) {
    // overwrite
    x[3] = (__uint64_t)hello;
}

void read_box(__uint64_t *x) {
    // double free & use after free
    free(x);
}