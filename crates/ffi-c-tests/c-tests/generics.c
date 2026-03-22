#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  const FfiGeneric *a = ffi__ffi_generic_new(1);
  const FfiGeneric *b = ffi__ffi_generic_new(2);

  const FfiGeneric *c = ffi__ffi_generic_add(a, b);
  const FfiGeneric *d = ffi__ffi_generic_multiply(b, b);
  const FfiGeneric *e = ffi_add2(a, b);

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