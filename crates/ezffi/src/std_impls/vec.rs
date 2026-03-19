use std::os::raw::c_void;

pub struct Vec {
    inner: *mut c_void,
}

impl<T> crate::IntoFfi<T> for std::vec::Vec<T> {
    type Ffi = Vec;

    unsafe fn ref_into_ffi(&self) -> Vec {
        Vec {
            inner: self as *const std::vec::Vec<T> as *mut c_void,
        }
    }

    unsafe fn owned_into_ffi(self) -> Vec {
        let result = unsafe { Self::ref_into_ffi(&self) };
        std::mem::forget(self);
        result
    }
}

impl<T> crate::IntoRust<T> for Vec {
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
