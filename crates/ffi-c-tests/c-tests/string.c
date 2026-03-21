#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  EzFfiString s = ffi_string_new();
  assert(s.inner != NULL);
  
  ez_ffi_string_free(s);

  return 0;
}
