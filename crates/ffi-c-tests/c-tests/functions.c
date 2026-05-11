#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  assert(ffi_return_primitive() == 8);

  FfiSimpleStruct owned = ffi_new_simple_struct();
  FfiSimpleStruct shared = ffi_new_simple_struct();
  FfiSimpleStruct mutable = ffi_new_simple_struct();

  ffi_test(&owned, &shared, &mutable);

  ffi__ffi_simple_struct_free(&shared);
  ffi__ffi_simple_struct_free(&mutable);

  FfiSimpleStruct r = ffi_new_simple_struct();
  ffi_receive_simple_struct(&r);
  ffi_receive_simple_struct_mut(&r);
  ffi__ffi_simple_struct_free(&r);

  FfiSimpleStruct o = ffi_new_simple_struct();
  ffi_receive_simple_struct_owned(&o);

  return 0;
}
