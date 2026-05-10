#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct EzFfiString {
  void *inner;
  uint8_t state;
} EzFfiString;

typedef struct EzFfiVec {
  void *inner;
  uint8_t state;
} EzFfiVec;

void ez_ffi_string_free(const struct EzFfiString *o);

void ez_ffi_vec_free(const struct EzFfiVec *o);
