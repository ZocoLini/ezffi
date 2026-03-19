#![allow(clippy::missing_safety_doc)]
#![allow(clippy::wrong_self_convention)]

pub use ezffi_macros::*;

mod std_impls;

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
