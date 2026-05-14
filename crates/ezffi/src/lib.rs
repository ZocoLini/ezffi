#![allow(clippy::missing_safety_doc)]
#![allow(clippy::wrong_self_convention)]

pub use ezffi_macros::export;

#[cfg(feature = "async")]
mod async_rt;
mod std_impls;

#[cfg(feature = "async")]
pub use async_rt::*;
pub use std_impls::*;

pub trait IntoFfi<T> {
    type Ffi;

    unsafe fn ref_into_ffi(&self) -> Self::Ffi;
    unsafe fn owned_into_ffi(self) -> Self::Ffi;
}

pub trait IntoRust<T> {
    unsafe fn into_rust(&self) -> &T;
    unsafe fn into_rust_mut(&mut self) -> &mut T;
    unsafe fn into_rust_owned(self) -> T;
}

#[cfg(debug_assertions)]
#[repr(u8)]
pub enum TypeState {
    Owned = 0,
    Ref = 1,
    Freed = 2,
}
