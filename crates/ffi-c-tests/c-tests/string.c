#include "../include/test.h"
#include <assert.h>

int main() {
  FfiString s = ffi_string_new();
  assert(s.inner != NULL);
  
  ffi_String_free(s);

  return 0;
}
