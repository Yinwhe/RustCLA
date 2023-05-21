#pragma once
#include <stddef.h>

struct Foo {
    size_t data;
};

struct Bar {
    size_t data;
    ~Bar() { data = 4; }
};

extern "C" Foo MakeFoo() { return Foo(); }

extern "C" Bar MakeBar() { return Bar(); }

void root() {
    Foo f;
    Bar b;
}