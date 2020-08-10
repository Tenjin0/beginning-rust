fn main() {
    
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {

        fn description(&self) -> String {
            return  "".to_string();
        }
        fn print_description(&self) {
            println!("Rectangle: {} x {}", self.width, self.height)
        }
        fn area(&self) -> u32 {
            &self.width * &self.height
        }

        fn is_square(&self) -> bool {
            self.width == self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect = Rectangle{width: 19, height: 5};

    rect.print_description();

    println!("rect is a square, {}", rect.area());
}
