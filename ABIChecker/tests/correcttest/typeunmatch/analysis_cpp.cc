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
void root0(B a){};

void root1(T a){};

void root2(A a){};
