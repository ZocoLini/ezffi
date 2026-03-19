#include "../include/test.h"
#include <assert.h>

int main() {
  assert(ffi_add(5, 5) == 10);

  return 0;
}
