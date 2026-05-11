#include "../include/ffi-c-tests/test.h"
#include <assert.h>

int main() {
  // Car struct lifecycle, tests different params and return combinations
  FfiCar carA = ffi__ffi_car_new();
  assert(ffi__ffi_car_km(&carA) == 0);
  ffi__ffi_car_drive(&carA, 100);
  assert(ffi__ffi_car_km(&carA) == 100);

  FfiCar carB = ffi__ffi_car_new();
  ffi__ffi_car_drive(&carB, 50);
  assert(ffi__ffi_car_km(&carB) == 50);

  // merge_cars consumes carB, so it must not be freed afterwards.
  ffi__ffi_car_merge_cars(&carA, &carB);
  assert(ffi__ffi_car_km(&carA) == 150);

  FfiCar carC = ffi__ffi_car_new();
  ffi__ffi_car_drive(&carC, 200);
  ffi__ffi_car_receive_mut_ref(&carA, &carC);
  assert(ffi__ffi_car_km(&carA) == 350);

  ffi__ffi_car_receive_ref(&carA, &carC);
  assert(ffi__ffi_car_km(&carA) == 550);

  ffi__ffi_car_free(&carA);
  ffi__ffi_car_free(&carC);

  // Special struct with a reference counter to ensure memory is freed
  FfiDeallocationStruct obj = ffi__ffi_deallocation_struct_new();
  FfiDeallocationStruct clone1 = ffi__ffi_deallocation_struct_manual_clone(&obj);
  FfiDeallocationStruct clone2 = ffi__ffi_deallocation_struct_manual_clone(&obj);

  assert(ffi__ffi_deallocation_struct_get_counter(&obj) == 3);
  ffi__ffi_deallocation_struct_consume(&clone1);

  assert(ffi__ffi_deallocation_struct_get_counter(&obj) == 2);
  ffi__ffi_deallocation_struct_free(&clone2);

  assert(ffi__ffi_deallocation_struct_get_counter(&obj) == 1);
  ffi__ffi_deallocation_struct_free(&obj);

  return 0;
}
