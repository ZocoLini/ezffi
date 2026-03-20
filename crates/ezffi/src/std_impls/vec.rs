use std::os::raw::c_void;

macro_rules! impl_ffi_extern_generic_type {
    ($t:ty, $free_t:ty, $n:ident, $free_fn:ident) => {
        #[repr(C)]
        pub struct $n {
            inner: *mut c_void,
        }

        impl<T> crate::IntoFfi<T> for $t {
            type Ffi = $n;

            unsafe fn ref_into_ffi(&self) -> $n {
                $n {
                    inner: self as *const $t as *mut c_void,
                }
            }

            unsafe fn owned_into_ffi(self) -> $n {
                $n {
                    inner: Box::into_raw(Box::new(self)) as *mut c_void,
                }
            }
        }

        impl<T> crate::IntoRust<T> for $n {
            unsafe fn into_rust(&self) -> &T {
                unsafe { &*(self.inner as *const T) }
            }

            unsafe fn into_rust_mut(&mut self) -> &mut T {
                unsafe { &mut *(self.inner as *mut T) }
            }

            unsafe fn into_rust_owned(self) -> T {
                unsafe { std::ptr::read(self.inner as *const T) }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $free_fn(o: $n) {
            let _ = unsafe { Box::from_raw(o.inner as *mut $free_t) };
        }
    };
}

impl_ffi_extern_generic_type!(std::vec::Vec<T>, std::vec::Vec<()>, Vec, ezffi_free_vec);
