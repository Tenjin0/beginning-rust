use std::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


pub fn longest3(x: &str) -> &str {
    x
}

pub fn longest4<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


pub fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display, 
{
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// pub fn longest5<'a>() -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
