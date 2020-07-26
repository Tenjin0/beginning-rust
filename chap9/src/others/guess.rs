pub struct Guess {
    value: i32,
}
pub fn main() {
    println!("Hello from guess")
}
impl Guess {

    pub fn new(value: i32) -> Guess {

        if value < 1 || value > 100 {
            panic!("The secret number must be between 1 and 100.")
        }
        Guess {value: value}

    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

