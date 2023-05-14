#pragma once

class A
{
public:
    A();
    void say_hello() const;
    
private:
    void *ptr;
    int _m_buf;
};

class B : A
{
public:
    B();
    void say_hello() const;

private:
    int _m_val;
};

void root() {
    A a;
    B b;
}