#pragma once
#include "rust/cxx.h"
#include <string>
#include <memory>

struct RHello;

class CHello {
public:
    CHello(std::string data): data(data) {}
    ~CHello();
    void hello() const;
    void setData(const std::string &data);
    void testRHello() const;
private:
    std::string data;
};

std::unique_ptr<CHello> getCHello(const std::string &data);