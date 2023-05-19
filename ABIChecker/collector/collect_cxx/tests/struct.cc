// // bindgen-flags: --constified-enum-module foo

// enum foo {
//   Type,
//   Type_,
//   Type1,
//   Type__,
// };

// struct bar {
//   enum foo member;
// };

// struct T {
//   int a;
//   int* c;
//   unsigned int d;
// };

// void root(foo f, bar b){}

struct A {
  int a;

  void hello();
};
