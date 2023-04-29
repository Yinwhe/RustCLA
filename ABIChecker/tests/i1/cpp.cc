#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class MyCLikeEnum
{
  Foo1,
  Bar1,
  Baz1,
};

enum class MyCLikeEnum_Prepended
{
  Foo1_Prepended,
  Bar1_Prepended,
  Baz1_Prepended,
};

struct MyFancyStruct
{
  int32_t i;
};

struct MyFancyEnum
{
  enum class Tag
  {
    Foo,
    Bar,
    Baz,
  };

  struct Bar_Body
  {
    int32_t _0;
  };

  struct Baz_Body
  {
    int32_t _0;
  };

  Tag tag;
  union
  {
    Bar_Body bar;
    Baz_Body baz;
  };
};

union MyUnion
{
  float f;
  uint32_t u;
};

struct MyFancyStruct_Prepended
{
  int32_t i;
};

struct MyFancyEnum_Prepended
{
  enum class Tag
  {
    Foo_Prepended,
    Bar_Prepended,
    Baz_Prepended,
  };

  struct Bar_Prepended_Body
  {
    int32_t _0;
  };

  struct Baz_Prepended_Body
  {
    int32_t _0;
  };

  Tag tag;
  union
  {
    Bar_Prepended_Body bar_prepended;
    Baz_Prepended_Body baz_prepended;
  };
};

union MyUnion_Prepended
{
  float f;
  uint32_t u;
};

extern "C"{
void root(MyFancyStruct a,
                     MyFancyEnum b,
                     MyUnion c,
                     MyFancyStruct_Prepended d,
                     MyFancyEnum_Prepended e,
                     MyUnion_Prepended f);
}