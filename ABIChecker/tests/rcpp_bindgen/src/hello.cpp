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