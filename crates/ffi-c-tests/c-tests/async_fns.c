#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  ffi_init_tokio();

  assert(ffi_async_double(21) == 42);
  assert(ffi_async_double(0) == 0);

  assert(ffi_async_sum_three(1, 2, 3) == 6);
  assert(ffi_async_sum_three(10, 20, 30) == 60);

  return 0;
}
