#include "../include/ffi-c-tests/test.h"

int main() {
  FfiTestEnum empty = ffi__ffi_test_enum_new_empty();
  ffi__ffi_test_enum_check(&empty);

  FfiTestEnum with_items = ffi__ffi_test_enum_new_with_items();
  ffi__ffi_test_enum_check(&with_items);

  FfiTestEnum with_named = ffi__ffi_test_enum_new_with_named_items();
  ffi__ffi_test_enum_check(&with_named);

  FfiTestEnum freeable = ffi__ffi_test_enum_new_empty();
  ffi__ffi_test_enum_free(&freeable);

  return 0;
}
