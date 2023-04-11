#pragma once

// Class
class A
{
public:
    A(int size);
    ~A();
    void func();

private:
    char *data;
};

class B : public virtual A
{
public:
    B(int size);
    ~B();
    void func();
};

class C : public virtual A, public B
{
public:
    C(int size);
    ~C();
    void func();
};

// Struct
struct S
{
    char a;
    int b;
    char c;
};

struct S getS();

// Overload
void overload(int a);
void overload(int a, int b);

// Template
template <typename T>
T add(T a, T b)
{
    return a + b;
}