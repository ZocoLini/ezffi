use std::rc::Rc;

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

    pub fn receive_mut_ref(&mut self, other: &mut Car) {
        self.km += other.km;
    }

    pub fn receive_ref(&mut self, other: &Car) {
        self.km += other.km;
    }
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
