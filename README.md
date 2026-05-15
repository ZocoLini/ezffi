# ezffi

Generate C-FFI bindings from Rust types and functions with a single attribute.

```rust
#[ezffi::export]
pub struct Car { km: u64 }

#[ezffi::export]
impl Car {
    pub fn new() -> Self { Self { km: 0 } }
    pub fn drive(&mut self, km: u64) { self.km += km; }
    pub fn km(&self) -> u64 { self.km }
}
```

A `build.rs` invoking `cbindgen` produces:

```c
typedef struct YourCrateCar { void *inner; uint8_t state; } YourCrateCar;

FfiCar   your_crate_car_new(void);
void     your_crate_car_drive(const FfiCar *this, uint64_t km);
uint64_t your_crate_car_km(const FfiCar *this);
void     your_crate_car_free(const FfiCar *o);
```

## Install

```toml
[dependencies]
ezffi = "0.1"

[build-dependencies]
cbindgen = "0.29"
```

See [`crates/ffi-c-tests`](crates/ffi-c-tests) for a working end-to-end setup
(Rust source, `build.rs`, `cbindgen.toml`, C tests).

## Features

- `generics` — N-parameters generic types (`Vec<T>`, `Generic<T>`).
- `async` — wrap `async fn` into sync C wrappers via a pluggable dispatcher
  (default `pollster`; install your own with `ezffi::set_async_dispatcher`).

## Naming

Per-crate `ezffi.toml` controls prefix/suffix/case-style for types and
functions. Defaults to the crate name + snake_case.
