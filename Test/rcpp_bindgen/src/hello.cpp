#include <stdio.h>
#include "../include/hello.hpp"

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

B::B(int size): A(size) {
    printf("Constructor B\n");
}

B::~B() {
    printf("Destructor B\n");
}

void B::func() {
    printf("func B\n");
}