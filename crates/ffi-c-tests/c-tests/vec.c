#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  const EzFfiVec *vec = ffi_create_vec();
  assert(ffi_count_elements(vec) == 10);

  ffi_add_element(vec, 10);
  assert(ffi_count_elements(vec) == 11);

  assert(ffi_get_element(vec, 10) == 10);

  ez_ffi_vec_free(vec);
  ez_ffi_vec_free(vec);
  
  return 0;
}
