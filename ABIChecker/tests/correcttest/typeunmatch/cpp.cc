struct T {
    char a;
};

struct A {
    unsigned int a;
    int b;
    T t;
};

struct B {
    int a;
    int b;
};

void root() {
    struct A a;
    struct B b;
}