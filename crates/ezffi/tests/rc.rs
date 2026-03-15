use std::rc::Rc;

#[ezffi::export]
pub fn create_rc() -> Rc<i32> {
    Rc::new(200)
}

#[ezffi::export]
pub fn receive_rc_ref(_rc: &Rc<i32>) {}

#[ezffi::export]
pub fn consume_rc(_rc: Rc<i32>) {}

#[ezffi::export]
pub fn return_rc_ref(rc: &Rc<i32>) -> &Rc<i32> {
    rc
}

#[ezffi::export]
pub fn return_rc(rc: Rc<i32>) -> Rc<i32> {
    rc
}

#[ezffi::export]
pub fn clone(rc: &Rc<i32>) -> Rc<i32> {
    Rc::clone(rc)
}

#[ezffi::export]
pub fn strong_count(rc: &Rc<i32>) -> usize {
    Rc::strong_count(rc)
}

#[test]
fn test_rc_lifecycle() {
    use ezffi::Rc;

    let rc: Rc<i32> = unsafe { ffi_create_rc() };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 1);

    unsafe { ffi_receive_rc_ref(rc) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 1);

    let clone1 = unsafe { ffi_clone(rc) };
    let clone2 = unsafe { ffi_clone(rc) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 3);

    unsafe { ffi_consume_rc(clone1) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 2);

    let clone2_ref = unsafe { ffi_return_rc_ref(clone2) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 2);

    let rc = unsafe { ffi_return_rc(rc) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 2);

    assert_eq!(unsafe { ffi_strong_count(rc) }, unsafe {
        ffi_strong_count(clone2)
    });
    assert_eq!(unsafe { ffi_strong_count(rc) }, unsafe {
        ffi_strong_count(clone2_ref)
    });

    unsafe { ffi_consume_rc(clone2_ref) };
    assert_eq!(unsafe { ffi_strong_count(rc) }, 1);
}
