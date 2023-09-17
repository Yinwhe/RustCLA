#include "../include/hello.hpp"
#include <stdio.h>

A::A()
{
    ptr = nullptr;
    _m_buf = 0;
}

B::B()
{
    _m_val = 0;
}

void A::say_hello() const {
    printf("Hello, world!");
}

void B::say_hello() const {
    printf("Hello, world!");
}