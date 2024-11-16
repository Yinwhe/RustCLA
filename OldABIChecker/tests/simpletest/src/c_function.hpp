struct T {
    int a;
    int b;
    int c[10];
};

struct TT {
    struct T t;
    int s[2];
};

void hello();

void override();

void override(TT t);