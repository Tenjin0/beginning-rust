fn main() {
    let f: bool = false;

    let c: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", f);
    println!("{}, {}, {}", c.0, c.1, c.2);
    println!("{:?}", c);

    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("{}", elem)
    }

    let mut b : i32 = 3;
   
    fn increase(value: i32) -> i32 {
        value + 1
    }

    fn increase_ref(value: &mut i32) -> i32 {
        *value = *value + 1;
        return *value;
    }

    // this is a comment
    println!("pass by value: {}; variable b: {}",  increase(b), b);

    let  b_ref: &mut i32 = &mut b;
    // cannot use increase in println cause borrowing
    println!("pass and mut by reference: {}; variable b: {}",  increase_ref(b_ref), b);

  
}
