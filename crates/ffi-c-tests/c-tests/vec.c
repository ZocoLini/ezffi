#include "../include/test.h"
#include <assert.h>

int main() {
  Vec vec = ffi_create_vec();
  assert(ffi_count_elements(vec) == 10);

  ffi_add_element(vec, 10);
  assert(ffi_count_elements(vec) == 11);

  assert(ffi_get_element(vec, 10) == 10);

  ezffi_free_Vec(vec);
  
  return 0;
}
