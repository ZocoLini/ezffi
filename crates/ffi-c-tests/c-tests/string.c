#include "../include/test.h"
#include <assert.h>

int main() {
  String s = ffi_string_new();
  assert(s.inner != NULL);
  
  ezffi_free_String(s);

  return 0;
}
