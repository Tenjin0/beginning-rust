
fn makes_copy(x: i32) {
    println!("print a copy of x: {}", x)
}

fn takes_ownership(mut s: String) -> String { // take ownership need to return in order to move out the ownership
    s.push_str(", world");
    return s
}

fn use_reference(s: &mut String) { 
    s.push_str(", Terry");
}

fn calulate_length(s: &String) -> usize { // use reference
    s.len()
}

fn main() {

    let mut x = 5;
    let y = x;

    x = 6;
    makes_copy(x);
    println!("x: {}, y: {}", x, y);
    let s = String::from("Hello");
    let mut s = takes_ownership(s);
    let mut s2 = s.clone();
    let _s3 = &s;


    use_reference(&mut s); // can only have ONE and unique &mut for s
    
    {
        use_reference(&mut s); //can be done cause we create a new scope
    }

    s2.push_str(", not Terry");

    println!("{}, len: {}", s, calulate_length(&s));
    println!("{}, len: {}", s2, calulate_length(&s2));
    // println!("{}, len: {}", s3, calulate_length(s3));  // Error, s is already borrowed
    let s4 = &s; 
    println!("{}, len: {}", s4, calulate_length(s4));  // can be done cause s2 is no longer used

}
