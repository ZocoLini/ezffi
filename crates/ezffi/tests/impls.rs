use std::rc::Rc;

#[ezffi::export]
#[derive(Default)]
pub struct SimpleStruct {
    field: u64,
}

#[ezffi::export]
impl SimpleStruct {
    pub fn new() -> Self {
        SimpleStruct { field: 0 }
    }

    pub fn getter(&self) -> u64 {
        self.field
    }

    pub fn setter(&mut self, value: u64) {
        self.field = value;
    }

    pub fn funny_destroyer(mut self) {
        self.field = 0;
    }

    pub fn static1() {}

    pub fn static2(mut a: Self, b: &Self, c: &mut Self) -> Self {
        c.field += a.field + b.field;
        a.field += b.field + c.field;
        a
    }

    pub fn ret_self_ref(&self) -> &Self {
        self
    }

    pub fn ret_self_mut(&mut self) -> &mut Self {
        self
    }
}

#[test]
fn test_struct_ffi_methods_usage() {
    let obj1 = unsafe { ffi_SimpleStruct_new() };
    assert_eq!(unsafe { ffi_SimpleStruct_getter(obj1) }, 0);
    unsafe { ffi_SimpleStruct_setter(obj1, 42) };
    assert_eq!(unsafe { ffi_SimpleStruct_getter(obj1) }, 42);

    let obj2 = unsafe { ffi_SimpleStruct_new() };
    unsafe { ffi_SimpleStruct_setter(obj2, 100) };

    let obj3 = unsafe { ffi_SimpleStruct_new() };
    unsafe { ffi_SimpleStruct_setter(obj3, 200) };

    unsafe { ffi_SimpleStruct_static1() };

    let obj4 = unsafe { ffi_SimpleStruct_static2(obj1, obj2, obj3) };
    assert_eq!(unsafe { ffi_SimpleStruct_getter(obj3) }, 342);
    assert_eq!(unsafe { ffi_SimpleStruct_getter(obj4) }, 484);

    let _ = unsafe { ffi_SimpleStruct_ret_self_ref(obj4) };
    let _ = unsafe { ffi_SimpleStruct_ret_self_mut(obj4) };

    // obj1 was already destroyed by static2
    // unsafe { ffi_SimpleStruct_funny_destroyer(obj1) };

    unsafe { ffi_SimpleStruct_free(obj2) };

    // obj2 was already destroyed by static2
    // unsafe { ffi_SimpleStruct_funny_destroyer(obj2) };

    unsafe { ffi_SimpleStruct_funny_destroyer(obj3) };
    unsafe { ffi_SimpleStruct_funny_destroyer(obj4) };
}

#[ezffi::export]
#[derive(Default)]
pub struct DeallocationStruct {
    counter: Rc<()>,
}

#[ezffi::export]
impl DeallocationStruct {
    pub fn new() -> Self {
        DeallocationStruct {
            counter: Rc::new(()),
        }
    }

    pub fn manual_clone(&self) -> Self {
        DeallocationStruct {
            counter: self.counter.clone(),
        }
    }

    pub fn get_counter(&self) -> usize {
        Rc::strong_count(&self.counter)
    }

    pub fn consume(self) {}
}

#[test]
fn test_deallocation_struct() {
    let obj = unsafe { ffi_DeallocationStruct_new() };
    let clone1 = unsafe { ffi_DeallocationStruct_manual_clone(obj) };
    let clone2 = unsafe { ffi_DeallocationStruct_manual_clone(obj) };

    assert_eq!(unsafe { ffi_DeallocationStruct_get_counter(obj) }, 3);
    unsafe { ffi_DeallocationStruct_consume(clone1) };

    assert_eq!(unsafe { ffi_DeallocationStruct_get_counter(obj) }, 2);
    unsafe { ffi_DeallocationStruct_free(clone2) };

    assert_eq!(unsafe { ffi_DeallocationStruct_get_counter(obj) }, 1);
}
