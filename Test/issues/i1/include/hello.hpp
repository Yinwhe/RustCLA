#pragma once

class A
{
public:
    A();
    virtual void say_hello() const;
    
private:
    void *ptr;
    int _m_buf;
};

class B : A
{
public:
    B();
    virtual void say_hello() const;

private:
    int _m_val;
};
