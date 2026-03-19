use std::os::raw::c_void;

#[repr(C)]
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
        Vec {
            inner: Box::into_raw(Box::new(self)) as *mut c_void,
        }
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezffi_free_vec(o: Vec) {
    let _ = unsafe { Box::from_raw(o.inner as *mut std::vec::Vec<()>) };
}
