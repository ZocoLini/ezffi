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

pub struct GenericInner {
    vec: Vec<String>,
    msg: String,
    number: u32,
}

#[ezffi::export]
impl Generic<GenericInner> {
    pub fn new2() -> Self {
        Generic {
            value: GenericInner {
                vec: vec!["Hello".to_string(), "bye".to_string()],
                msg: "Open the gate".to_string(),
                number: 56,
            },
        }
    }

    pub fn check(&self) {
        assert_eq!(self.value.vec[0], "Hello");
        assert_eq!(self.value.vec[1], "bye");
        assert_eq!(self.value.msg, "Open the gate");
        assert_eq!(self.value.number, 56);
    }
}
