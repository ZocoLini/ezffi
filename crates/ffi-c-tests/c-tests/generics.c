#include "../include/test.h"
#include <assert.h>

int main() {
  FfiGeneric a = ffi_Generic_new(1);
  FfiGeneric b = ffi_Generic_new(2);

  FfiGeneric c = ffi_Generic_add(a, b);
  FfiGeneric d = ffi_Generic_multiply(b, b);
  FfiGeneric e = ffi_add2(a, b);

  assert(ffi_Generic_get(c) == 3);
  assert(ffi_Generic_get(d) == 4);
  assert(ffi_Generic_get(e) == 3);

  ffi_Generic_free(a);
  ffi_Generic_free(b);
  ffi_Generic_free(c);
  ffi_Generic_free(d);
  ffi_Generic_free(e);
  
  return 0;
}