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
