#[derive(Debug, Eq, PartialEq)]
pub struct Student {
    first_name: String,
    last_name: String,
    age: u8,
}
impl Student {
    pub fn new(first_name: &str, last_name: &str, age: u8) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }
    pub fn age(&self) -> u8 {
        self.age
    }
}
