#pragma once

class A
{
public:
    A();
private:
    void *ptr;
    int _m_buf;
};

class B : A
{
public:
    B();
private:
    int _m_val;
};