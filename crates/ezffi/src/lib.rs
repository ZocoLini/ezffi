#![allow(clippy::missing_safety_doc)]
#![allow(clippy::wrong_self_convention)]

pub use ezffi_macros::*;

mod std_impls;

pub use std_impls::*;

pub trait IntoFfi {
    type Ffi;

    unsafe fn ref_into_ffi(&self) -> Self::Ffi;
    unsafe fn owned_into_ffi(self) -> Self::Ffi;
}

pub trait IntoRust {
    type Rust;

    unsafe fn into_rust(&self) -> &Self::Rust;
    unsafe fn into_rust_mut(&mut self) -> &mut Self::Rust;
    unsafe fn into_rust_owned(self) -> Self::Rust;
}

pub trait GenericIntoFfi<T> {
    type Ffi;

    unsafe fn ref_into_ffi(&self) -> Self::Ffi;
    unsafe fn owned_into_ffi(self) -> Self::Ffi;
}

pub trait GenericIntoRust {
    unsafe fn into_rust<T>(&self) -> &T;
    unsafe fn into_rust_mut<T>(&mut self) -> &mut T;
    unsafe fn into_rust_owned<T>(self) -> T;
}
