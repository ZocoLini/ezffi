use crate::{IntoFfi, IntoRust};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rc<T> {
    inner: *mut core::ffi::c_void,
    _marker: std::marker::PhantomData<T>,
}

impl<T> IntoFfi for std::rc::Rc<T> {
    type Ffi = Rc<T>;

    unsafe fn ref_into_ffi(&self) -> Self::Ffi {
        Rc {
            inner: self as *const Self as *mut core::ffi::c_void,
            _marker: std::marker::PhantomData,
        }
    }

    unsafe fn owned_into_ffi(self) -> Self::Ffi {
        Rc {
            inner: Box::into_raw(Box::new(self.clone())) as *mut core::ffi::c_void,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> IntoRust for Rc<T> {
    type Rust = std::rc::Rc<T>;

    unsafe fn into_rust(&self) -> &Self::Rust {
        unsafe { &*(self.inner as *mut Self::Rust) }
    }

    unsafe fn into_rust_mut(&mut self) -> &mut Self::Rust {
        unsafe { &mut *(self.inner as *mut Self::Rust) }
    }

    unsafe fn into_rust_owned(self) -> Self::Rust {
        unsafe { *Box::from_raw(self.inner as *mut Self::Rust) }
    }
}
