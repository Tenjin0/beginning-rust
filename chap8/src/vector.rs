
#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    let _third = &v[2];
    
    match v.get(2) {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element")
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("value")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
