#[ezffi::export]
pub struct Generic<T> {
    pub value: T,
}

#[ezffi::export]
impl Generic<u64> {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get(&self) -> u64 {
        self.value
    }

    pub fn add(&self, other: &Self) -> Generic<u64> {
        Generic {
            value: self.value + other.value,
        }
    }

    pub fn multiply(&self, other: &Generic<u64>) -> Self {
        Generic {
            value: self.value * other.value,
        }
    }
}

#[ezffi::export]
pub fn add2(a: &Generic<u64>, b: &Generic<u64>) -> Generic<u64> {
    Generic {
        value: a.value + b.value,
    }
}

#[test]
fn test_generic_add() {
    let a = unsafe { ffi_Generic_new(1) };
    let b = unsafe { ffi_Generic_new(2) };

    let c = unsafe { ffi_Generic_add(a, b) };
    let d = unsafe { ffi_Generic_multiply(b, b) };
    let e = unsafe { ffi_add2(a, b) };

    assert_eq!(unsafe { ffi_Generic_get(c) }, 3);
    assert_eq!(unsafe { ffi_Generic_get(d) }, 4);
    assert_eq!(unsafe { ffi_Generic_get(e) }, 3);

    unsafe { ffi_Generic_free(a) };
    unsafe { ffi_Generic_free(b) };
    unsafe { ffi_Generic_free(c) };
    unsafe { ffi_Generic_free(d) };
    unsafe { ffi_Generic_free(e) };
}
