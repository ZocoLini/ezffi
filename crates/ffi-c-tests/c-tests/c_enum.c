#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  FfiColor c = ffi__ffi_color_new_red();
  assert(c == FfiColor_Red);
  assert(!ffi__ffi_color_check_is_green(&c));

  c = ffi__ffi_color_next(&c);
  assert(c == FfiColor_Green);
  assert(ffi__ffi_color_check_is_green(&c));

  c = ffi__ffi_color_next(&c);
  assert(c == FfiColor_Blue);

  c = ffi__ffi_color_next(&c);
  assert(c == FfiColor_Red);

  return 0;
}
