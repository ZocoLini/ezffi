#![allow(clippy::ptr_arg)]

#[ezffi::export]
pub fn count_elements(vec: &Vec<u32>) -> usize {
    vec.len()
}

#[ezffi::export]
pub fn add_element(vec: &mut Vec<u32>, value: u32) {
    vec.push(value);
}

#[ezffi::export]
pub fn get_element(vec: &mut Vec<u32>, index: usize) -> u32 {
    vec[index]
}

#[ezffi::export]
pub fn create_vec() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}
