#include "../include/test.h"
#include <assert.h>

int main() {
  // Car struct lifecycle, tests different params and return combinations
  FfiCar carA = ffi_Car_new();
  assert(ffi_Car_km(carA) == 0);
  ffi_Car_drive(carA, 100);
  assert(ffi_Car_km(carA) == 100);

  FfiCar carB = ffi_Car_new();
  ffi_Car_drive(carB, 50);
  assert(ffi_Car_km(carB) == 50);

  ffi_Car_merge_cars(carA, carB);
  assert(ffi_Car_km(carA) == 150);

  FfiCar carC = ffi_Car_new();
  ffi_Car_drive(carC, 200);
  ffi_Car_receive_mut_ref(carA, carC);
  assert(ffi_Car_km(carA) == 350);

  ffi_Car_receive_ref(carA, carC);
  assert(ffi_Car_km(carA) == 550);
  
  ffi_Car_free(carA);
  ffi_Car_free(carC);
  
  // Special struct with a reference counter to ensure memory is freed 
  FfiDeallocationStruct obj = ffi_DeallocationStruct_new();
  FfiDeallocationStruct clone1 = ffi_DeallocationStruct_manual_clone(obj);
  FfiDeallocationStruct clone2 = ffi_DeallocationStruct_manual_clone(obj);

  assert(ffi_DeallocationStruct_get_counter(obj) == 3);
  ffi_DeallocationStruct_consume(clone1);

  assert(ffi_DeallocationStruct_get_counter(obj) == 2);
  ffi_DeallocationStruct_free(clone2);

  assert(ffi_DeallocationStruct_get_counter(obj) == 1);
  
  return 0;
}