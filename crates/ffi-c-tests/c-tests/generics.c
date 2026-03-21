#include "../include/test.h"
#include <assert.h>

int main() {
  FfiGeneric a = ffi__ffi_generic_new(1);
  FfiGeneric b = ffi__ffi_generic_new(2);

  FfiGeneric c = ffi__ffi_generic_add(a, b);
  FfiGeneric d = ffi__ffi_generic_multiply(b, b);
  FfiGeneric e = ffi_add2(a, b);

  assert(ffi__ffi_generic_get(c) == 3);
  assert(ffi__ffi_generic_get(d) == 4);
  assert(ffi__ffi_generic_get(e) == 3);

  ffi__ffi_generic_free(a);
  ffi__ffi_generic_free(b);
  ffi__ffi_generic_free(c);
  ffi__ffi_generic_free(d);
  ffi__ffi_generic_free(e);
  
  return 0;
}