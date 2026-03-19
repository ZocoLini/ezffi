#include "../include/test.h"
#include <assert.h>

int main() {
  FfiCar car = ffi_Car_new();
  assert(ffi_Car_km(car) == 0);
  ffi_Car_drive(car, 100);
  assert(ffi_Car_km(car) == 100);

  FfiCar other = ffi_Car_new();
  ffi_Car_drive(other, 50);
  assert(ffi_Car_km(other) == 50);

  ffi_Car_merge_cars(car, other);
  assert(ffi_Car_km(car) == 150);

  ffi_Car_free(car);
  return 0;
}
