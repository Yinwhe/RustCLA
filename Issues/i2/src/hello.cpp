#include "../include/hello.hpp"
#include <stdio.h>

Foo MakeFoo() { return Foo{.data = 4}; }

Bar MakeBar() { return Bar{.data = 8}; }

// int main() {
//     Foo f = MakeFoo();
//     Bar b = MakeBar();
//     printf("Foo: %d, Bar: %d", f.data, b.data);
// }