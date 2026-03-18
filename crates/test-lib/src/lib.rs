#[ezffi::export]
pub struct Car {}

#[ezffi::export]
impl Car {
    pub fn new() -> Self {
        Self {}
    }
}

#[ezffi::export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
