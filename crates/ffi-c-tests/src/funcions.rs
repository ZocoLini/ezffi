#[ezffi::export]
pub struct SimpleStruct;

#[ezffi::export]
pub fn return_primitive() -> u32 {
    8
}

#[ezffi::export]
pub fn new_simple_struct() -> SimpleStruct {
    SimpleStruct
}

#[ezffi::export]
pub fn test(_o: SimpleStruct, _r: &SimpleStruct, _m: &mut SimpleStruct) {}

#[ezffi::export]
pub fn receive_simple_struct(_o: &SimpleStruct) {}

#[ezffi::export]
pub fn receive_simple_struct_mut(_o: &mut SimpleStruct) {}

#[ezffi::export]
pub fn receive_simple_struct_owned(_o: SimpleStruct) {}
