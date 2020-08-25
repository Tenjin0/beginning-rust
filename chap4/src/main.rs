fn borrow_and_mut(s1: &mut String) {
    s1.push_str(", world")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn gives_ownership() -> String {
    return String::from("Hello");
}

fn makes_copy(x: i32) {
}

fn main() {
    let mut s1 = gives_ownership();
    let mut s2 = String::from("Hello");

    s2 = takes_and_gives_back(s2);
    borrow_and_mut(&mut s1);
    println!("{}", s1);
    println!("{} {}", s1, s2);
    let x = 5;

    makes_copy(x);

    println!("{}", x)
}
