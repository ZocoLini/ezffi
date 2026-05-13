#[ezffi::export]
pub enum TestEnum {
    Empty,
    WithItems(String),
    WithNamedItems { msg: String, number: u64 },
}

#[ezffi::export]
impl TestEnum {
    pub fn new_empty() -> Self {
        TestEnum::Empty
    }

    pub fn new_with_items() -> Self {
        TestEnum::WithItems(String::from("Hello"))
    }

    pub fn new_with_named_items() -> Self {
        TestEnum::WithNamedItems {
            msg: String::from("world"),
            number: 67,
        }
    }

    pub fn check(self) {
        match self {
            TestEnum::Empty => {}
            TestEnum::WithItems(m) => assert_eq!(m, "Hello"),
            TestEnum::WithNamedItems { msg, number } => {
                assert_eq!(msg, "world");
                assert_eq!(number, 67)
            }
        }
    }
}

#[ezffi::export]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[ezffi::export]
impl Color {
    pub fn new_red() -> Self {
        Color::Red
    }

    pub fn next(self) -> Self {
        match self {
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Red,
        }
    }

    pub fn check_is_green(&self) -> bool {
        matches!(self, Color::Green)
    }
}
