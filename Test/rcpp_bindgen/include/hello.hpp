#pragma once

class A {
public:
    A(int size);
    ~A();
    void func();

private:
    char* data;
};

class B: public A {
public:
    B(int size);
    ~B();
    void func();
};