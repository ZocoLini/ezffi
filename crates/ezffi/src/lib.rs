#![allow(clippy::missing_safety_doc)]
#![allow(clippy::wrong_self_convention)]

pub use ezffi_macros::*;

mod std_impls;

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
