#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  assert(ffi_return_primitive() == 8);
  
  const FfiSimpleStruct *a = ffi_new_simple_struct();
  const FfiSimpleStruct *b = ffi_new_simple_struct();
  const FfiSimpleStruct *c = ffi_new_simple_struct();
  
  ffi_test(a, b, c);
  ffi_receive_simple_struct(a);
  ffi_receive_simple_struct_mut(a);
  
  ffi_receive_simple_struct_owned(a);
  ffi_receive_simple_struct_owned(b);
  ffi_receive_simple_struct_owned(c);

  return 0;
}
