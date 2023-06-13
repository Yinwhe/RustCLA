#include <cstdint>

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


extern "C" void hello(A a) {}
void root0(A a){};

void root1(__fsid_t a){};
