#[ezffi::export]
pub struct Car {
    km: u64,
}

#[ezffi::export]
impl Car {
    pub fn new() -> Self {
        Self { km: 0 }
    }

    pub fn km(&self) -> u64 {
        self.km
    }

    pub fn drive(&mut self, km: u64) {
        self.km += km;
    }

    pub fn merge_cars(&mut self, other: Car) {
        self.km += other.km;
    }
}
