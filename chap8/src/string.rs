pub fn main() {

    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);
    
    let s1 = String::from("Hello, ");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;
    println!("{}, {}", s2, s3);

}
