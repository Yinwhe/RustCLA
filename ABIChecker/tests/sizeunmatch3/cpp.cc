#include <cstdint>

// class A {
// private:
//     int64_t ptr;
// public:
//     int32_t m_buf[1];
// };

// union T {
//   int64_t a;
//   int32_t b[3];
// };

// struct A {
//     void* ptr;
//     unsigned int val[1];
//     T t[2];
// };

struct A {
  enum class Tag {
    One,
    Two,
  };

  struct One_Body {
    int32_t _0;
  };

  struct Two_Body {
    uint64_t _0;
  };

  Tag tag;
  union {
    One_Body one;
    Two_Body two;
  };
};


void hello(A a) {}

extern "C" void root(A a) {
    // A a;
}