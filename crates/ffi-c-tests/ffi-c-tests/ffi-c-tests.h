#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "../ezffi/ezffi.h"

typedef struct FfiSimpleStruct {
  void *inner;
  uint8_t state;
} FfiSimpleStruct;

typedef struct FfiGeneric {
  void *inner;
  uint8_t state;
} FfiGeneric;

typedef struct FfiCar {
  void *inner;
  uint8_t state;
} FfiCar;

typedef struct FfiDeallocationStruct {
  void *inner;
  uint8_t state;
} FfiDeallocationStruct;

void ffi__ffi_simple_struct_free(const struct FfiSimpleStruct *o);

uint32_t ffi_return_primitive(void);

const struct FfiSimpleStruct *ffi_new_simple_struct(void);

void ffi_test(const struct FfiSimpleStruct *_o,
              const struct FfiSimpleStruct *_r,
              const struct FfiSimpleStruct *_m);

void ffi_receive_simple_struct(const struct FfiSimpleStruct *_o);

void ffi_receive_simple_struct_mut(const struct FfiSimpleStruct *_o);

void ffi_receive_simple_struct_owned(const struct FfiSimpleStruct *_o);

void ffi__ffi_generic_free(const struct FfiGeneric *o);

const struct FfiGeneric *ffi__ffi_generic_new(uint64_t value);

uint64_t ffi__ffi_generic_get(const struct FfiGeneric *this_);

const struct FfiGeneric *ffi__ffi_generic_add(const struct FfiGeneric *this_,
                                              const struct FfiGeneric *other);

const struct FfiGeneric *ffi__ffi_generic_multiply(const struct FfiGeneric *this_,
                                                   const struct FfiGeneric *other);

const struct FfiGeneric *ffi_add2(const struct FfiGeneric *a, const struct FfiGeneric *b);

const EzFfiString *ffi_string_new(void);

void ffi__ffi_car_free(const struct FfiCar *o);

const struct FfiCar *ffi__ffi_car_new(void);

uint64_t ffi__ffi_car_km(const struct FfiCar *this_);

void ffi__ffi_car_drive(const struct FfiCar *this_, uint64_t km);

void ffi__ffi_car_merge_cars(const struct FfiCar *this_, const struct FfiCar *other);

void ffi__ffi_car_receive_mut_ref(const struct FfiCar *this_, const struct FfiCar *other);

void ffi__ffi_car_receive_ref(const struct FfiCar *this_, const struct FfiCar *other);

void ffi__ffi_deallocation_struct_free(const struct FfiDeallocationStruct *o);

const struct FfiDeallocationStruct *ffi__ffi_deallocation_struct_new(void);

const struct FfiDeallocationStruct *ffi__ffi_deallocation_struct_manual_clone(const struct FfiDeallocationStruct *this_);

uintptr_t ffi__ffi_deallocation_struct_get_counter(const struct FfiDeallocationStruct *this_);

void ffi__ffi_deallocation_struct_consume(const struct FfiDeallocationStruct *this_);

uintptr_t ffi_count_elements(const EzFfiVec *vec);

void ffi_add_element(const EzFfiVec *vec, uint32_t value);

uint32_t ffi_get_element(const EzFfiVec *vec, uintptr_t index);

const EzFfiVec *ffi_create_vec(void);
