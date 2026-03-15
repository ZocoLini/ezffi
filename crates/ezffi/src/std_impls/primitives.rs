macro_rules! impl_ffi_identity {
    ($t:ty) => {
        impl crate::IntoRust for $t {
            type Rust = $t;

            unsafe fn into_rust(&self) -> &Self::Rust {
                self
            }

            unsafe fn into_rust_mut(&mut self) -> &mut Self::Rust {
                self
            }

            unsafe fn into_rust_owned(self) -> Self::Rust {
                self
            }
        }

        impl crate::IntoFfi for $t {
            type Ffi = $t;

            unsafe fn owned_into_ffi(self) -> Self::Ffi {
                self
            }

            unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                *self
            }
        }
    };
}

impl_ffi_identity!(());

impl_ffi_identity!(i8);
impl_ffi_identity!(u8);
impl_ffi_identity!(i16);
impl_ffi_identity!(u16);
impl_ffi_identity!(i32);
impl_ffi_identity!(u32);
impl_ffi_identity!(i64);
impl_ffi_identity!(u64);
impl_ffi_identity!(i128);
impl_ffi_identity!(u128);
impl_ffi_identity!(isize);
impl_ffi_identity!(usize);

impl_ffi_identity!(f32);
impl_ffi_identity!(f64);

impl_ffi_identity!(bool);
impl_ffi_identity!(char);
