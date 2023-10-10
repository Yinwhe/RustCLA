#include<stdlib.h>
#include<stdio.h>
#include<string.h>

class Hello {
private:
    char data[16];
    void* ptr;
public:
    Hello();
    ~Hello();
    void sayHello();
};

Hello::Hello() {
    ptr = malloc(16);
    strcpy(data, "Hello World!");
}

Hello::~Hello() {
    free(ptr);
}

void Hello::sayHello() {
    printf("%s\n", data);
}