#include <stdio.h>
#include "../include/hello.hpp"

// Class A
A::A(int size) {
    this->data = new char[size];
    printf("Constructor A\n");
}

A::~A() {
    delete[] this->data;
    printf("Destructor A\n");
}

void A::func() {
    printf("func A\n");
}


// Class B
B::B(int size): A(size) {
    printf("Constructor B\n");
}

B::~B() {
    printf("Destructor B\n");
}

void B::func() {
    printf("func B\n");
}


// Class C
C::C(int size): A(size), B(size) {
    printf("Constructor C\n");
}

C::~C() {
    printf("Destructor C\n");
}

void C::func() {
    printf("func C\n");
}

// Struct S
struct S getS() {
    return S{ 'a', 1, 'b' };
}

void overload(int a) {
    printf("overload 1, data: %d\n", a);
}

void overload(int a, int b) {
    printf("overload 2, data: %d, %d\n", a, b);
}